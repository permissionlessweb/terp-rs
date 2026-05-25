#!/usr/bin/env python3
"""
Dependency Graph & Push Order for the Abstract monorepo.

Parses all managed repo Cargo.tomls, builds a directed graph of inter-repo
git dependencies, computes the topological push order, and generates a
Graphviz DOT visualization.

Usage:
    dep-graph.py                    # show push order + write DOT file
    dep-graph.py --order            # push order only
    dep-graph.py --dot              # DOT output only (to stdout)
    dep-graph.py --svg              # generate SVG via dot (requires graphviz)
    dep-graph.py --update           # run cargo update in topological order
    dep-graph.py --update --dry-run # show what would be updated
    dep-graph.py --push --mode zk_git       # push to zk branches from matrix
    dep-graph.py --push --mode git          # push to git branches from matrix
    dep-graph.py --commit -m "msg"          # git add + commit all dirty repos
    dep-graph.py --commit --dry-run         # preview which repos would be committed
"""

import argparse
import os
import subprocess
import sys
from collections import defaultdict, deque
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    import tomllib
except ImportError:
    try:
        import tomli as tomllib
    except ImportError:
        print("ERROR: Python 3.11+ required (for tomllib), or install tomli")
        sys.exit(1)

from dep_common import REPO_ROOT, SKIP_DIRS, load_defaults, load_matrix, resolve_git_source


# ── Repo-to-URL mapping ────────────────────────────────────────────────────
# Map repo directory names to their GitHub org/repo identifiers (lowercase)
# so we can resolve git URLs back to local repo directories.

    # Repos where the Cargo.toml is nested inside a subdirectory
NESTED_CARGO_TOMLS = {
    "ics23": "rust",
    "nam-blst": "bindings/rust",
}


def discover_repos():
    """Discover all repos with Cargo.toml under REPO_ROOT.
    Returns dict of { repo_id: Path } where repo_id is the relative dir name.
    Handles repos with nested Cargo.toml (e.g. ics23/rust/)."""
    repos = {}
    for entry in sorted(REPO_ROOT.iterdir()):
        if not entry.is_dir() or entry.name in SKIP_DIRS or entry.name.startswith("."):
            continue
        if entry.name == "abstract":
            for sub in sorted(entry.iterdir()):
                if sub.is_dir() and (sub / "Cargo.toml").exists():
                    repo_id = f"abstract/{sub.name}"
                    repos[repo_id] = sub
        else:
            if (entry / "Cargo.toml").exists():
                repos[entry.name] = entry
            elif entry.name in NESTED_CARGO_TOMLS:
                nested = entry / NESTED_CARGO_TOMLS[entry.name]
                if (nested / "Cargo.toml").exists():
                    repos[entry.name] = nested
    return repos


def _is_valid_git_dir(path):
    """Check if a .git entry is a valid git repo (not an empty directory)."""
    git_path = path / ".git"
    if git_path.is_file():
        return True  # submodule pointer file
    if git_path.is_dir():
        return (git_path / "HEAD").exists()
    return False


def discover_all_git_repos(max_depth=5):
    """Discover ALL git repos under REPO_ROOT by scanning for .git directories.
    Returns dict of { repo_id: Path } where repo_id is relative path from REPO_ROOT.
    Used by commit/push to cover non-Cargo repos (Go, websites, etc.).
    Also finds nested git repos inside other git repos (e.g. submodules,
    independently-cloned repos like terp-rs/crates/zk/headstash)."""
    repos = {}

    def _scan(directory, depth):
        if depth > max_depth:
            return
        try:
            entries = sorted(directory.iterdir())
        except PermissionError:
            return
        for entry in entries:
            if not entry.is_dir():
                continue
            if entry.name in SKIP_DIRS or entry.name.startswith("."):
                continue
            if _is_valid_git_dir(entry):
                try:
                    rel = entry.relative_to(REPO_ROOT)
                except ValueError:
                    continue
                repo_id = str(rel)
                repos[repo_id] = entry
            # Always recurse — find nested git repos inside parent repos
            _scan(entry, depth + 1)

    _scan(REPO_ROOT, 1)
    return repos


def build_url_to_repo(repos):
    """Build a mapping from GitHub URL fragments to repo IDs.
    E.g. 'permissionlessweb/cosmwasm' -> 'cosmwasm'"""
    url_map = {}
    for repo_id, repo_path in repos.items():
        cargo = repo_path / "Cargo.toml"
        try:
            with open(cargo, "rb") as f:
                data = tomllib.load(f)
        except Exception:
            continue

        # Check workspace.package.repository or package.repository
        repo_url = (
            data.get("workspace", {}).get("package", {}).get("repository", "")
            or data.get("package", {}).get("repository", "")
        )
        if repo_url:
            # "https://github.com/permissionlessweb/cosmwasm" -> "permissionlessweb/cosmwasm"
            parts = repo_url.rstrip("/").split("/")
            if len(parts) >= 2:
                url_key = "/".join(parts[-2:]).lower()
                url_map[url_key] = repo_id

    # Manual overrides for repos without matching repository field
    # or with upstream-pointing repository fields
    manual = {
        "permissionlessweb/cosmwasm": "cosmwasm",
        "permissionlessweb/cw-storage-plus": "cw-storage-plus",
        "permissionlessweb/cw-minus": "cw-minus",
        "permissionlessweb/cw-plus": "cw-plus",
        "permissionlessweb/cw-plus-plus": "cw-plus-plus",
        "permissionlessweb/cw-asset": "cw-asset",
        "permissionlessweb/cosmos-rust": "cosmos-rust",
        "permissionlessweb/tendermint-rs": "tendermint-rs",
        "permissionlessweb/ibc-rs": "ibc-rs",
        "permissionlessweb/ibc-middleware": "ibc-middleware",
        "permissionlessweb/ibc-proto-rs": "ibc-proto-rs",
        "permissionlessweb/ics23": "ics23",
        "permissionlessweb/jmt": "jmt",
        "permissionlessweb/sparse-merkle-tree": "sparse-merkle-tree",
        "permissionlessweb/cnidarium": "cnidarium",
        "permissionlessweb/namada": "namada",
        "permissionlessweb/cw-orchestrator": "cw-orchestrator",
        "permissionlessweb/polytone": "polytone",
        "permissionlessweb/cw-multi-test-fork": "cw-multi-test-fork",
        "permissionlessweb/cw-multi-test": "cw-multi-test",
        "permissionlessweb/osmosis-rust": "osmosis-rust",
        "permissionlessweb/osmosis-test-tube": "osmosis-test-tube",
        "permissionlessweb/neutron-test-tube": "neutron-test-tube",
        "permissionlessweb/neutron-std": "neutron-std",
        "permissionlessweb/cw-ibc-demo": "cw-ibc-demo",
        "permissionlessweb/cw-nfts": "cw-nfts",
        "permissionlessweb/hermes": "hermes",
        "permissionlessweb/pbjson": "pbjson",
        "permissionlessweb/wynddex": "wynddex",
        "permissionlessweb/wynd-lsd": "wynd-lsd",
        "permissionlessweb/dao-contracts": "dao-contracts",
        "permissionlessweb/contracts": "xion-account",
        "permissionlessweb/cw-packages": "cw-packages",
        "permissionlessweb/token-bindings": "token-bindings",
        "permissionlessweb/basecoin-rs": "basecoin-rs",
        "permissionlessweb/tower-abci": "tower-abci",
        "permissionlessweb/blst": "nam-blst",
        "permissionlessweb/abstract": "abstract/framework",
        "permissionlessweb/abstract-cw-plus": "abstract-cw-plus",
        "permissionlessweb/cw-ica-controller": "cw-ica-controller",
        "permissionlessweb/ibc-types": "ibc-types",
        "permissionlessweb/smart-account-auth": "smart-account-auth",
        "permissionlessweb/penumbra": "penumbra",
        "permissionlessweb/halo2-axiom": "halo2-axiom",
        "anvdev/wavs-foundry-template": "new-cosmic-wavs",
        "megarocklabs/proxy-accounts": "proxy-accounts",
        "abstractsdk/cw-orchestrator": "cw-orchestrator",
    }
    for url_key, repo_id in manual.items():
        if repo_id in repos:
            url_map[url_key] = repo_id

    return url_map


def extract_git_deps_from_toml(cargo_path, url_map):
    """Extract set of repo_ids that a Cargo.toml depends on via git."""
    try:
        with open(cargo_path, "rb") as f:
            data = tomllib.load(f)
    except Exception:
        return set()

    deps = set()

    def check_table(table):
        if not isinstance(table, dict):
            return
        for name, spec in table.items():
            if isinstance(spec, dict) and spec.get("git"):
                git_url = spec["git"].rstrip("/")
                parts = git_url.split("/")
                if len(parts) >= 2:
                    url_key = "/".join(parts[-2:]).lower()
                    if url_key in url_map:
                        deps.add(url_map[url_key])

    # workspace.dependencies
    check_table(data.get("workspace", {}).get("dependencies", {}))
    # direct dependencies
    check_table(data.get("dependencies", {}))
    check_table(data.get("dev-dependencies", {}))
    check_table(data.get("build-dependencies", {}))
    # target-specific
    for _target, tdata in data.get("target", {}).items():
        check_table(tdata.get("dependencies", {}))
        check_table(tdata.get("dev-dependencies", {}))
    # patches (git patches also create dep edges)
    for _source, ptable in data.get("patch", {}).items():
        check_table(ptable)

    return deps


def build_dep_graph(repos, url_map):
    """Build directed graph: edges[repo_id] = set of repo_ids it depends on."""
    edges = defaultdict(set)

    for repo_id, repo_path in repos.items():
        cargo = repo_path / "Cargo.toml"
        if not cargo.exists():
            continue

        dep_repos = extract_git_deps_from_toml(cargo, url_map)

        # Also scan workspace member Cargo.tomls
        try:
            with open(cargo, "rb") as f:
                data = tomllib.load(f)
            members = data.get("workspace", {}).get("members", [])
            import glob as glob_mod
            for pattern in members:
                for match in glob_mod.glob(str(repo_path / pattern)):
                    member_cargo = Path(match) / "Cargo.toml"
                    if member_cargo.exists():
                        dep_repos |= extract_git_deps_from_toml(member_cargo, url_map)
        except Exception:
            pass

        # Remove self-references
        dep_repos.discard(repo_id)
        # Remove deps on repos we don't manage
        dep_repos = {d for d in dep_repos if d in repos}

        edges[repo_id] = dep_repos

    return edges


def topological_sort(repos, edges):
    """Kahn's algorithm. Returns ordered list or raises on cycle."""
    in_degree = defaultdict(int)
    for node in repos:
        in_degree.setdefault(node, 0)
    for node, deps in edges.items():
        for dep in deps:
            in_degree[node] += 1  # node depends on dep

    # Start with nodes that have no dependencies
    queue = deque(sorted(n for n in repos if in_degree[n] == 0))
    order = []

    while queue:
        node = queue.popleft()
        order.append(node)
        # Find all nodes that depend on this node, decrement their in-degree
        for other, deps in edges.items():
            if node in deps:
                in_degree[other] -= 1
                if in_degree[other] == 0:
                    queue.append(other)

    if len(order) != len(repos):
        missing = set(repos) - set(order)
        print(f"WARNING: cycle detected involving: {missing}", file=sys.stderr)
        # Append remaining in alphabetical order
        order.extend(sorted(missing))

    return order


# ── Graph layers for visual grouping ───────────────────────────────────────

LAYER_COLORS = {
    "leaf": "#4CAF50",       # Green - no deps
    "core-cw": "#2196F3",    # Blue - CosmWasm ecosystem
    "core-ibc": "#FF9800",   # Orange - IBC/Tendermint ecosystem
    "middleware": "#9C27B0",  # Purple - middleware/integration
    "consumer": "#F44336",   # Red - top-level consumers
}

def classify_layer(repo_id, edges):
    """Classify a repo into a visual layer."""
    deps = edges.get(repo_id, set())
    dependents = sum(1 for e in edges.values() if repo_id in e)

    if not deps:
        return "leaf"

    cw_repos = {"cosmwasm", "cw-storage-plus", "cw-minus", "cw-plus", "cw-plus-plus",
                "cw-asset", "cw-nfts", "cw-packages", "cw-multi-test", "cw-multi-test-fork",
                "clone-cw-multi-test"}
    ibc_repos = {"tendermint-rs", "cosmos-rust", "ibc-rs", "ibc-proto-rs", "ics23",
                 "ibc-middleware", "jmt", "cnidarium", "pbjson", "basecoin-rs", "tower-abci"}

    if repo_id in cw_repos:
        return "core-cw"
    if repo_id in ibc_repos:
        return "core-ibc"

    if repo_id in ("cw-orchestrator", "polytone", "hermes", "namada"):
        return "middleware"

    return "consumer"


def generate_dot(repos, edges, order):
    """Generate Graphviz DOT format string."""
    lines = [
        'digraph deps {',
        '  rankdir=BT;',  # bottom-to-top: leaves at bottom, consumers at top
        '  node [shape=box, style="rounded,filled", fontname="Helvetica", fontsize=11];',
        '  edge [color="#666666", arrowsize=0.7];',
        '  bgcolor="white";',
        '  pad=0.5;',
        '  nodesep=0.4;',
        '  ranksep=0.8;',
        '',
    ]

    # Group by layer
    layers = defaultdict(list)
    for repo_id in order:
        layer = classify_layer(repo_id, edges)
        layers[layer].append(repo_id)

    # Emit nodes grouped by layer
    for layer_name, color in LAYER_COLORS.items():
        if layer_name not in layers:
            continue
        lines.append(f'  // {layer_name}')
        for repo_id in layers[layer_name]:
            dep_count = len(edges.get(repo_id, set()))
            dependent_count = sum(1 for e in edges.values() if repo_id in e)
            label = repo_id.replace("abstract/", "a/")
            tooltip = f"{repo_id}: {dep_count} deps, {dependent_count} dependents"
            lines.append(
                f'  "{repo_id}" [label="{label}", fillcolor="{color}40", '
                f'color="{color}", tooltip="{tooltip}"];'
            )
        lines.append('')

    # Emit edges
    lines.append('  // edges')
    for repo_id in order:
        for dep in sorted(edges.get(repo_id, set())):
            lines.append(f'  "{repo_id}" -> "{dep}";')

    # Rank constraints: same rank for leaf nodes
    if layers.get("leaf"):
        leaf_list = " ".join(f'"{r}"' for r in layers["leaf"])
        lines.append(f'  {{ rank=same; {leaf_list} }}')

    lines.append('}')
    return "\n".join(lines)


def print_push_order(order, edges):
    """Print the topological push order with dependency info."""
    print("Push Order (topological — push & cargo update in this sequence):\n")
    for i, repo_id in enumerate(order, 1):
        deps = sorted(edges.get(repo_id, set()))
        dependents = sorted(r for r, e in edges.items() if repo_id in e)
        dep_str = ", ".join(deps) if deps else "(leaf)"
        dep_count = len(dependents)
        marker = "●" if not deps else "◆" if dep_count > 3 else "○"
        print(f"  {i:2d}. {marker} {repo_id:<30s} depends on: {dep_str}")
    print()
    print(f"Total: {len(order)} repos")
    print(f"Leaves (push first): {sum(1 for r in order if not edges.get(r))}")
    print(f"Heaviest consumers: {', '.join(r for r in order[-5:])}")


def run_cargo_update(order, repos, dry_run=False):
    """Run cargo update in topological order."""
    print("Running cargo update in topological order:\n")
    ok = fail = skip = 0
    for i, repo_id in enumerate(order, 1):
        repo_path = repos.get(repo_id)
        if not repo_path:
            continue
        lock = repo_path / "Cargo.lock"
        if not lock.exists():
            print(f"  {i:2d}. SKIP {repo_id} (no Cargo.lock)")
            skip += 1
            continue

        if dry_run:
            print(f"  {i:2d}. [DRY RUN] cargo update in {repo_id}")
            ok += 1
            continue

        print(f"  {i:2d}. Updating {repo_id} ... ", end="", flush=True)
        result = subprocess.run(
            ["cargo", "update"],
            cwd=repo_path,
            capture_output=True,
            text=True,
            timeout=120,
        )
        if result.returncode == 0:
            print("OK")
            ok += 1
        else:
            # Extract last meaningful line from stderr
            err_lines = [l for l in result.stderr.strip().splitlines() if l.strip()]
            err_msg = err_lines[-1] if err_lines else "unknown error"
            print(f"FAIL: {err_msg}")
            fail += 1

    print(f"\nDone: {ok} updated, {fail} failed, {skip} skipped")


def root_dir(repo_id, repo_path):
    """Get the actual git root directory for a repo (handles nested Cargo.toml repos)."""
    if repo_id in NESTED_CARGO_TOMLS:
        # e.g. ics23 -> repo_path is ics23/rust, git root is ics23
        parts = NESTED_CARGO_TOMLS[repo_id].split("/")
        root = repo_path
        for _ in parts:
            root = root.parent
        return root
    if repo_id.startswith("abstract/"):
        # abstract/framework -> git root is abstract/
        return repo_path.parent
    return repo_path


def _norm_url(u):
    """Normalize a git URL for comparison."""
    u = u.rstrip("/")
    if u.endswith(".git"):
        u = u[:-4]
    if u.startswith("git@github.com:"):
        u = "https://github.com/" + u[len("git@github.com:"):]
    return u.lower()


def _get_push_remote(git_root, target_url=None):
    """Determine the correct remote to push to.

    If target_url is provided, find the remote whose fetch URL matches it.
    This avoids pushing to upstream (403) instead of the fork.
    Falls back to fork > origin > first remote.
    """
    result = subprocess.run(
        ["git", "remote", "-v"], cwd=git_root, capture_output=True, text=True
    )
    lines = result.stdout.strip().splitlines()
    if not lines:
        return None

    # Parse remote name → URL (from fetch lines)
    remotes = {}
    for line in lines:
        parts = line.split()
        if len(parts) >= 2:
            remotes[parts[0]] = parts[1]

    if not remotes:
        return None

    # If we have a target URL from the matrix, find the matching remote
    if target_url:
        target_norm = _norm_url(target_url)
        for rname, rurl in remotes.items():
            if _norm_url(rurl) == target_norm:
                return rname

    # Fallback: prefer "pw" > "fork" > "origin" > first
    for preferred in ("pw", "fork", "origin"):
        if preferred in remotes:
            return preferred
    return next(iter(remotes))


def _get_current_branch(git_root):
    """Get current branch name."""
    result = subprocess.run(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        cwd=git_root, capture_output=True, text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def _has_unpushed_commits(git_root, remote, branch):
    """Check if there are local commits not on the remote."""
    # Fetch first to be accurate
    subprocess.run(
        ["git", "fetch", remote, branch],
        cwd=git_root, capture_output=True, text=True, timeout=30,
    )
    result = subprocess.run(
        ["git", "rev-list", f"{remote}/{branch}..HEAD", "--count"],
        cwd=git_root, capture_output=True, text=True,
    )
    if result.returncode != 0:
        # No upstream tracking — check if there are any commits at all
        return True
    count = int(result.stdout.strip())
    return count > 0


def _has_dirty_workdir(git_root):
    """Check for uncommitted changes."""
    result = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=git_root, capture_output=True, text=True,
    )
    return bool(result.stdout.strip())


def buildbranch_map(mode, url_map, repos=None):
    """Build repo_id → (branch, url) from the dependency matrix for a given mode.

    mode is 'git' or 'zk_git'. Uses resolve_git_source() for URL/branch resolution
    (with defaults and inheritance), deduplicates by repo URL, then maps via
    url_map to repo_id.

    Returns dict of repo_id → (branch, url). The url is the matrix fork URL,
    used by run_git_push to find the correct remote (avoiding 403 on upstream).

    If repos is provided, also does a local-path fallback for repos not covered
    by URL mapping (handles cases where multiple local directories clone the
    same remote, e.g. clone-cw-multi-test and cw-multi-test-fork).
    """
    crates = load_matrix()
    defaults = load_defaults()
    # url_fragment → (branch, full_url) (deduplicated; first wins per repo)
    url_entries = {}
    for _crate_name, info in crates.items():
        url, branch = resolve_git_source(info, mode, defaults)
        if not url or not branch:
            continue
        clean_url = url.rstrip("/")
        parts = clean_url.split("/")
        if len(parts) >= 2:
            url_key = "/".join(parts[-2:]).lower()
            if url_key not in url_entries:
                url_entries[url_key] = (branch, clean_url)

    # Map to repo_ids
    branch_map = {}
    for url_key, (branch, full_url) in url_entries.items():
        repo_id = url_map.get(url_key)
        if repo_id:
            branch_map[repo_id] = (branch, full_url)

    # Local-path fallback: for discovered repos not yet in branch_map,
    # check if any matrix crate's local path lives inside the repo directory.
    if repos:
        for repo_id, repo_path in repos.items():
            if repo_id in branch_map:
                continue
            # Check parent for abstract/* sub-workspaces
            parent_id = repo_id.split("/")[0] if "/" in repo_id else None
            if parent_id and parent_id in branch_map:
                continue
            repo_resolved = str(repo_path.resolve())
            for _crate_name, info in crates.items():
                local = info.get("local", "")
                if not local or local == "N/A":
                    continue
                crate_path = str((REPO_ROOT / local.lstrip("./")).resolve())
                if crate_path.startswith(repo_resolved + "/") or crate_path == repo_resolved:
                    url, branch = resolve_git_source(info, mode, defaults)
                    if url and branch:
                        branch_map[repo_id] = (branch, url.rstrip("/"))
                        break

    return branch_map


def run_git_commit(order, repos, message, dry_run=False):
    """Git add + commit all dirty repos in topological order."""
    print(f"Git commit in topological order (message: {message!r}):\n")
    committed = skip_clean = skip_nogit = skip_err = 0
    seen_roots = set()

    for i, repo_id in enumerate(order, 1):
        repo_path = repos.get(repo_id)
        if not repo_path:
            continue

        git_root = root_dir(repo_id, repo_path)

        if str(git_root) in seen_roots:
            continue
        seen_roots.add(str(git_root))

        if not (git_root / ".git").exists():
            print(f"  {i:2d}. SKIP {repo_id} (no .git)")
            skip_nogit += 1
            continue

        if not _has_dirty_workdir(git_root):
            print(f"  {i:2d}. CLEAN {repo_id}")
            skip_clean += 1
            continue

        if dry_run:
            print(f"  {i:2d}. [DRY RUN] would commit {repo_id}")
            committed += 1
            continue

        # git add -A && git commit
        subprocess.run(["git", "add", "-A"], cwd=git_root, capture_output=True)
        result = subprocess.run(
            ["git", "commit", "-m", message],
            cwd=git_root, capture_output=True, text=True, timeout=30,
        )
        if result.returncode == 0:
            short = result.stdout.strip().splitlines()[0] if result.stdout.strip() else "ok"
            print(f"  {i:2d}. COMMIT {repo_id}: {short}")
            committed += 1
        else:
            err_lines = result.stderr.strip().splitlines()
            err_msg = err_lines[-1] if err_lines else "unknown error"
            print(f"  {i:2d}. FAIL {repo_id}: {err_msg}")
            skip_err += 1

    print(f"\nDone: {committed} committed, {skip_clean} clean, "
          f"{skip_nogit} no-git, {skip_err} errors")


def _is_fast_forward(git_root, remote, branch):
    """Check if pushing HEAD to remote/branch would be a fast-forward.

    Returns True if fast-forward (safe), False if not (needs rebase/pull).
    Returns True if remote branch doesn't exist (new branch push).
    """
    # Check if remote branch ref exists locally (after fetch)
    result = subprocess.run(
        ["git", "rev-parse", "--verify", f"refs/remotes/{remote}/{branch}"],
        cwd=git_root, capture_output=True, text=True,
    )
    if result.returncode != 0:
        return True  # Remote branch doesn't exist — new branch, always OK

    # Check if remote/branch is an ancestor of HEAD (i.e. fast-forward)
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", f"{remote}/{branch}", "HEAD"],
        cwd=git_root, capture_output=True, text=True,
    )
    return result.returncode == 0


def _has_upstream(git_root, remote, branch):
    """Check if remote/branch exists (without fetching)."""
    result = subprocess.run(
        ["git", "rev-parse", "--verify", f"refs/remotes/{remote}/{branch}"],
        cwd=git_root, capture_output=True, text=True,
    )
    return result.returncode == 0


def run_git_push(order, repos, dry_run=False, branch_map=None):
    """Git push all repos in topological order.

    branch_map: optional dict of repo_id → (target_branch, matrix_url) or
                repo_id → target_branch (legacy).
    When set, pushes HEAD:<target_branch> so local work lands on the right
    remote branch regardless of which branch is currently checked out.
    The matrix_url is used to find the correct remote (avoids 403 on upstream).
    When None, pushes the current branch as-is.
    """
    mode_label = ""
    if branch_map:
        mode_label = " (targeting matrix branches)"
    print(f"Git push in topological order{mode_label}:\n")
    pushed = skip_clean = skip_nogit = skip_err = skip_nomap = 0
    dirty = []
    new_branches = []
    not_ff = []
    seen_roots = set()

    for i, repo_id in enumerate(order, 1):
        repo_path = repos.get(repo_id)
        if not repo_path:
            continue

        git_root = root_dir(repo_id, repo_path)

        # Skip abstract/* sub-workspaces (they share a single git root)
        if str(git_root) in seen_roots:
            continue
        seen_roots.add(str(git_root))

        # Check if it's a git repo
        if not (git_root / ".git").exists():
            print(f"  {i:2d}. SKIP  {repo_id} (no .git)")
            skip_nogit += 1
            continue

        local_branch = _get_current_branch(git_root)

        if not local_branch or local_branch == "HEAD":
            print(f"  {i:2d}. SKIP  {repo_id} (detached HEAD)")
            skip_err += 1
            continue

        # Determine target branch and matrix URL
        target_url = None
        if branch_map:
            entry = branch_map.get(repo_id)
            if not entry:
                parent_id = repo_id.split("/")[0] if "/" in repo_id else None
                entry = branch_map.get(parent_id) if parent_id else None
            if not entry:
                print(f"  {i:2d}. SKIP  {repo_id} @ {local_branch} (not in matrix for this mode)")
                skip_nomap += 1
                continue
            # Support both (branch, url) tuple and plain branch string
            if isinstance(entry, tuple):
                target_branch, target_url = entry
            else:
                target_branch = entry
            refspec = f"HEAD:{target_branch}"
        else:
            target_branch = local_branch

        # Find the right remote (use matrix URL to avoid pushing to upstream)
        remote = _get_push_remote(git_root, target_url=target_url)
        if not remote:
            print(f"  {i:2d}. SKIP  {repo_id} (no remote)")
            skip_err += 1
            continue

        if branch_map:
            tag = f"{repo_id} @ {local_branch} → {remote}/{target_branch}"
        else:
            refspec = target_branch
            tag = f"{repo_id} @ {local_branch} → {remote}"

        # Check for dirty workdir
        if _has_dirty_workdir(git_root):
            dirty.append(f"{repo_id} ({local_branch})")
            print(f"  {i:2d}. DIRTY {tag} (uncommitted changes)")
            continue

        # Check if target branch exists on remote
        is_new = not _has_upstream(git_root, remote, target_branch)

        if not is_new:
            # Fetch to get latest remote state
            subprocess.run(
                ["git", "fetch", remote, target_branch],
                cwd=git_root, capture_output=True, text=True, timeout=30,
            )

            # Check for unpushed commits
            if not _has_unpushed_commits(git_root, remote, target_branch):
                print(f"  {i:2d}. CLEAN {tag} (up to date)")
                skip_clean += 1
                continue

            # Check fast-forward safety
            if not _is_fast_forward(git_root, remote, target_branch):
                not_ff.append(f"{repo_id} ({local_branch} → {target_branch})")
                print(f"  {i:2d}. NOT-FF {tag} (needs rebase/pull)")
                skip_err += 1
                continue

        if dry_run:
            label = "[DRY RUN] would push (new branch)" if is_new else "[DRY RUN] would push"
            print(f"  {i:2d}. {label} {tag}")
            pushed += 1
            if is_new:
                new_branches.append(f"{repo_id} ({target_branch})")
            continue

        action = "PUSH*" if is_new else "PUSH "
        print(f"  {i:2d}. {action} {tag} ... ", end="", flush=True)
        result = subprocess.run(
            ["git", "push", "-u", remote, refspec],
            cwd=git_root, capture_output=True, text=True, timeout=60,
        )
        if result.returncode == 0:
            print("OK" + (" (new branch)" if is_new else ""))
            pushed += 1
            if is_new:
                new_branches.append(f"{repo_id} ({target_branch})")
        else:
            err_lines = [l for l in result.stderr.strip().splitlines() if l.strip()]
            err_msg = err_lines[-1] if err_lines else "unknown error"
            print(f"FAIL: {err_msg}")
            skip_err += 1

    summary = f"\nDone: {pushed} pushed, {skip_clean} up-to-date, {skip_nogit} no-git, {skip_err} errors"
    if skip_nomap:
        summary += f", {skip_nomap} not in matrix"
    print(summary)
    if new_branches:
        print(f"New branches pushed: {', '.join(new_branches)}")
    if not_ff:
        print(f"Non-fast-forward (rebase first): {', '.join(not_ff)}")
    if dirty:
        print(f"Dirty repos (commit first): {', '.join(dirty)}")


def main():
    parser = argparse.ArgumentParser(description="Dependency graph & push order")
    parser.add_argument("--order", action="store_true", help="Show push order only")
    parser.add_argument("--dot", action="store_true", help="Output DOT to stdout")
    parser.add_argument("--svg", action="store_true", help="Generate SVG file")
    parser.add_argument("--update", action="store_true", help="Run cargo update in order")
    parser.add_argument("--push", action="store_true", help="Git push all repos in order")
    parser.add_argument("--mode", choices=["git", "zk_git"],
                        help="Push to branch names from the dependency matrix (git or zk_git)")
    parser.add_argument("--commit", action="store_true", help="Git commit all dirty repos")
    parser.add_argument("--message", "-m", type=str, default="checkpoint", help="Commit message")
    parser.add_argument("--dry-run", action="store_true", help="Preview without executing")
    args = parser.parse_args()

    repos = discover_repos()
    url_map = build_url_to_repo(repos)
    edges = build_dep_graph(repos, url_map)
    order = topological_sort(repos, edges)

    if args.commit:
        # Discover ALL git repos (not just Cargo) so we commit terp-rs, terp-core, etc.
        all_git = discover_all_git_repos()
        combined_repos = dict(all_git)
        combined_repos.update(repos)  # Cargo repos get priority for path accuracy
        # Order: topo-sorted Cargo repos first, then remaining git repos alphabetically
        in_order = set(order)
        extra = sorted(r for r in all_git if r not in in_order)
        combined_order = list(order) + extra
        run_git_commit(combined_order, combined_repos, args.message, args.dry_run)
        return

    if args.push:
        branch_map = None
        if args.mode:
            branch_map = buildbranch_map(args.mode, url_map, repos=repos)
            if not branch_map:
                print(f"ERROR: no branches found in matrix for mode '{args.mode}'")
                sys.exit(1)
        # For plain push (no --mode), discover ALL git repos like commit does
        if not args.mode:
            all_git = discover_all_git_repos()
            push_repos = dict(all_git)
            push_repos.update(repos)
            in_order = set(order)
            extra = sorted(r for r in all_git if r not in in_order)
            push_order = list(order) + extra
        else:
            push_repos = repos
            push_order = order
        run_git_push(push_order, push_repos, args.dry_run, branch_map=branch_map)
        return

    if args.update:
        run_cargo_update(order, repos, args.dry_run)
        return

    if args.dot:
        print(generate_dot(repos, edges, order))
        return

    if args.svg:
        dot_content = generate_dot(repos, edges, order)
        dot_path = REPO_ROOT / "_devops" / "dep-graph.dot"
        svg_path = REPO_ROOT / "_devops" / "dep-graph.svg"
        dot_path.write_text(dot_content)
        result = subprocess.run(
            ["dot", "-Tsvg", str(dot_path), "-o", str(svg_path)],
            capture_output=True, text=True,
        )
        if result.returncode == 0:
            print(f"Wrote {svg_path}")
        else:
            print(f"dot failed: {result.stderr}")
            sys.exit(1)
        return

    # Default: show order + write DOT
    print_push_order(order, edges)

    dot_content = generate_dot(repos, edges, order)
    dot_path = REPO_ROOT / "_devops" / "dep-graph.dot"
    dot_path.parent.mkdir(parents=True, exist_ok=True)
    dot_path.write_text(dot_content)
    print(f"\nWrote {dot_path}")

    # Try to generate SVG if dot is available
    svg_path = REPO_ROOT / "_devops" / "dep-graph.svg"
    result = subprocess.run(
        ["dot", "-Tsvg", str(dot_path), "-o", str(svg_path)],
        capture_output=True, text=True,
    )
    if result.returncode == 0:
        print(f"Wrote {svg_path}")


if __name__ == "__main__":
    main()
