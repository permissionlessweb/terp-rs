#!/usr/bin/env python3
"""
Run `cargo check` on all discovered project workspaces.
Streams output to terminal in real-time AND writes a TOML results file.

Usage:
    cargo-check-all.py                  # run all, stream + write _devops/cargo-check.toml
    cargo-check-all.py --project foo    # single project
    cargo-check-all.py --stdout         # print TOML to stdout instead of file
"""

import argparse
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from dep_common import REPO_ROOT, discover_projects

DEVOPS_DIR = REPO_ROOT / "_devops"

# ── Tee helper ─────────────────────────────────────────────────────────────

def _tee(line, captured):
    """Print a line to stdout and append to captured list."""
    print(line)
    captured.append(line)


# ── Cargo check (streaming) ───────────────────────────────────────────────

def cargo_check(project_dir, captured, timeout=600):
    """Run cargo check, stream output line-by-line, return (ok, duration, stderr_text)."""
    start = time.time()
    try:
        proc = subprocess.Popen(
            ["cargo", "chec", "--workspace"],
            cwd=str(project_dir),
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,  # merge stderr into stdout for streaming
            text=True,
        )
        output_lines = []
        for line in proc.stdout:
            line = line.rstrip("\n")
            _tee(line, captured)
            output_lines.append(line)

        proc.wait(timeout=timeout)
        dur = round(time.time() - start, 1)
        output = "\n".join(output_lines)
        return proc.returncode == 0, dur, output

    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait()
        dur = round(time.time() - start, 1)
        msg = f"TIMEOUT after {timeout}s"
        _tee(msg, captured)
        return False, dur, msg
    except Exception as e:
        dur = round(time.time() - start, 1)
        msg = str(e)
        _tee(msg, captured)
        return False, dur, msg


# ── TOML helpers ───────────────────────────────────────────────────────────

def escape_toml_string(s):
    """Escape a string for TOML multi-line basic strings."""
    s = s.replace("\\", "\\\\").replace('"', '\\"')
    return s


def _toml_key(name):
    """Sanitize a project name for use as a TOML key."""
    return name.replace("/", "-").replace(".", "-")


def _extract_errors(output):
    """Extract the most relevant error lines from cargo check output."""
    lines = output.split("\n")
    error_lines = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("error") or stripped.startswith("warning["):
            error_lines.append(line)
        elif "could not compile" in stripped:
            error_lines.append(line)
    if error_lines:
        return "\n".join(error_lines[-30:])
    return "\n".join(lines[-20:])


# ── Main ───────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Run cargo check on all workspaces")
    parser.add_argument("--project", help="Check only this project (name or path)")
    parser.add_argument("--stdout", action="store_true", help="Print TOML to stdout")
    parser.add_argument("--timeout", type=int, default=600,
                        help="Per-project timeout in seconds (default: 600)")
    args = parser.parse_args()

    projects = discover_projects()

    if args.project:
        projects = [
            p for p in projects
            if args.project in (p.name, p.rel_path)
        ]
        if not projects:
            print(f"ERROR: project '{args.project}' not found")
            sys.exit(1)

    total = len(projects)
    passed = failed = skipped = 0
    results = []
    captured = []  # all lines printed to terminal, for the TOML file

    _tee(f"Running cargo check on {total} projects...", captured)
    _tee("", captured)

    for i, proj in enumerate(projects, 1):
        proj_dir = REPO_ROOT / proj.rel_path
        cargo_toml = proj_dir / "Cargo.toml"

        if not cargo_toml.exists():
            _tee(f"  [{i}/{total}] SKIP {proj.name} (no Cargo.toml)", captured)
            skipped += 1
            results.append((proj, "skip", 0.0, "no Cargo.toml"))
            continue

        _tee(f"{'─' * 60}", captured)
        _tee(f"  [{i}/{total}] {proj.name}", captured)
        _tee(f"{'─' * 60}", captured)

        ok, dur, output = cargo_check(proj_dir, captured, timeout=args.timeout)

        if ok:
            _tee(f"  → PASS ({dur}s)", captured)
            passed += 1
            results.append((proj, "pass", dur, output))
        else:
            _tee(f"  → FAIL ({dur}s)", captured)
            failed += 1
            results.append((proj, "fail", dur, output))

        _tee("", captured)

    # ── Summary ────────────────────────────────────────────────────────────
    summary = f"Summary: {passed} pass, {failed} fail, {skipped} skip / {total} total"
    _tee(f"{'═' * 60}", captured)
    _tee(summary, captured)
    _tee(f"{'═' * 60}", captured)

    # ── TOML output ────────────────────────────────────────────────────────
    toml_lines = [
        "# Cargo check results",
        f"# Generated: {datetime.now().isoformat(timespec='seconds')}",
        f"# {summary}",
        "",
        "[summary]",
        f"total = {total}",
        f"pass = {passed}",
        f"fail = {failed}",
        f"skip = {skipped}",
        f'timestamp = "{datetime.now().isoformat(timespec="seconds")}"',
        "",
    ]

    for proj, status, dur, output in results:
        toml_lines.append(f"[projects.{_toml_key(proj.name)}]")
        toml_lines.append(f'path = "{proj.rel_path}"')
        toml_lines.append(f'status = "{status}"')
        toml_lines.append(f"duration = {dur}")
        if status == "fail" and output:
            err_lines = _extract_errors(output)
            escaped = escape_toml_string(err_lines)
            toml_lines.append(f'errors = """\n{escaped}"""')
        toml_lines.append("")

    toml_str = "\n".join(toml_lines)

    if args.stdout:
        print(toml_str)
    else:
        out_path = DEVOPS_DIR / "cargo-check.toml"
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(toml_str)
        print(f"\nWrote {out_path}")

    sys.exit(1 if failed > 0 else 0)


if __name__ == "__main__":
    main()
