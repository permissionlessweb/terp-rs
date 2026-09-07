#!/usr/bin/env python3
"""
Unified Dependency Report for the Abstract monorepo.

Scans all Cargo.toml files, classifies deps, runs diagnostics, and generates
a single markdown report combining the scraper table, overview dashboard,
matrix coverage, and diagnostics.

Also writes _devops/dep-scrape.json for programmatic consumption.

Usage:
    dep-report.py                         # full report → MD + JSON + stdout summary
    dep-report.py --output path.md        # custom markdown output
    dep-report.py --project cw-plus       # single project
    dep-report.py --json-only             # JSON only (_devops/dep-scrape.json)
    dep-report.py --summary-only          # human summary to stdout only
    dep-report.py --check                 # exit 1 on warnings (CI mode)
"""

import argparse
import json
import os
import subprocess
import sys
from datetime import date
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from dep_common import (
    DEVOPS_DIR,
    REPO_ROOT,
    discover_projects,
    load_active_mode,
    load_matrix,
    load_migration_progress,
    matrix_pkg_names,
    pkg_to_matrix,
    run_cross_project_diagnostics,
    run_diagnostics,
    scan_project,
    validate_cargo_locks,
    validate_feature_optional_deps,
    validate_matrix_branches,
    validate_matrix_packages,
    validate_matrix_paths,
    validatetomls,
)


# ── Git Remote Helpers ─────────────────────────────────────────────────────

def _build_readme_urls(results):
    """Build {rel_path: GitHub README URL} for all projects with READMEs.

    Caches git remote info per repo root so we only query each repo once.
    Prefers remotes containing 'permissionlessweb', then tries origin/fork/first.
    """
    git_cache = {}  # git_root_str -> (base_url, branch) | None
    urls = {}

    for scan in results:
        proj = scan.project
        if not proj.has_readme:
            continue

        proj_dir = REPO_ROOT / proj.rel_path

        # Find git root
        try:
            r = subprocess.run(
                ["git", "-C", str(proj_dir), "rev-parse", "--show-toplevel"],
                capture_output=True, text=True, timeout=5,
            )
            if r.returncode != 0:
                continue
            git_root = r.stdout.strip()
        except Exception:
            continue

        if git_root not in git_cache:
            git_cache[git_root] = _resolve_git_remote(git_root)

        info = git_cache[git_root]
        if not info:
            continue

        base_url, branch = info
        rel = os.path.relpath(str(proj_dir), git_root)

        if rel == ".":
            urls[proj.rel_path] = f"{base_url}/blob/{branch}/README.md"
        else:
            urls[proj.rel_path] = f"{base_url}/blob/{branch}/{rel}/README.md"

    return urls


def _resolve_git_remote(git_root):
    """Resolve (https_url, branch) for a git root, or None."""
    try:
        # List all remotes with URLs
        r = subprocess.run(
            ["git", "-C", git_root, "remote", "-v"],
            capture_output=True, text=True, timeout=5,
        )
        if r.returncode != 0 or not r.stdout.strip():
            return None

        # Parse remote URLs (fetch lines)
        remotes = {}
        for line in r.stdout.strip().split("\n"):
            parts = line.split()
            if len(parts) >= 2 and "(fetch)" in line:
                remotes[parts[0]] = parts[1]

        if not remotes:
            return None

        # Pick best remote: prefer permissionlessweb, then origin, then fork, then first
        chosen = None
        for name, url in remotes.items():
            if "permissionlessweb" in url:
                chosen = url
                break
        if not chosen:
            for pref in ("origin", "fork"):
                if pref in remotes:
                    chosen = remotes[pref]
                    break
        if not chosen:
            chosen = next(iter(remotes.values()))

        # Normalize to HTTPS
        url = chosen
        if url.startswith("git@"):
            url = "https://" + url[4:].replace(":", "/", 1)
        if url.endswith(".git"):
            url = url[:-4]

        # Get current branch
        br = subprocess.run(
            ["git", "-C", git_root, "rev-parse", "--abbrev-ref", "HEAD"],
            capture_output=True, text=True, timeout=5,
        )
        branch = br.stdout.strip() if br.returncode == 0 else "main"

        return (url, branch)
    except Exception:
        return None


# ── Scanning ────────────────────────────────────────────────────────────────

def scan_all(matrix, project_filter=None, mode=None):
    """Scan all projects. Returns list of ProjectScanResult."""
    projects = discover_projects()

    if project_filter:
        projects = [
            p for p in projects
            if p.name == project_filter or p.rel_path == project_filter
        ]
        if not projects:
            print(f"ERROR: project '{project_filter}' not found")
            sys.exit(1)

    results = []
    for proj in projects:
        scan = scan_project(proj, matrix)
        run_diagnostics(scan, matrix, mode=mode)
        results.append(scan)

    return results


def run_global_diagnostics(results, matrix, mode=None):
    """Run cross-project and TOML validation diagnostics.
    If mode is specified, only validates paths/branches relevant to that mode."""
    cross_warnings = run_cross_project_diagnostics(results, matrix)
    toml_warnings = validatetomls()
    toml_warnings.extend(validate_feature_optional_deps())
    toml_warnings.extend(validate_cargo_locks())
    toml_warnings.extend(validate_matrix_paths(matrix, mode=mode))
    toml_warnings.extend(validate_matrix_packages(matrix, mode=mode))
    print("Checking remote branches (this may take a moment)...")
    toml_warnings.extend(validate_matrix_branches(matrix, mode=mode))
    return cross_warnings, toml_warnings


# ── JSON Output ─────────────────────────────────────────────────────────────

def results_to_json(results, matrix, cross_warnings=None, toml_warnings=None):
    """Convert scan results to a JSON-serializable dict."""
    known = matrix_pkg_names(matrix)

    projects = []
    all_diagnostics = []

    for scan in results:
        proj = scan.project
        proj_data = {
            "name": proj.name,
            "path": proj.rel_path,
            "is_workspace": proj.is_workspace,
            "members": len(proj.member_tomls),
            "has_readme": proj.has_readme,
            "deps": {
                "path": scan.path_dep_count,
                "git": scan.git_dep_count,
                "registry": scan.registry_dep_count,
                "total": scan.path_dep_count + scan.git_dep_count + scan.registry_dep_count,
            },
            "patches": scan.patch_count,
            "loc": scan.loc,
            "diagnostics": scan.diagnostics,
        }

        dep_by_source = {"path": {}, "git": {}, "registry": {}}
        for dep in scan.member_deps:
            src = dep.source
            pkg = dep.pkg_name
            if pkg not in dep_by_source[src]:
                dep_by_source[src][pkg] = {
                    "name": dep.name,
                    "package": dep.package,
                    "in_matrix": pkg in known,
                }
                if dep.path:
                    dep_by_source[src][pkg]["path"] = dep.path
                if dep.git_url:
                    dep_by_source[src][pkg]["git_url"] = dep.git_url
                if dep.version:
                    dep_by_source[src][pkg]["version"] = dep.version

        proj_data["dep_details"] = {
            src: list(deps.values()) for src, deps in dep_by_source.items()
        }

        projects.append(proj_data)
        all_diagnostics.extend(scan.diagnostics)

    global_diags = (cross_warnings or []) + (toml_warnings or [])

    return {
        "repo_root": str(REPO_ROOT),
        "matrix_crates": len(matrix),
        "projects_scanned": len(projects),
        "total_diagnostics": len(all_diagnostics) + len(global_diags),
        "projects": projects,
        "diagnostics": all_diagnostics,
        "cross_project_diagnostics": cross_warnings or [],
        "toml_validation": toml_warnings or [],
    }


# ── Stdout Summary ──────────────────────────────────────────────────────────

def print_summary(results, matrix, cross_warnings=None, toml_warnings=None):
    """Print a human-readable summary to stdout."""
    total_members = 0
    total_deps = 0
    total_loc = 0
    all_diags = []

    print("=== Dependency Report ===")
    print()

    print(f"  {'Project':<28} {'Type':<12} {'Members':>7} {'Path':>5} {'Git':>5} {'Reg':>5} {'Patch':>6} {'LOC':>9}")
    print(f"  {'-'*28} {'-'*12} {'-'*7} {'-'*5} {'-'*5} {'-'*5} {'-'*6} {'-'*9}")

    for scan in results:
        proj = scan.project
        ptype = "workspace" if proj.is_workspace else "package"
        members = len(proj.member_tomls) if proj.is_workspace else 0
        total_members += members
        total_deps += scan.path_dep_count + scan.git_dep_count + scan.registry_dep_count
        total_loc += scan.loc
        all_diags.extend(scan.diagnostics)

        loc_str = f"{scan.loc:,}"
        print(
            f"  {proj.name:<28} {ptype:<12} {members:>7} "
            f"{scan.path_dep_count:>5} {scan.git_dep_count:>5} "
            f"{scan.registry_dep_count:>5} {scan.patch_count:>6} {loc_str:>9}"
        )

    print()
    print(f"Scanned: {len(results)} projects, {total_members} workspace members, {total_deps} dep entries, {total_loc:,} LOC")
    print(f"Matrix: {len(matrix)} crate definitions")
    print()

    if all_diags:
        print("Per-project Diagnostics:")
        for diag in all_diags:
            print(f"  {diag}")
        print()

    if cross_warnings:
        print("Cross-project Diagnostics:")
        for diag in cross_warnings:
            print(f"  {diag}")
        print()

    if toml_warnings:
        print("TOML Validation (all local clones):")
        for diag in toml_warnings:
            print(f"  {diag}")
        print()


# ── Markdown Generation ─────────────────────────────────────────────────────

STATUS_ICONS = {
    "green": ":green_circle:",
    "yellow": ":yellow_circle:",
    "red": ":red_circle:",
    "skipped": ":white_circle:",
}

LAYER_ORDER = [
    "Foundation Layer",
    "Core CW Libraries",
    "Test Frameworks",
    "Application Contracts",
    "Core SDK + Orchestrator",
    "Other",
]

LAYER_NAMES = {
    "Foundation Layer": {"tendermint-rs", "cosmos-rust", "ibc-proto-rs", "ics23-rust"},
    "Core CW Libraries": {"cw-minus", "cw-plus", "cw-plus-plus", "cw-asset", "cw-nfts"},
    "Test Frameworks": {
        "cw-multi-test-fork", "clone-cw-multi-test",
        "osmosis-test-tube", "neutron-test-tube", "neutron-std", "osmosis-rust",
    },
    "Application Contracts": {
        "polytone", "cw-ibc-demo", "wynddex", "wynd-lsd",
        "dao-contracts", "xion-account", "cw-packages", "abstract-cw-plus",
    },
    "Core SDK + Orchestrator": {"cw-orchestrator", "abstract"},
    "Other": {"proxy-accounts", "new-cosmic-wavs", "halo2-axiom"},
}


def _classify_layer(name):
    """Classify a migration entry into a layer."""
    for layer, names in LAYER_NAMES.items():
        if name in names:
            return layer
    return "Other"


def _get_migration_entry(scan, mig_by_name, mig_by_path):
    """Find the migration entry for a project."""
    name = scan.project.name
    path = scan.project.rel_path

    if name in mig_by_name:
        return mig_by_name[name]
    if path in mig_by_path:
        return mig_by_path[path]

    if name.startswith("abstract-"):
        short = name.replace("abstract-", "", 1)
        if short in mig_by_name:
            return mig_by_name[short]

    if path.startswith("abstract/"):
        if "abstract" in mig_by_name and path == "abstract/framework":
            return mig_by_name["abstract"]

    return None


def generate_markdown(results, matrix, cross_warnings, toml_warnings):
    """Generate the unified report markdown."""
    migration = load_migration_progress()
    today = date.today().isoformat()
    readme_urls = _build_readme_urls(results)

    mig_by_name = {}
    mig_by_path = {}
    for m in migration:
        mig_by_name[m.get("name", "")] = m
        mig_by_path[m.get("path", "")] = m

    known = matrix_pkg_names(matrix)
    lookup = pkg_to_matrix(matrix)
    lines = []

    # ── Header ────────────────────────────────────────────────────────────
    lines.append("# Abstract Monorepo — Dependency Report")
    lines.append("")
    lines.append(f"*Auto-generated by `dep-report.py` on {today}*")
    lines.append("")

    # ── Summary ───────────────────────────────────────────────────────────
    total_projects = len(results)
    total_members = sum(len(s.project.member_tomls) for s in results)
    total_deps = sum(s.path_dep_count + s.git_dep_count + s.registry_dep_count for s in results)
    total_patches = sum(s.patch_count for s in results)
    total_loc = sum(s.loc for s in results)

    mig_check = [m for m in migration if m.get("cargo_check") == "green"]
    mig_test_green = [m for m in migration if m.get("cargo_test") == "green"]
    mig_test_red = [m for m in migration if m.get("cargo_test") == "red"]

    lines.append("## Summary")
    lines.append("")
    lines.append(f"| Metric | Value |")
    lines.append(f"|--------|------:|")
    lines.append(f"| Projects | {total_projects} |")
    lines.append(f"| Workspace members | {total_members} |")
    lines.append(f"| Total dep entries | {total_deps} |")
    lines.append(f"| Total patches | {total_patches} |")
    lines.append(f"| Total LOC | {total_loc:,} |")
    lines.append(f"| Matrix crates | {len(matrix)} |")
    lines.append(f"| Migration check pass | {len(mig_check)}/{len(migration)} |")
    lines.append(f"| Migration test pass | {len(mig_test_green)}/{len(migration)} ({len(mig_test_red)} red) |")
    lines.append("")

    # ── Scraper Table ─────────────────────────────────────────────────────
    lines.append("## All Projects")
    lines.append("")
    lines.append("| Project | Path | Type | Members | Path | Git | Reg | Patches | LOC |")
    lines.append("|---------|------|------|--------:|-----:|----:|----:|--------:|----:|")

    for scan in results:
        proj = scan.project
        ptype = "ws" if proj.is_workspace else "pkg"
        members = len(proj.member_tomls) if proj.is_workspace else 0
        name_display = proj.name
        readme_url = readme_urls.get(proj.rel_path)
        if readme_url:
            name_display = f"[{proj.name}]({readme_url})"

        lines.append(
            f"| {name_display} | `{proj.rel_path}` | {ptype} | {members} "
            f"| {scan.path_dep_count} | {scan.git_dep_count} "
            f"| {scan.registry_dep_count} | {scan.patch_count} | {scan.loc:,} |"
        )

    lines.append("")

    # ── Projects by Layer ─────────────────────────────────────────────────
    layer_map = {}
    for m in migration:
        name = m.get("name", "")
        layer_map[name] = _classify_layer(name)

    grouped = {layer: [] for layer in LAYER_ORDER}
    for scan in results:
        mig = _get_migration_entry(scan, mig_by_name, mig_by_path)
        if mig:
            layer = layer_map.get(mig.get("name", ""), "Other")
        else:
            layer = "Other"
        grouped[layer].append((scan, mig))

    lines.append("## Projects by Layer")
    lines.append("")

    for layer in LAYER_ORDER:
        items = grouped[layer]
        if not items:
            continue

        lines.append(f"### {layer}")
        lines.append("")
        lines.append("| Project | Path | Members | Path | Git | Reg | Patches | Check | Test |")
        lines.append("|---------|------|--------:|-----:|----:|----:|--------:|:-----:|:----:|")

        for scan, mig in items:
            proj = scan.project
            name_display = proj.name
            readme_url = readme_urls.get(proj.rel_path)
            if readme_url:
                name_display = f"[{proj.name}]({readme_url})"

            members = len(proj.member_tomls) if proj.is_workspace else 0

            check_icon = ""
            test_icon = ""
            if mig:
                check_icon = STATUS_ICONS.get(mig.get("cargo_check", ""), "")
                test_icon = STATUS_ICONS.get(mig.get("cargo_test", ""), "")

            lines.append(
                f"| {name_display} | `{proj.rel_path}` | {members} "
                f"| {scan.path_dep_count} | {scan.git_dep_count} "
                f"| {scan.registry_dep_count} | {scan.patch_count} "
                f"| {check_icon} | {test_icon} |"
            )

        lines.append("")

    # ── Diagnostics ───────────────────────────────────────────────────────
    all_diags = []
    for scan in results:
        all_diags.extend(scan.diagnostics)

    all_warnings = [d for d in all_diags if d.startswith("[WARN]")]
    all_infos = [d for d in all_diags if d.startswith("[INFO]")]

    has_diagnostics = all_warnings or all_infos or cross_warnings or toml_warnings

    if has_diagnostics:
        lines.append("## Diagnostics")
        lines.append("")

        if all_warnings:
            lines.append(f"### Per-project Warnings ({len(all_warnings)})")
            lines.append("")
            for w in all_warnings:
                lines.append(f"- {w}")
            lines.append("")

        if all_infos:
            lines.append(f"### Per-project Info ({len(all_infos)})")
            lines.append("")
            for i in all_infos:
                lines.append(f"- {i}")
            lines.append("")

        if cross_warnings:
            lines.append(f"### Cross-project ({len(cross_warnings)})")
            lines.append("")
            for d in cross_warnings:
                lines.append(f"- {d}")
            lines.append("")

        if toml_warnings:
            lines.append(f"### TOML Validation ({len(toml_warnings)})")
            lines.append("")
            for d in toml_warnings:
                lines.append(f"- {d}")
            lines.append("")

    # ── Matrix Coverage ───────────────────────────────────────────────────
    lines.append("## Matrix Coverage")
    lines.append("")
    lines.append("| Matrix Crate | Package | Stable | Git Branch | ZK Branch | Origin | Found In |")
    lines.append("|-------------|---------|--------|------------|-----------|--------|----------|")

    proj_paths = {scan.project.name: scan.project.rel_path for scan in results}

    actual_consumers = {}
    for scan in results:
        for dep in scan.member_deps:
            pkg = dep.pkg_name
            if pkg in known:
                actual_consumers.setdefault(pkg, set()).add(scan.project.name)

    def _link_names(names, paths_map):
        parts = []
        for n in names:
            if n in paths_map:
                parts.append(f"[{n}]({paths_map[n]}/Cargo.toml)")
            else:
                parts.append(n)
        return ", ".join(parts)

    for key, info in sorted(matrix.items()):
        pkg = info.get("package", key)
        stable = info.get("stable", "N/A")
        actual = sorted(actual_consumers.get(pkg, set()))
        actual_str = _link_names(actual, proj_paths)

        # Git branch + origin
        gi = info.get("git", {})
        git_branch = gi.get("branch", "") if isinstance(gi, dict) else ""
        git_origin = gi.get("origin", "") if isinstance(gi, dict) else ""

        # ZK branch + origin
        zk = info.get("zk_git", {})
        zk_branch = zk.get("branch", "") if isinstance(zk, dict) else ""
        zk_origin = zk.get("origin", "") if isinstance(zk, dict) else ""

        # Combine origins (show if either git or zk_git specifies one)
        origin_str = git_origin or zk_origin or ""

        lines.append(
            f"| {key} | {pkg} | {stable} | {git_branch} | {zk_branch} "
            f"| {origin_str} | {actual_str} |"
        )

    lines.append("")

    # ── Migration Blockers ────────────────────────────────────────────────
    blockers = [
        m for m in migration
        if m.get("blockers") and m["blockers"] != []
    ]
    if blockers:
        lines.append("## Active Blockers")
        lines.append("")
        for m in blockers:
            lines.append(f"**{m['name']}:**")
            for b in m.get("blockers", []):
                lines.append(f"- {b}")
            lines.append("")

    return "\n".join(lines)


# ── Main ────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Unified dependency report: scan, diagnose, and generate markdown"
    )
    parser.add_argument("--project", help="Scan only this project (name or path)")
    parser.add_argument("--output", help="Markdown output path (default: _devops/REPORT.md)")
    parser.add_argument("--json-only", action="store_true", help="Output JSON only")
    parser.add_argument("--summary-only", action="store_true", help="Stdout summary only (no files)")
    parser.add_argument("--mode", choices=["stable", "local", "git", "zk_local", "zk_git"],
                        help="Only validate paths/branches relevant to this mode")
    parser.add_argument("--check", action="store_true", help="Exit 1 on warnings (CI mode)")
    args = parser.parse_args()

    mode = args.mode or load_active_mode()
    if mode:
        src = "--mode" if args.mode else "auto-detected from .dep-mode"
        print(f"Using mode: {mode} ({src})")
        print()

    matrix = load_matrix()
    results = scan_all(matrix, args.project, mode=mode)
    cross_warnings, toml_warnings = run_global_diagnostics(results, matrix, mode=mode)

    # JSON output
    if not args.summary_only:
        json_data = results_to_json(results, matrix, cross_warnings, toml_warnings)
        json_path = DEVOPS_DIR / "dep-scrape.json"
        json_path.parent.mkdir(parents=True, exist_ok=True)
        with open(json_path, "w") as f:
            json.dump(json_data, f, indent=2)
        if not args.json_only:
            print(f"Wrote {json_path}")

    # Markdown output
    if not args.json_only and not args.summary_only:
        md = generate_markdown(results, matrix, cross_warnings, toml_warnings)
        md_path = Path(args.output) if args.output else DEVOPS_DIR / "REPORT.md"
        md_path.parent.mkdir(parents=True, exist_ok=True)
        md_path.write_text(md)
        print(f"Wrote {md_path}")
        print()

    # Stdout summary
    if not args.json_only:
        print_summary(results, matrix, cross_warnings, toml_warnings)

    # CI check
    if args.check:
        warns = []
        for scan in results:
            warns.extend(d for d in scan.diagnostics if d.startswith("[WARN]"))
        warns.extend(d for d in (cross_warnings or []) if d.startswith("[WARN]"))
        warns.extend(d for d in (toml_warnings or []) if d.startswith("[WARN]"))
        if warns:
            print(f"FAIL: {len(warns)} warnings found")
            sys.exit(1)
        else:
            print("PASS: no warnings")


if __name__ == "__main__":
    main()
