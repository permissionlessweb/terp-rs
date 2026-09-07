#!/usr/bin/env python3
"""Generate thin include-shell pages from LIBRARIES.toml.

Each shell: category table (path, role, non-goals) + `{{#include}}` of the
synced mirror under `src/_include/<monorepo-path>` (from sync_includes.py),
or a short stub if the mirror is missing.

Do not paste full README bodies into shells.

Usage (from docs/plans/spectrum/book):
  python3 scripts/sync_includes.py
  python3 scripts/gen_from_registry.py
  mdbook build
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore

BOOK = Path(__file__).resolve().parents[1]
SRC = BOOK / "src"


def slug(s: str) -> str:
    s = s.lower().replace("_", "-")
    return re.sub(r"[^a-z0-9-]+", "-", s).strip("-")


def include_rel_for(path: str) -> tuple[str, bool]:
    """Return (include path relative to category page, whether mirror exists)."""
    if path.endswith(".md"):
        rel = f"../_include/{path}"
        return rel, (SRC / "_include" / path).is_file()
    if path.endswith(".sh"):
        md_path = Path(path).with_suffix(".md")
        rel = f"../_include/{md_path}"
        return rel, (SRC / "_include" / md_path).is_file()
    return "", False


def page_href(lib: dict) -> str:
    return f"{slug(lib['id'])}.md"


def shell_header(lib: dict, next_lib: dict | None) -> str:
    cat = lib["category"]
    path = lib["path"]
    role = lib.get("role", "")
    non_goals = lib.get("non_goals") or lib.get("non-goals") or ""
    test_hint = lib.get("test") or lib.get("how_to_test") or ""

    rows = [
        f"| **Category** | `{cat}` |",
        f"| **Monorepo path** | `{path}` |",
        f"| **Role** | {role} |",
    ]
    if non_goals:
        rows.append(f"| **Non-goals** | {non_goals} |")
    if test_hint:
        rows.append(f"| **How to test** | `{test_hint}` |")

    nav = ""
    if next_lib:
        nav = (
            f"\n**Next:** [{next_lib['title']}]({page_href(next_lib)})\n"
        )

    return f"""# {lib["title"]}

| | |
|--|--|
{chr(10).join(rows)}

> Thin shell only. Source of truth is the document below (synced into `src/_include/` on build). Do not edit the include tree by hand.
{nav}"""


def main() -> int:
    reg = tomllib.loads((BOOK / "LIBRARIES.toml").read_text())
    libs = reg.get("libraries", [])

    # Next link within the same category (registry order).
    next_by_id: dict[str, dict | None] = {}
    by_cat: dict[str, list[dict]] = {}
    for lib in libs:
        by_cat.setdefault(lib["category"], []).append(lib)
    for group in by_cat.values():
        for i, lib in enumerate(group):
            next_by_id[lib["id"]] = group[i + 1] if i + 1 < len(group) else None

    for lib in libs:
        cat_dir = SRC / lib["category"]
        cat_dir.mkdir(parents=True, exist_ok=True)
        page = cat_dir / f"{slug(lib['id'])}.md"
        header = shell_header(lib, next_by_id.get(lib["id"]))
        path = lib["path"]

        include_rel, has_include = include_rel_for(path)
        if has_include:
            content = header + f"\n{{{{#include {include_rel}}}}}\n"
        else:
            body = lib.get(
                "stub",
                f"_No synced document at `{path}` — see monorepo. "
                f"Run `python3 scripts/sync_includes.py`._",
            )
            content = header + f"\n{body}\n"

        page.write_text(content)
        print(f"wrote {page.relative_to(BOOK)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
