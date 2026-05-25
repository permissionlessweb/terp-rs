#!/usr/bin/env python3
"""
Dependency Scraper for the Abstract monorepo.

Scans all Cargo.toml files, classifies every dependency as local/git/registry,
cross-references against the dependency matrix, and reports diagnostics.

Usage:
    dep-scrape.py                         # full scan → JSON + summary
    dep-scrape.py --project cw-plus       # single project
    dep-scrape.py --json-only             # JSON only (_devops/dep-scrape.json)
    dep-scrape.py --summary-only          # human summary to stdout only
    dep-scrape.py --check                 # exit 1 on warnings (CI mode)
"""

import argparse
import json
import sys
from pathlib import Path

# Add _scripts to path for dep_common import
sys.path.insert(0, str(Path(__file__).resolve().parent))

from dep_common import (
    DEVOPS_DIR,
    REPO_ROOT,
    discover_projects,
    load_active_mode,
    load_matrix,
    matrix_pkg_names,
    pkg_to_matrix,
    run_cross_project_diagnostics,
    run_diagnostics,
    scan_project,
    validate_cargo_locks,
    validate_feature_optional_deps,
    validate_matrix_packages,
    validate_matrix_paths,
    validatetomls,
)


def scan_all(matrix, project_filter=None, mode=None):
    """Scan all projects (or a filtered subset). Returns list of ProjectScanResult."""
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
    """Run diagnostics that span across projects and local clones.
    If mode is specified, only validates paths/packages relevant to that mode.
    Returns (cross_project_warnings, toml_warnings)."""
    cross_warnings = run_cross_project_diagnostics(results, matrix)
    toml_warnings = validatetomls()
    toml_warnings.extend(validate_feature_optional_deps())
    toml_warnings.extend(validate_cargo_locks())
    toml_warnings.extend(validate_matrix_paths(matrix, mode=mode))
    toml_warnings.extend(validate_matrix_packages(matrix, mode=mode))
    return cross_warnings, toml_warnings


def results_to_json(results, matrix, cross_warnings=None, toml_warnings=None):
    """Convert scan results to a JSON-serializable dict."""
    lookup = pkg_to_matrix(matrix)
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

        # Unique dep packages by source
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


def print_summary(results, matrix, cross_warnings=None, toml_warnings=None):
    """Print a human-readable summary to stdout."""
    known = matrix_pkg_names(matrix)
    total_members = 0
    total_deps = 0
    all_diags = []

    print("=== Dependency Scraper ===")
    print()

    total_loc = 0

    # Table header
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
    else:
        print("Per-project Diagnostics: (none)")
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


def main():
    parser = argparse.ArgumentParser(description="Scan and report dependency status")
    parser.add_argument("--project", help="Scan only this project (name or path)")
    parser.add_argument("--json-only", action="store_true", help="Output JSON only")
    parser.add_argument("--summary-only", action="store_true", help="Output summary only")
    parser.add_argument("--mode", choices=["stable", "local", "git", "zk_local", "zk_git"],
                        help="Only validate paths/packages relevant to this mode")
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

    if not args.summary_only:
        json_data = results_to_json(results, matrix, cross_warnings, toml_warnings)
        output_path = DEVOPS_DIR / "dep-scrape.json"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            json.dump(json_data, f, indent=2)
        if not args.json_only:
            print(f"Wrote {output_path}")
            print()

    if not args.json_only:
        print_summary(results, matrix, cross_warnings, toml_warnings)

    if args.check:
        all_diags = []
        for scan in results:
            all_diags.extend(d for d in scan.diagnostics if d.startswith("[WARN]"))
        all_diags.extend(d for d in (cross_warnings or []) if d.startswith("[WARN]"))
        all_diags.extend(d for d in (toml_warnings or []) if d.startswith("[WARN]"))
        if all_diags:
            print(f"FAIL: {len(all_diags)} warnings found")
            sys.exit(1)
        else:
            print("PASS: no warnings")


if __name__ == "__main__":
    main()
