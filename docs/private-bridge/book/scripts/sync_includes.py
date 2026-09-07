#!/usr/bin/env python3
"""Copy registered monorepo docs into src/_include/ for mdbook {{#include}}.

mdBook may refuse includes outside the book tree (path sandbox). This keeps
sources in-repo SSOT while making a build-time mirror under the book.

Usage (from docs/plans/spectrum/book):
  python3 scripts/sync_includes.py
  python3 scripts/gen_from_registry.py
  mdbook build
"""
from __future__ import annotations

import shutil
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore

BOOK = Path(__file__).resolve().parents[1]
ROOT = BOOK.parents[3]
INCLUDE = BOOK / "src" / "_include"


def main() -> int:
    reg = tomllib.loads((BOOK / "LIBRARIES.toml").read_text())
    INCLUDE.mkdir(parents=True, exist_ok=True)
    n = 0
    for lib in reg.get("libraries", []):
        path = lib["path"]
        src = ROOT / path
        if not src.is_file():
            print(f"skip missing {path}")
            continue
        if lib.get("readme") is False and not path.endswith(".md"):
            continue
        # only sync markdown (and treat .sh as fenced later if needed)
        if src.suffix not in {".md", ".MD"}:
            # write a tiny md wrapper for scripts
            dest = INCLUDE / Path(path).with_suffix(".md")
            dest.parent.mkdir(parents=True, exist_ok=True)
            body = src.read_text(errors="replace")
            dest.write_text(
                f"# `{path}`\n\n```bash\n{body}\n```\n"
            )
            print(f"wrap {path} -> {dest.relative_to(BOOK)}")
            n += 1
            continue
        dest = INCLUDE / path
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
        print(f"sync {path}")
        n += 1
    print(f"synced {n} files into src/_include/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
