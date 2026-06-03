"""
Shared library for dependency tooling in the Abstract monorepo.

Provides: constants, matrix loading, project discovery, dep extraction,
TOML writing helpers, and data classes used by dep-scrape, dep-overview,
and dep-switch.
"""

import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

try:
    import tomllib
except ImportError:
    try:
        import tomli as tomllib
    except ImportError:
        print("ERROR: Python 3.11+ required (for tomllib), or install tomli")
        sys.exit(1)


# ── Constants ────────────────────────────────────────────────────────────────

REPO_ROOT = Path(__file__).resolve().parent.parent
MATRIX_PATH = REPO_ROOT / "_devops" / "dependency-matrix.toml"
MIGRATION_PATH = REPO_ROOT / "_devops" / "migration-progress.yaml"
DEVOPS_DIR = REPO_ROOT / "_devops"
MODE_STATE_PATH = DEVOPS_DIR / ".dep-mode"

VALID_MODES = ("stable", "local", "git", "zk_local", "zk_git", "dev", "zk_dev")

# Composite mode decomposition: (ws_dep_mode, patch_mode, local_key)
DEV_MODE_MAP = {
    "dev":    ("git",    "local",    "local"),
    "zk_dev": ("zk_git", "zk_local", "zk_local"),
}

SKIP_DIRS = {".git", "target", "_devops", "_scripts", ".claude", "node_modules"}


def save_active_mode(mode: str):
    """Persist the current dep mode to _devops/.dep-mode."""
    MODE_STATE_PATH.write_text(mode.strip() + "\n")


def load_active_mode() -> Optional[str]:
    """Read the saved dep mode from _devops/.dep-mode, or None if not set."""
    if MODE_STATE_PATH.exists():
        mode = MODE_STATE_PATH.read_text().strip()
        if mode in VALID_MODES:
            return mode
    return None

# Hardcoded workspace roots (fallback / filter)
KNOWN_WORKSPACE_ROOTS = [
    "abstract/framework/Cargo.toml",
    "abstract/modules/Cargo.toml",
    "abstract/integrations/Cargo.toml",
    "abstract/interchain/Cargo.toml",
    "cw-orchestrator/Cargo.toml",
]


# ── Data Classes ─────────────────────────────────────────────────────────────

@dataclass
class ProjectRoot:
    name: str
    rel_path: str
    cargo_toml: Path
    is_workspace: bool
    member_tomls: list  # list[Path]
    has_readme: bool

    @property
    def dir(self) -> Path:
        return self.cargo_toml.parent


@dataclass
class DepSpec:
    name: str
    package: Optional[str]  # renamed package name if any
    source: str  # "path" | "git" | "registry"
    version: Optional[str]
    path: Optional[str]
    git_url: Optional[str]
    git_branch: Optional[str]
    features: list  # list[str]
    default_features: bool
    dep_kind: str  # "normal" | "dev" | "build"
    workspace_inherited: bool
    declaring_file: str

    @property
    def pkg_name(self) -> str:
        return self.package or self.name


@dataclass
class PatchEntry:
    source_url: str  # e.g. "crates-io" or a git URL
    name: str
    package: Optional[str]
    path: Optional[str]
    git_url: Optional[str]
    git_branch: Optional[str]


@dataclass
class ProjectScanResult:
    project: ProjectRoot
    workspace_deps: dict  # name -> parsed dep info
    member_deps: list  # list[DepSpec]
    patches: list  # list[PatchEntry]
    path_dep_count: int = 0
    git_dep_count: int = 0
    registry_dep_count: int = 0
    patch_count: int = 0
    loc: int = 0
    diagnostics: list = field(default_factory=list)


# ── Matrix Functions ─────────────────────────────────────────────────────────

def load_matrix() -> dict:
    """Load the dependency matrix TOML. Returns the [crates] table."""
    with open(MATRIX_PATH, "rb") as f:
        return tomllib.load(f).get("crates", {})


def load_defaults() -> dict:
    """Load [defaults] from the matrix. Returns {} if absent."""
    with open(MATRIX_PATH, "rb") as f:
        return tomllib.load(f).get("defaults", {})


def resolve_git_source(info: dict, mode: str, defaults: dict) -> tuple:
    """Resolve (url, branch) for a crate in git or zk_git mode.

    Resolution order for zk_git:
      url:    zk_git.url  → git.url
      branch: zk_git.branch → defaults.zk_git_branch → git.branch → defaults.git_branch

    Resolution order for git:
      url:    git.url (required)
      branch: git.branch → defaults.git_branch → "main"

    Returns (url, branch) or (None, None) if unresolvable.
    """
    gi = info.get("git", {})
    if not isinstance(gi, dict):
        gi = {}
    zg = info.get("zk_git", {})
    if not isinstance(zg, dict):
        zg = {}

    if mode == "git":
        url = gi.get("url")
        if not url or url == "N/A":
            return (None, None)
        branch = gi.get("branch") or defaults.get("git_branch", "main")
        return (url, branch)

    if mode == "zk_git":
        url = zg.get("url") or gi.get("url")
        if not url or url == "N/A":
            return (None, None)
        branch = (zg.get("branch")
                  or defaults.get("zk_git_branch")
                  or gi.get("branch")
                  or defaults.get("git_branch", "main"))
        return (url, branch)

    return (None, None)


def is_repo_only(info: dict) -> bool:
    """Check if a matrix entry is repo-level metadata (not a publishable crate).
    These entries exist for dep-graph push coverage but must NEVER be written
    to Cargo.toml files as dependency or patch entries."""
    return bool(info.get("repo_only"))


def pkg_name(key: str, info: dict) -> str:
    """Published crates.io package name for a matrix crate."""
    return info.get("package", key)


def matrix_pkg_names(crates: dict) -> set:
    """Set of all published package names + dep_aliases in the matrix."""
    names = set()
    for k, v in crates.items():
        if is_repo_only(v):
            continue
        names.add(pkg_name(k, v))
        for alias in v.get("dep_aliases", []):
            names.add(alias)
    return names


def pkg_to_matrix(crates: dict) -> dict:
    """Map published package name (and dep_aliases) -> (matrix_key, matrix_info)."""
    result = {}
    for k, v in crates.items():
        if is_repo_only(v):
            continue
        result[pkg_name(k, v)] = (k, v)
        for alias in v.get("dep_aliases", []):
            result[alias] = (k, v)
    return result


def matrix_key_to_pkg(crates: dict) -> dict:
    """Map matrix key -> published package name."""
    return {k: pkg_name(k, v) for k, v in crates.items()}


# ── Migration YAML Loader ───────────────────────────────────────────────────

def load_migration_progress() -> list:
    """Load migration-progress.yaml. Returns list of workspace dicts.
    Uses a simple line-based parser (no pyyaml dependency)."""
    if not MIGRATION_PATH.exists():
        return []

    try:
        import yaml
        with open(MIGRATION_PATH) as f:
            data = yaml.safe_load(f)
        return data.get("workspaces", []) if data else []
    except ImportError:
        pass

    # Simple fallback parser
    workspaces = []
    current = None
    in_blockers = False

    for line in MIGRATION_PATH.read_text().splitlines():
        stripped = line.strip()

        if stripped.startswith("#") or not stripped:
            in_blockers = False
            continue

        if stripped == "workspaces:":
            continue

        if stripped.startswith("- name:"):
            if current:
                workspaces.append(current)
            current = {"name": stripped.split(":", 1)[1].strip()}
            in_blockers = False
            continue

        if current and ":" in stripped and not stripped.startswith("-"):
            key, val = stripped.split(":", 1)
            key = key.strip()
            val = val.strip().strip('"').strip("'")
            if key == "blockers":
                in_blockers = True
                current["blockers"] = []
                # Check for inline empty list
                if val in ("[]", ""):
                    continue
            elif key in ("path", "cw_version", "cargo_check", "cargo_test", "notes"):
                current[key] = val
                in_blockers = False
            continue

        if in_blockers and current and stripped.startswith("- "):
            blocker = stripped[2:].strip().strip('"').strip("'")
            current.setdefault("blockers", []).append(blocker)
            continue

    if current:
        workspaces.append(current)

    return workspaces


# ── Project Discovery ────────────────────────────────────────────────────────

def discover_projects() -> list:
    """Discover all project roots under REPO_ROOT.
    Walks top-level dirs; when a dir has no root Cargo.toml, checks one
    level of subdirs (handles abstract/, ics23/rust, etc.).
    Also discovers excluded workspace members, standalone sub-projects
    (tools, examples, nested workspaces), and deeply nested projects."""
    import glob as glob_mod

    projects = []
    seen_paths = set()  # dedup by cargo_toml path

    def _add_project(proj):
        if proj and proj.cargo_toml not in seen_paths:
            seen_paths.add(proj.cargo_toml)
            projects.append(proj)

    # Top-level dirs
    for entry in sorted(REPO_ROOT.iterdir()):
        if not entry.is_dir():
            continue
        if entry.name in SKIP_DIRS or entry.name.startswith("."):
            continue

        cargo_toml = entry / "Cargo.toml"
        if cargo_toml.exists():
            proj = _build_project(cargo_toml)
            _add_project(proj)
            # Check excluded members — they have independent dep resolution
            if proj and proj.is_workspace:
                try:
                    with open(cargo_toml, "rb") as f:
                        data = tomllib.load(f)
                    for exc in data.get("workspace", {}).get("exclude", []):
                        for match in glob_mod.glob(str(entry / exc)):
                            match_path = Path(match)
                            exc_cargo = match_path / "Cargo.toml"
                            if exc_cargo.exists():
                                _add_project(_build_project(exc_cargo))
                            elif match_path.is_dir():
                                # Excluded dir has sub-projects (e.g. testing/contracts/*)
                                _discover_subdirs(match_path, projects, seen_paths, depth=2)
                except Exception:
                    pass
            # Also discover sub-projects (tools/, examples/, dependencies/)
            _discover_subdirs(entry, projects, seen_paths, depth=4)
            continue

        # No root Cargo.toml — recursively check subdirs up to 4 levels deep
        # (handles abstract/, ics23/rust, <project>/,
        #  terp-rs/crates/<project>/, etc.)
        _discover_subdirs(entry, projects, seen_paths, depth=4)

    return projects


def _discover_subdirs(directory: Path, projects: list, seen_paths: set, depth: int):
    """Recursively discover Cargo.toml projects in subdirectories."""
    if depth <= 0:
        return
    try:
        children = sorted(directory.iterdir())
    except PermissionError:
        return
    for sub in children:
        if not sub.is_dir() or sub.name in SKIP_DIRS or sub.name.startswith("."):
            continue
        sub_cargo = sub / "Cargo.toml"
        if sub_cargo.exists():
            proj = _build_project(sub_cargo)
            if proj and proj.cargo_toml not in seen_paths:
                seen_paths.add(proj.cargo_toml)
                projects.append(proj)
            # Also recurse to find sub-projects (codegen/, tools/, examples/)
            _discover_subdirs(sub, projects, seen_paths, depth - 1)
        else:
            # No Cargo.toml here — go deeper
            _discover_subdirs(sub, projects, seen_paths, depth - 1)


def _build_project(cargo_toml: Path) -> Optional[ProjectRoot]:
    """Build a ProjectRoot from a Cargo.toml path."""
    try:
        with open(cargo_toml, "rb") as f:
            data = tomllib.load(f)
    except Exception:
        return None

    proj_dir = cargo_toml.parent
    rel = os.path.relpath(proj_dir, REPO_ROOT)
    name = rel.replace("/", "-") if "/" in rel else proj_dir.name

    ws = data.get("workspace", {})
    is_workspace = "members" in ws
    member_tomls = []

    if is_workspace:
        members = ws.get("members", [])
        excludes = ws.get("exclude", [])
        member_tomls = resolve_workspace_members(proj_dir, members, excludes)

    has_readme = (proj_dir / "README.md").exists()

    return ProjectRoot(
        name=name,
        rel_path=rel,
        cargo_toml=cargo_toml,
        is_workspace=is_workspace,
        member_tomls=member_tomls,
        has_readme=has_readme,
    )


def resolve_workspace_members(project_dir: Path, members: list, excludes: list) -> list:
    """Resolve workspace member glob patterns to Cargo.toml paths."""
    import glob as glob_mod

    exclude_set = set()
    for exc in excludes:
        for match in glob_mod.glob(str(project_dir / exc)):
            exclude_set.add(Path(match).resolve())

    result = []
    for pattern in members:
        # Expand globs
        expanded = glob_mod.glob(str(project_dir / pattern))
        for match in sorted(expanded):
            match_path = Path(match).resolve()
            if match_path in exclude_set:
                continue
            cargo = match_path / "Cargo.toml" if match_path.is_dir() else match_path
            if cargo.exists() and cargo.name == "Cargo.toml":
                result.append(cargo)

    return result


# ── Dep Extraction ───────────────────────────────────────────────────────────

def extract_workspace_deps(data: dict) -> dict:
    """Extract [workspace.dependencies] table from parsed TOML data."""
    return data.get("workspace", {}).get("dependencies", {})


def extract_deps_from_table(table: dict, kind: str, ws_deps: dict,
                            declaring_file: str) -> list:
    """Extract deps from a [dependencies], [dev-dependencies], or [build-dependencies] table.

    Args:
        table: The dependency table dict
        kind: "normal" | "dev" | "build"
        ws_deps: The workspace.dependencies dict for resolving workspace=true
        declaring_file: Path of the Cargo.toml declaring these deps
    """
    deps = []
    for name, spec in table.items():
        dep = _parse_dep_spec(name, spec, kind, ws_deps, declaring_file)
        if dep:
            deps.append(dep)
    return deps


def _parse_dep_spec(name: str, spec, kind: str, ws_deps: dict,
                    declaring_file: str) -> Optional[DepSpec]:
    """Parse a single dependency specification."""
    if isinstance(spec, str):
        # Simple version string: name = "1.0"
        return DepSpec(
            name=name, package=None, source="registry",
            version=spec, path=None, git_url=None, git_branch=None,
            features=[], default_features=True, dep_kind=kind,
            workspace_inherited=False, declaring_file=declaring_file,
        )

    if isinstance(spec, dict):
        # Check for workspace = true
        if spec.get("workspace", False):
            # Resolve from workspace deps
            ws_spec = ws_deps.get(name, {})
            resolved = _parse_dep_spec(name, ws_spec, kind, {}, declaring_file) if ws_spec else None
            if resolved:
                resolved.workspace_inherited = True
                # Merge features from the member-level spec
                extra_features = spec.get("features", [])
                if extra_features:
                    resolved.features = list(set(resolved.features + extra_features))
                if "default-features" in spec:
                    resolved.default_features = spec["default-features"]
                return resolved
            # Workspace ref but not found in workspace deps
            return DepSpec(
                name=name, package=None, source="registry",
                version=None, path=None, git_url=None, git_branch=None,
                features=spec.get("features", []),
                default_features=spec.get("default-features", True),
                dep_kind=kind, workspace_inherited=True,
                declaring_file=declaring_file,
            )

        package = spec.get("package")
        version = spec.get("version")
        path = spec.get("path")
        git_url = spec.get("git")
        git_branch = spec.get("branch")
        features = spec.get("features", [])
        default_features = spec.get("default-features", True)

        if path:
            source = "path"
        elif git_url:
            source = "git"
        else:
            source = "registry"

        return DepSpec(
            name=name, package=package, source=source,
            version=version, path=path, git_url=git_url, git_branch=git_branch,
            features=features, default_features=default_features,
            dep_kind=kind, workspace_inherited=False,
            declaring_file=declaring_file,
        )

    return None


def extract_all_deps(data: dict, ws_deps: dict, declaring_file: str) -> list:
    """Extract all deps from a Cargo.toml's parsed data."""
    deps = []
    for section, kind in [
        ("dependencies", "normal"),
        ("dev-dependencies", "dev"),
        ("build-dependencies", "build"),
    ]:
        table = data.get(section, {})
        deps.extend(extract_deps_from_table(table, kind, ws_deps, declaring_file))

    # Also handle target-specific deps
    for target_key, target_data in data.get("target", {}).items():
        for section, kind in [
            ("dependencies", "normal"),
            ("dev-dependencies", "dev"),
            ("build-dependencies", "build"),
        ]:
            table = target_data.get(section, {})
            deps.extend(extract_deps_from_table(table, kind, ws_deps, declaring_file))

    return deps


def extract_patches(data: dict) -> list:
    """Extract all [patch.*] entries from parsed TOML data."""
    patches = []
    patch_tables = data.get("patch", {})
    for source_url, table in patch_tables.items():
        for name, spec in table.items():
            if isinstance(spec, dict):
                patches.append(PatchEntry(
                    source_url=source_url,
                    name=name,
                    package=spec.get("package"),
                    path=spec.get("path"),
                    git_url=spec.get("git"),
                    git_branch=spec.get("branch"),
                ))
            else:
                patches.append(PatchEntry(
                    source_url=source_url,
                    name=name,
                    package=None,
                    path=None,
                    git_url=None,
                    git_branch=None,
                ))
    return patches


# ── TOML Writing Helpers ─────────────────────────────────────────────────────

def find_section(lines: list, header: str) -> tuple:
    """Find start/end line indices for a TOML section.
    Returns (start_index, end_index) where end is the next section or EOF.
    Sub-sections (e.g. [workspace.dependencies.foo] under [workspace.dependencies])
    are included in the range."""
    # Derive prefix for sub-section matching: "[workspace.dependencies]" -> "[workspace.dependencies."
    prefix = header.rstrip("]") + "."
    start = end = None
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == header:
            start = i
        elif start is not None and end is None and stripped.startswith("["):
            # Skip sub-sections like [workspace.dependencies.foo]
            if stripped.startswith(prefix):
                continue
            end = i
            break
    if start is not None and end is None:
        end = len(lines)
    return start, end


def read_cargo_toml(path: Path) -> tuple:
    """Read a Cargo.toml: returns (parsed_dict, raw_text)."""
    raw = path.read_text()
    with open(path, "rb") as f:
        data = tomllib.load(f)
    return data, raw


def rewrite_section(lines: list, start: int, end: int, new_lines: list) -> list:
    """Replace lines[start:end] with new_lines, return updated list."""
    return lines[:start] + new_lines + lines[end:]


# ── Path Utilities ───────────────────────────────────────────────────────────

def rel_path(ws_toml: str, local: str) -> str:
    """Relative path from workspace Cargo.toml dir to a dependency."""
    ws_dir = (REPO_ROOT / ws_toml).parent
    dep = (REPO_ROOT / local).resolve()
    return os.path.relpath(dep, ws_dir).replace("\\", "/")


def build_entry(key: str, info: dict, mode: str, ws: str, defaults: dict = None) -> Optional[str]:
    """Build a patch entry value string for a crate in a given mode.
    Returns value string or None if mode source is unavailable."""
    if defaults is None:
        defaults = {}

    if mode == "stable":
        return None

    if mode == "local":
        local = info.get("local", "N/A")
        if local == "N/A":
            return None
        return f'{{ path = "{rel_path(ws, local)}" }}'

    if mode in ("git", "zk_git"):
        url, branch = resolve_git_source(info, mode, defaults)
        if not url:
            return None
        return f'{{ git = "{url}", branch = "{branch}" }}'

    if mode == "zk_local":
        zl = info.get("zk_local")
        if zl and zl != "N/A":
            return f'{{ path = "{rel_path(ws, zl)}" }}'
        local = info.get("local", "N/A")
        if local == "N/A":
            return None
        return f'{{ path = "{rel_path(ws, local)}" }}'

    return None


def build_ws_dep_entry(key: str, info: dict, mode: str, ws_toml: str,
                       existing_spec: dict, consumer_name: str = None,
                       defaults: dict = None, dep_name: str = None) -> Optional[str]:
    """Build a [workspace.dependencies] entry string for a crate.

    Preserves existing fields like features, default-features, etc.
    Merges required_features from matrix when consumer_name matches.
    Returns None if mode source unavailable.

    Args:
        key: matrix key
        info: matrix info dict
        mode: stable/local/git/zk_local/zk_git
        ws_toml: workspace Cargo.toml relative path
        existing_spec: the existing parsed dependency spec dict
        consumer_name: project/consumer name for required_features lookup
        defaults: global defaults from [defaults] section
    """
    if defaults is None:
        defaults = {}

    if mode == "stable":
        version = info.get("stable", "N/A")
        if version == "N/A":
            return None
        parts = [f'version = "{version}"']
    elif mode == "local":
        local = info.get("local", "N/A")
        if local == "N/A":
            return None
        parts = [f'path = "{rel_path(ws_toml, local)}"']
    elif mode in ("git", "zk_git"):
        url, branch = resolve_git_source(info, mode, defaults)
        if not url:
            return None
        parts = [f'git = "{url}"', f'branch = "{branch}"']
    elif mode == "zk_local":
        zl = info.get("zk_local")
        if zl and zl != "N/A":
            parts = [f'path = "{rel_path(ws_toml, zl)}"']
        else:
            local = info.get("local", "N/A")
            if local == "N/A":
                return None
            parts = [f'path = "{rel_path(ws_toml, local)}"']
    else:
        return None

    # Preserve extra fields from existing spec
    all_features = set()
    pkg = None
    if isinstance(existing_spec, dict):
        pkg = existing_spec.get("package")
        if pkg:
            parts.insert(0, f'package = "{pkg}"')
        features = existing_spec.get("features", [])
        if features:
            all_features.update(features)
        df = existing_spec.get("default-features")
        if df is not None and df is False:
            parts.append("default-features = false")
        if existing_spec.get("optional"):
            parts.append("optional = true")
    # Auto-add package when dep name differs from matrix package name
    if not pkg and dep_name:
        matrix_pkg = info.get("package", key)
        if matrix_pkg != dep_name:
            parts.insert(0, f'package = "{matrix_pkg}"')

    # Merge required_features from matrix for this consumer
    req_feats = info.get("required_features", {})
    req_df_off = info.get("required_default_features_off", [])
    # Build set of consumer name variants for matching
    consumer_names = set()
    if consumer_name:
        consumer_names.add(consumer_name)
        if "-" in consumer_name:
            consumer_names.add(consumer_name.split("-")[0])
    if ws_toml:
        ws_parent = str(Path(ws_toml).parent)
        consumer_names.add(ws_parent)  # e.g. "bme/climb"
        consumer_names.add(ws_parent.split("/")[0])  # e.g. "bme"

    if req_feats and consumer_names:
        for cname in consumer_names:
            if cname in req_feats:
                all_features.update(req_feats[cname])
                break

    # Apply required default-features = false from matrix
    if req_df_off and consumer_names:
        for cname in consumer_names:
            if cname in req_df_off:
                if "default-features = false" not in parts:
                    parts.append("default-features = false")
                break

    if all_features:
        feat_str = ", ".join(f'"{f}"' for f in sorted(all_features))
        parts.append(f"features = [{feat_str}]")

    return "{ " + ", ".join(parts) + " }"


# ── Scan Helpers ─────────────────────────────────────────────────────────────

def _discover_all_cargo_tomls(project_dir: Path) -> list:
    """Walk project_dir and find ALL Cargo.toml files (for deep scanning)."""
    found = []
    for root, dirs, files in os.walk(project_dir):
        # Skip common non-project dirs
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "Cargo.toml" in files:
            found.append(Path(root) / "Cargo.toml")
    return found


def _get_excluded_dirs(project_dir: Path, data: dict) -> set:
    """Get the set of resolved excluded directory paths from workspace config."""
    import glob as glob_mod
    ws = data.get("workspace", {})
    excludes = ws.get("exclude", [])
    excluded = set()
    for exc in excludes:
        for match in glob_mod.glob(str(project_dir / exc)):
            excluded.add(Path(match).resolve())
    return excluded


LOC_EXTENSIONS = {".rs", ".toml", ".ts", ".js", ".py", ".go", ".sol"}


def count_loc(project_dir: Path) -> int:
    """Count lines of source code in a project directory.
    Counts .rs, .toml, .ts, .js, .py, .go, .sol files, skipping target/ etc."""
    total = 0
    for root, dirs, files in os.walk(project_dir):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for f in files:
            ext = os.path.splitext(f)[1]
            if ext in LOC_EXTENSIONS:
                try:
                    # Fast: count newlines without reading full content into memory
                    fp = os.path.join(root, f)
                    with open(fp, "rb") as fh:
                        total += sum(1 for _ in fh)
                except (OSError, UnicodeDecodeError):
                    continue
    return total


def scan_project(project: ProjectRoot, matrix: dict) -> ProjectScanResult:
    """Scan a single project: extract all deps, patches, and compute stats."""
    data, _ = read_cargo_toml(project.cargo_toml)
    ws_deps = extract_workspace_deps(data)
    rel_file = os.path.relpath(project.cargo_toml, REPO_ROOT)

    # Root-level deps
    all_deps = extract_all_deps(data, ws_deps, rel_file)

    # Root-level patches
    patches = extract_patches(data)

    # Member deps
    toml_errors = []
    for member_toml in project.member_tomls:
        try:
            with open(member_toml, "rb") as f:
                member_data = tomllib.load(f)
            member_rel = os.path.relpath(member_toml, REPO_ROOT)
            member_deps = extract_all_deps(member_data, ws_deps, member_rel)
            all_deps.extend(member_deps)
        except Exception as e:
            rel = os.path.relpath(member_toml, REPO_ROOT)
            toml_errors.append(f"[WARN] {project.name}: broken TOML in member '{rel}': {e}")
            continue

    # Deep scan: find ALL Cargo.tomls under the project (including excluded/hidden ones)
    # These are stored separately for diagnostics but not counted in main stats
    proj_dir = project.cargo_toml.parent
    member_set = {t.resolve() for t in project.member_tomls}
    member_set.add(project.cargo_toml.resolve())
    excluded_dirs = _get_excluded_dirs(proj_dir, data)

    hidden_deps = []
    all_tomls = _discover_all_cargo_tomls(proj_dir)
    for toml_path in all_tomls:
        resolved = toml_path.resolve()
        if resolved in member_set:
            continue
        # Determine if this toml is in an excluded dir
        toml_dir = toml_path.parent.resolve()
        is_excluded = any(
            toml_dir == exc or str(toml_dir).startswith(str(exc) + os.sep)
            for exc in excluded_dirs
        )
        try:
            with open(toml_path, "rb") as f:
                hidden_data = tomllib.load(f)
            hidden_rel = os.path.relpath(toml_path, REPO_ROOT)
            deps = extract_all_deps(hidden_data, {}, hidden_rel)
            for dep in deps:
                dep._hidden = True
                dep._is_excluded = is_excluded
            hidden_deps.extend(deps)
        except Exception as e:
            rel = os.path.relpath(toml_path, REPO_ROOT)
            toml_errors.append(f"[WARN] {project.name}: broken TOML '{rel}': {e}")
            continue

    # Compute stats (main deps only, not hidden)
    path_count = sum(1 for d in all_deps if d.source == "path")
    git_count = sum(1 for d in all_deps if d.source == "git")
    reg_count = sum(1 for d in all_deps if d.source == "registry")

    result = ProjectScanResult(
        project=project,
        workspace_deps=ws_deps,
        member_deps=all_deps,
        patches=patches,
        path_dep_count=path_count,
        git_dep_count=git_count,
        registry_dep_count=reg_count,
        patch_count=len(patches),
        loc=count_loc(project.cargo_toml.parent),
    )
    # Seed diagnostics with TOML parse errors found during scan
    result.diagnostics = toml_errors
    # Attach hidden deps for diagnostics
    result._hidden_deps = hidden_deps
    return result


def run_diagnostics(scan: ProjectScanResult, matrix: dict, mode: str = None) -> list:
    """Run diagnostic checks on a scanned project. Returns list of warning strings.
    If mode is specified, compares git branches/URLs against the mode-appropriate
    matrix section (e.g. zk_git when in zk_git mode)."""
    warnings = []
    lookup = pkg_to_matrix(matrix)
    known_pkgs = matrix_pkg_names(matrix)

    # Check git deps that should be local (matrix has a local path)
    seen_git_warns = set()
    for dep in scan.member_deps:
        dep_pkg = dep.pkg_name
        if dep.source == "git" and dep_pkg in lookup:
            _key, info = lookup[dep_pkg]
            local = info.get("local", "N/A")
            if local != "N/A" and dep.name not in seen_git_warns:
                seen_git_warns.add(dep.name)
                warnings.append(
                    f"[WARN] {scan.project.name}: git dep '{dep.name}' has local path in matrix"
                )

    # Check path deps not tracked in matrix
    seen_path_deps = set()
    for dep in scan.member_deps:
        if dep.source == "path" and dep.pkg_name not in known_pkgs:
            # Ignore internal workspace path deps (path starts with . or within same project)
            if dep.path and not dep.path.startswith(".."):
                continue
            seen_path_deps.add(dep.pkg_name)

    # Version mismatches: workspace.dependencies version != matrix stable
    for dep_name, ws_spec in scan.workspace_deps.items():
        if isinstance(ws_spec, dict):
            version = ws_spec.get("version")
        elif isinstance(ws_spec, str):
            version = ws_spec
        else:
            continue

        if dep_name in lookup and version:
            _key, info = lookup[dep_name]
            stable = info.get("stable", "N/A")
            if stable != "N/A" and version and version.lstrip("=^~") != stable:
                warnings.append(
                    f"[INFO] {scan.project.name}: '{dep_name}' version {version} vs matrix stable {stable}"
                )

    # Check git branch mismatch: workspace.dependencies vs matrix
    # Pick the best matrix git section based on mode
    git_keys = _mode_git_keys(mode)

    seen_branch_warns = set()
    for dep_name, ws_spec in scan.workspace_deps.items():
        if isinstance(ws_spec, dict) and ws_spec.get("git"):
            pkg = ws_spec.get("package", dep_name)
            if pkg in lookup:
                _key, info = lookup[pkg]
                # Find the first matching git section for this mode
                matrix_git = None
                for gk in git_keys:
                    mg = info.get(gk)
                    if mg and isinstance(mg, dict):
                        matrix_git = mg
                        break
                if not matrix_git:
                    continue
                matrix_branch = matrix_git.get("branch", "main")
                ws_branch = ws_spec.get("branch", "main")
                matrix_url = matrix_git.get("url", "")
                ws_url = ws_spec["git"]
                # Branch mismatch (same URL or close enough)
                if ws_branch != matrix_branch and dep_name not in seen_branch_warns:
                    seen_branch_warns.add(dep_name)
                    warnings.append(
                        f"[WARN] {scan.project.name}: ws dep '{dep_name}' branch "
                        f"'{ws_branch}' != matrix branch '{matrix_branch}'"
                    )
                # URL mismatch (pointing to different repo entirely)
                if matrix_url and ws_url:
                    m_id = "/".join(matrix_url.rstrip("/").split("/")[-2:]).lower()
                    w_id = "/".join(ws_url.rstrip("/").split("/")[-2:]).lower()
                    if m_id != w_id and dep_name not in seen_branch_warns:
                        seen_branch_warns.add(dep_name)
                        warnings.append(
                            f"[WARN] {scan.project.name}: ws dep '{dep_name}' git URL "
                            f"'{ws_url}' != matrix URL '{matrix_url}'"
                        )

    # Check git branch mismatch: patches vs matrix
    for patch in scan.patches:
        pkg = patch.package or patch.name
        if pkg in lookup and patch.git_url and patch.git_branch:
            _key, info = lookup[pkg]
            matrix_git = None
            for gk in git_keys:
                mg = info.get(gk)
                if mg and isinstance(mg, dict):
                    matrix_git = mg
                    break
            if not matrix_git:
                continue
            matrix_branch = matrix_git.get("branch", "main")
            if patch.git_branch != matrix_branch:
                warnings.append(
                    f"[WARN] {scan.project.name}: patch '{patch.name}' branch "
                    f"'{patch.git_branch}' != matrix branch '{matrix_branch}'"
                )

    # Check for self-referential git deps in workspace.dependencies
    # (workspace members pointing to their own repo's git URL)
    proj_data, _ = read_cargo_toml(scan.project.cargo_toml)
    ws_data = proj_data.get("workspace", {})
    ws_members_raw = ws_data.get("members", [])
    if ws_members_raw:
        # Try to detect the repo's own git URLs
        pkg_section = proj_data.get("package", {})
        ws_pkg = proj_data.get("workspace", {}).get("package", {})
        repo_url = pkg_section.get("repository", "") or ws_pkg.get("repository", "")
        # Normalize: extract github owner/repo
        own_repos = set()
        if repo_url:
            # e.g. "https://github.com/permissionlessweb/tendermint-rs" → "permissionlessweb/tendermint-rs"
            for part in repo_url.replace("https://", "").replace("http://", "").split("/")[1:]:
                pass
            parts = repo_url.rstrip("/").split("/")
            if len(parts) >= 2:
                own_repos.add("/".join(parts[-2:]).lower())

        for dep_name, ws_spec in scan.workspace_deps.items():
            if isinstance(ws_spec, dict) and ws_spec.get("git"):
                git_url = ws_spec["git"].rstrip("/")
                git_parts = git_url.split("/")
                if len(git_parts) >= 2:
                    git_id = "/".join(git_parts[-2:]).lower()
                    if git_id in own_repos:
                        warnings.append(
                            f"[WARN] {scan.project.name}: '{dep_name}' is a self-referential git dep "
                            f"(points to own repo '{git_url}', should use path)"
                        )

    # Check for features referencing non-optional dependencies
    # Runs on root Cargo.toml AND all member Cargo.tomls
    tomls_to_check = [(proj_data, scan.project.cargo_toml)]
    for member_toml in scan.project.member_tomls:
        try:
            with open(member_toml, "rb") as f:
                member_data = tomllib.load(f)
            tomls_to_check.append((member_data, member_toml))
        except Exception:
            continue

    seen_feat_warns = set()
    for check_data, check_path in tomls_to_check:
        check_features = check_data.get("features", {})
        check_deps = check_data.get("dependencies", {})
        check_rel = os.path.relpath(check_path, REPO_ROOT)
        for feat_name, feat_list in check_features.items():
            if not isinstance(feat_list, list):
                continue
            for entry in feat_list:
                if not isinstance(entry, str):
                    continue
                # Handle dep:crate syntax
                if entry.startswith("dep:"):
                    dep_ref = entry[4:]
                    if dep_ref in check_deps:
                        dep_spec = check_deps[dep_ref]
                        is_optional = isinstance(dep_spec, dict) and dep_spec.get("optional", False)
                        if not is_optional:
                            key = (check_rel, feat_name, dep_ref)
                            if key not in seen_feat_warns:
                                seen_feat_warns.add(key)
                                warnings.append(
                                    f"[WARN] {scan.project.name}: '{check_rel}' feature '{feat_name}' "
                                    f"uses 'dep:{dep_ref}' but '{dep_ref}' is not optional"
                                )
                # Handle plain crate name in features that matches a dep name
                # e.g. features: bank = ["osmosis-std"] where osmosis-std is a dep
                elif "/" not in entry and not entry.startswith("dep:"):
                    # Could be "crate/feature" (enabling feature of a dep) or just "crate"
                    dep_ref = entry
                    if dep_ref in check_deps:
                        dep_spec = check_deps[dep_ref]
                        is_optional = isinstance(dep_spec, dict) and dep_spec.get("optional", False)
                        if not is_optional:
                            key = (check_rel, feat_name, dep_ref)
                            if key not in seen_feat_warns:
                                seen_feat_warns.add(key)
                                warnings.append(
                                    f"[WARN] {scan.project.name}: '{check_rel}' feature '{feat_name}' "
                                    f"references '{dep_ref}' but it's not optional"
                                )

    # Check for missing required_features from matrix
    # Matrix entries can specify required_features = { consumer = ["feat1", "feat2"] }
    # to indicate that a consumer must enable those features on the dep
    proj_name = scan.project.name
    # Also match by consumer-style names (e.g. "abstract-framework" matches consumer "abstract")
    proj_consumer_names = {proj_name}
    if "-" in proj_name:
        proj_consumer_names.add(proj_name.split("-")[0])
    proj_consumer_names.add(scan.project.rel_path.split("/")[0])

    for dep_name, ws_spec in scan.workspace_deps.items():
        pkg = dep_name
        if isinstance(ws_spec, dict):
            pkg = ws_spec.get("package", dep_name)

        if pkg not in lookup:
            continue

        _key, info = lookup[pkg]
        req_feats = info.get("required_features", {})
        if not req_feats:
            continue

        # Find which consumer key matches this project
        matched_consumer = None
        for consumer_key in req_feats:
            if consumer_key in proj_consumer_names:
                matched_consumer = consumer_key
                break

        if not matched_consumer:
            continue

        needed = set(req_feats[matched_consumer])
        # Get actual features enabled on this dep
        actual = set()
        if isinstance(ws_spec, dict):
            actual = set(ws_spec.get("features", []))

        missing = needed - actual
        if missing:
            warnings.append(
                f"[WARN] {scan.project.name}: dep '{dep_name}' missing required features "
                f"{sorted(missing)} (needed per matrix required_features.{matched_consumer})"
            )

    # Check for external path deps (path resolves outside the git repo)
    # These break when the project is consumed via git
    seen_ext_paths = set()
    proj_dir = scan.project.rel_path
    # Git repo root: first path component (e.g., "abstract/interchain" → "abstract")
    # Cross-refs within the same git repo are fine (e.g., abstract/framework ↔ abstract/interchain)
    git_root = proj_dir.split("/")[0] if "/" in proj_dir else proj_dir

    def _is_external_path(dep_path, base_dir):
        """Check if a relative path resolves outside the git repo root."""
        if "/../" not in "/" + dep_path + "/" and not dep_path.startswith("../"):
            return False
        resolved = os.path.normpath(os.path.join(base_dir, dep_path))
        # Inside the project dir itself
        if resolved == proj_dir or resolved.startswith(proj_dir + "/"):
            return False
        # Inside the same git repo (e.g., abstract/framework and abstract/interchain)
        if resolved.startswith(git_root + "/") or resolved == git_root:
            return False
        return True

    # 1. Check [workspace.dependencies] entries directly (paths relative to project root)
    for dep_name, ws_spec in scan.workspace_deps.items():
        if isinstance(ws_spec, dict) and ws_spec.get("path"):
            dep_path = ws_spec["path"]
            if _is_external_path(dep_path, proj_dir):
                key = (dep_name, dep_path)
                if key not in seen_ext_paths:
                    seen_ext_paths.add(key)
                    warnings.append(
                        f"[WARN] {scan.project.name}: '{dep_name}' uses external path '{dep_path}' "
                        f"(breaks git resolution)"
                    )

    # 2. Check member deps (skip workspace-inherited since ws-deps check already covers those)
    for dep in scan.member_deps:
        if dep.source == "path" and dep.path and not dep.workspace_inherited:
            dep_path = dep.path
            declaring_dir = str(Path(dep.declaring_file).parent)
            if _is_external_path(dep_path, declaring_dir):
                key = (dep.pkg_name, dep_path)
                if key not in seen_ext_paths:
                    seen_ext_paths.add(key)
                    warnings.append(
                        f"[WARN] {scan.project.name}: '{dep.name}' uses external path '{dep_path}' "
                        f"(breaks git resolution)"
                    )

    # 3. Check hidden/excluded Cargo.tomls for external path deps
    # These are sub-Cargo.tomls not in workspace members — cargo auto-discovers them
    # if not properly excluded, and even excluded ones can cause issues
    hidden_deps = getattr(scan, "_hidden_deps", [])
    for dep in hidden_deps:
        if dep.source == "path" and dep.path:
            dep_path = dep.path
            declaring_dir = str(Path(dep.declaring_file).parent)
            if _is_external_path(dep_path, declaring_dir):
                is_excluded = getattr(dep, "_is_excluded", False)
                key = (dep.pkg_name, dep_path, dep.declaring_file)
                if key not in seen_ext_paths:
                    seen_ext_paths.add(key)
                    rel_file = dep.declaring_file
                    if is_excluded:
                        warnings.append(
                            f"[INFO] {scan.project.name}: excluded '{rel_file}' has external path dep "
                            f"'{dep.name}' → '{dep_path}'"
                        )
                    else:
                        warnings.append(
                            f"[WARN] {scan.project.name}: hidden '{rel_file}' has external path dep "
                            f"'{dep.name}' → '{dep_path}' (not excluded, breaks git resolution!)"
                        )

    scan.diagnostics.extend(warnings)
    return warnings


def run_cross_project_diagnostics(scans: list, matrix: dict) -> list:
    """Run diagnostics that compare across multiple projects.
    Detects when the same package is sourced from divergent git repos/branches.
    Returns list of warning strings."""
    warnings = []

    # Collect all git sources by package name (or alias) across projects
    # Key: effective package name, Value: list of (project, toml_key, git_url, branch)
    git_sources = {}

    for scan in scans:
        # From workspace.dependencies
        for dep_name, ws_spec in scan.workspace_deps.items():
            if isinstance(ws_spec, dict) and ws_spec.get("git"):
                pkg = ws_spec.get("package", dep_name)
                git_url = ws_spec["git"]
                branch = ws_spec.get("branch")
                git_sources.setdefault(pkg, []).append(
                    (scan.project.name, dep_name, git_url, branch)
                )

        # From patches
        for patch in scan.patches:
            if patch.git_url:
                pkg = patch.package or patch.name
                git_sources.setdefault(pkg, []).append(
                    (scan.project.name, patch.name, patch.git_url, patch.git_branch)
                )

    # Find divergent sources: same package, different git URL or branch
    for pkg, sources in git_sources.items():
        # Normalize URLs for comparison
        url_branches = set()
        for proj, key, url, branch in sources:
            norm_url = "/".join(url.rstrip("/").split("/")[-2:]).lower()
            url_branches.add((norm_url, branch or "(default)"))

        if len(url_branches) > 1:
            details = "; ".join(
                f"{proj}: {key}→{url}@{branch or '(default)'}"
                for proj, key, url, branch in sources
            )
            warnings.append(
                f"[WARN] cross-project: package '{pkg}' has divergent git sources: {details}"
            )

    return warnings


def validatetomls() -> list:
    """Walk all local repo clones and validate every Cargo.toml parses correctly.
    Returns list of warning strings for broken files."""
    warnings = []

    for entry in sorted(REPO_ROOT.iterdir()):
        if not entry.is_dir() or entry.name in SKIP_DIRS or entry.name.startswith("."):
            continue

        # For the 'abstract' dir, walk sub-workspaces
        if entry.name == "abstract":
            dirs_to_walk = [sub for sub in sorted(entry.iterdir()) if sub.is_dir()]
        else:
            dirs_to_walk = [entry]

        for walk_dir in dirs_to_walk:
            for root, dirs, files in os.walk(walk_dir):
                dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
                if "Cargo.toml" in files:
                    toml_path = Path(root) / "Cargo.toml"
                    try:
                        with open(toml_path, "rb") as f:
                            tomllib.load(f)
                    except Exception as e:
                        rel = os.path.relpath(toml_path, REPO_ROOT)
                        # Extract just the first line of the error for brevity
                        err_msg = str(e).split("\n")[0]
                        warnings.append(
                            f"[WARN] broken TOML: '{rel}': {err_msg}"
                        )

    return warnings


def validate_feature_optional_deps() -> list:
    """Walk all local repo Cargo.tomls and check for features referencing
    non-optional dependencies. Cargo rejects this at parse time so catching
    it here prevents 'cargo update' and 'cargo check' from failing.

    Checks both dep:crate and plain crate name syntax in [features]."""
    warnings = []

    for entry in sorted(REPO_ROOT.iterdir()):
        if not entry.is_dir() or entry.name in SKIP_DIRS or entry.name.startswith("."):
            continue

        if entry.name == "abstract":
            dirs_to_walk = [sub for sub in sorted(entry.iterdir()) if sub.is_dir()]
        else:
            dirs_to_walk = [entry]

        for walk_dir in dirs_to_walk:
            for root, dirs, files in os.walk(walk_dir):
                dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
                if "Cargo.toml" not in files:
                    continue
                toml_path = Path(root) / "Cargo.toml"
                try:
                    with open(toml_path, "rb") as f:
                        data = tomllib.load(f)
                except Exception:
                    continue

                features = data.get("features", {})
                deps = data.get("dependencies", {})
                if not features or not deps:
                    continue

                rel = os.path.relpath(toml_path, REPO_ROOT)
                for feat_name, feat_list in features.items():
                    if not isinstance(feat_list, list):
                        continue
                    for entry_str in feat_list:
                        if not isinstance(entry_str, str):
                            continue
                        # dep:crate syntax
                        if entry_str.startswith("dep:"):
                            dep_ref = entry_str[4:]
                        elif "/" not in entry_str:
                            dep_ref = entry_str
                        else:
                            continue
                        if dep_ref in deps:
                            dep_spec = deps[dep_ref]
                            is_optional = isinstance(dep_spec, dict) and dep_spec.get("optional", False)
                            if not is_optional:
                                warnings.append(
                                    f"[WARN] '{rel}': feature '{feat_name}' references "
                                    f"'{dep_ref}' but it is not optional"
                                )

    return warnings


def validate_cargo_locks() -> list:
    """Scan all Cargo.lock files for duplicate crate instances from different
    sources (e.g., same crate from registry AND git, or two git branches).
    This catches the ics23-style two-instance type-incompatibility bug.

    Returns list of warning strings."""
    warnings = []

    for entry in sorted(REPO_ROOT.iterdir()):
        if not entry.is_dir() or entry.name in SKIP_DIRS or entry.name.startswith("."):
            continue

        if entry.name == "abstract":
            dirs_to_scan = [sub for sub in sorted(entry.iterdir()) if sub.is_dir()]
        else:
            dirs_to_scan = [entry]

        for scan_dir in dirs_to_scan:
            # Only check top-level Cargo.lock (not wasm/ subprojects)
            lock_path = scan_dir / "Cargo.lock"
            if not lock_path.exists():
                continue

            rel = os.path.relpath(lock_path, REPO_ROOT)
            try:
                content = lock_path.read_text()
            except Exception:
                continue

            # Parse [[package]] entries: collect (name, version, source) tuples
            packages = {}  # name -> list of (version, source)
            current_name = None
            current_version = None
            current_source = None

            for line in content.splitlines():
                stripped = line.strip()
                if stripped == "[[package]]":
                    if current_name:
                        packages.setdefault(current_name, []).append(
                            (current_version or "?", current_source or "local")
                        )
                    current_name = current_version = current_source = None
                elif stripped.startswith('name = "'):
                    current_name = stripped.split('"')[1]
                elif stripped.startswith('version = "'):
                    current_version = stripped.split('"')[1]
                elif stripped.startswith('source = "'):
                    current_source = stripped.split('"')[1]

            # Don't forget last entry
            if current_name:
                packages.setdefault(current_name, []).append(
                    (current_version or "?", current_source or "local")
                )

            # Flag duplicates with different sources
            for pkg_name, entries in packages.items():
                if len(entries) <= 1:
                    continue
                # Group by source type (registry vs git vs path)
                sources = set()
                for ver, src in entries:
                    if src.startswith("git+"):
                        # Normalize: extract url+branch
                        sources.add(src.split("#")[0])
                    else:
                        sources.add(src)
                if len(sources) > 1:
                    src_list = ", ".join(
                        f"v{ver} from {src[:80]}" for ver, src in entries
                    )
                    warnings.append(
                        f"[WARN] '{rel}': duplicate crate '{pkg_name}' "
                        f"from different sources: {src_list}"
                    )

    return warnings


def _mode_path_keys(mode: str = None) -> list:
    """Return which path keys (local/zk_local) are relevant for a mode."""
    if mode in ("zk_local", "zk_git"):
        return ["zk_local", "local"]  # zk_local with local fallback
    if mode in ("local", "git"):
        return ["local"]
    return ["local", "zk_local"]  # None = check all


def _mode_git_keys(mode: str = None) -> list:
    """Return which git keys (git/zk_git) are relevant for a mode."""
    if mode in ("zk_git", "zk_local"):
        return ["zk_git", "git"]  # zk_git with git fallback
    if mode in ("git", "local"):
        return ["git"]
    return ["git", "zk_git"]  # None = check all


def validate_matrix_branches(matrix: dict, mode: str = None) -> list:
    """Check that git and zk_git branches in the matrix exist on their remotes.
    Groups by URL to minimize network calls (one git ls-remote per URL).
    If mode is specified, only checks branches relevant to that mode.
    Returns list of warning strings for missing branches."""
    import subprocess

    git_keys = _mode_git_keys(mode)

    # Collect all (url, branch, key, field) tuples
    checks = []
    for key, info in sorted(matrix.items()):
        for gk in git_keys:
            gi = info.get(gk)
            if gi and isinstance(gi, dict):
                url = gi.get("url")
                branch = gi.get("branch")
                if url and branch:
                    checks.append((url, branch, key, gk))

    # Group by URL → { url: set((branch, key, field)) }
    url_groups = {}
    for url, branch, key, field in checks:
        url_groups.setdefault(url, set()).add((branch, key, field))

    warnings = []
    for url in sorted(url_groups):
        entries = url_groups[url]
        # Collect unique branches needed from this URL
        needed_branches = {branch for branch, _, _ in entries}
        try:
            r = subprocess.run(
                ["git", "ls-remote", "--heads", url],
                capture_output=True, text=True, timeout=30,
            )
            if r.returncode != 0:
                warnings.append(
                    f"[WARN] git ls-remote failed for {url}: {r.stderr.strip()}"
                )
                continue

            # Parse refs/heads/<branch> from output
            remote_branches = set()
            for line in r.stdout.strip().split("\n"):
                if not line:
                    continue
                parts = line.split("\t")
                if len(parts) == 2 and parts[1].startswith("refs/heads/"):
                    remote_branches.add(parts[1][len("refs/heads/"):])

            # Check each needed branch
            for branch, key, field in sorted(entries):
                if branch not in remote_branches:
                    warnings.append(
                        f"[WARN] matrix '{key}': {field} branch '{branch}' "
                        f"not found on {url}"
                    )
        except subprocess.TimeoutExpired:
            warnings.append(f"[WARN] git ls-remote timed out for {url}")
        except Exception as e:
            warnings.append(f"[WARN] git ls-remote error for {url}: {e}")

    return warnings


def validate_matrix_paths(matrix: dict, mode: str = None) -> list:
    """Check that local and zk_local paths in the matrix actually exist on disk.
    If mode is specified, only checks paths relevant to that mode.
    Returns list of warning strings for missing paths."""
    warnings = []
    path_keys = _mode_path_keys(mode)

    for key, info in sorted(matrix.items()):
        for pk in path_keys:
            local = info.get(pk)
            if local and local != "N/A":
                resolved = REPO_ROOT / local.lstrip("./")
                if not resolved.exists():
                    warnings.append(
                        f"[WARN] matrix '{key}': {pk} path '{local}' does not exist"
                    )
                break  # first found path is sufficient (fallback order)

    return warnings


def validate_matrix_packages(matrix: dict, mode: str = None) -> list:
    """Verify that Cargo.toml at each matrix path has the expected package name.

    For each matrix entry with a local or zk_local path, reads the Cargo.toml
    at that path and checks the [package] name matches the matrix's expected
    package name. Also checks that dep_aliases are not stale.

    If mode is specified, only checks paths relevant to that mode.
    Returns list of warning/info strings."""
    warnings = []
    path_keys = _mode_path_keys(mode)

    for key, info in sorted(matrix.items()):
        if is_repo_only(info):
            continue

        expected_pkg = info.get("package", key)
        aliases = set(info.get("dep_aliases", []))

        for path_key in path_keys:
            local = info.get(path_key)
            if not local or local == "N/A":
                continue

            resolved = REPO_ROOT / local.lstrip("./")
            cargo_toml = resolved / "Cargo.toml" if resolved.is_dir() else resolved
            if not cargo_toml.exists():
                continue  # validate_matrix_paths already flags this

            try:
                import tomllib as _toml
            except ImportError:
                try:
                    import tomli as _toml
                except ImportError:
                    continue

            try:
                with open(cargo_toml, "rb") as f:
                    data = _toml.load(f)
            except Exception:
                continue

            actual_name = data.get("package", {}).get("name")
            if not actual_name:
                # Workspace root (no [package]) or virtual manifest — skip
                continue

            # Check package name matches
            if actual_name != expected_pkg:
                warnings.append(
                    f"[WARN] matrix '{key}': {path_key} '{local}' has package "
                    f"name '{actual_name}', expected '{expected_pkg}'"
                )
                # Suggest dep_aliases if the actual name is commonly used as a dep name
                if actual_name not in aliases and actual_name != key:
                    warnings.append(
                        f"  [HINT] Consider adding dep_aliases = [\"{actual_name}\"] "
                        f"to matrix entry '{key}' if consumers import it as '{actual_name}'"
                    )

            # Check dep_aliases consistency: each alias should differ from package name
            for alias in aliases:
                if alias == expected_pkg:
                    warnings.append(
                        f"[WARN] matrix '{key}': dep_alias '{alias}' is same as "
                        f"package name (redundant)"
                    )

    return warnings
