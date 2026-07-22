# mdBook design — Private Bridge Spectrum library book

| Field | Value |
|-------|--------|
| **Location** | `docs/plans/spectrum/book/` |
| **Goal** | One navigable surface for all Private Bridge / Cash App corridor docs and libraries |
| **Non-goal** | Rewriting READMEs into the book (causes drift) |
| **Sync model** | **Include-at-build** via mdBook `{{#include …}}` of monorepo source files |
| **Registry** | `LIBRARIES.toml` — categories + path + role (SSOT for SUMMARY generation) |

---

## 1. Design principles

1. **No body redundancy** — book pages are short *context shells* (what / where / when) plus include of the live README or SPEC.  
2. **Libraries by type** — categories match how engineers navigate the monorepo (product, design, pure-seams, contracts-circuit, harness-ict, observe-notify, ui, zakura, platform-wasmvm).  
3. **Circuit / manifold feel** — each library page answers: *role, monorepo path, how to test, what it does not own*.  
4. **Host floor honesty** — platform-wasmvm section documents wasmd/wasmvm alignment (bulk-memory, imports).  
5. **Sprint STATUS is thin** — point at agents/ STATUS files; do not freeze epic prose in the book forever.

---

## 2. Category map

| Category | Contains |
|----------|----------|
| **product** | USER-GUIDE, DEMO SPEC, OPERATOR |
| **design** | Freezes D1–D7, FLOW, domain SPECs, SEAM-NOTE-OUT |
| **pure-seams** | fixtures/* pure crates (cashapp, bridge_auth, dex, compose, seam_note) |
| **contracts-circuit** | headstash workspace, zk-headstash, cw-headstash |
| **harness-ict** | ict-rs, e2e README, CORRIDOR-LAB-STATUS |
| **observe-notify** | hash-market, oline play |
| **ui** | PrivateCorridor |
| **zakura** | ZAKURA-LOCAL, e2e/zakura |
| **platform-wasmvm** | cosmwasm, prepare-corridor-ict-wasm |
| **sprint-status** | final-sprint STATUS (archive later) |

---

## 3. Page template (include shell)

```markdown
# {title}

| | |
|--|--|
| **Category** | `{category}` |
| **Monorepo path** | `{path}` |
| **Role** | {role} |

> Context only. Full detail is the source document below (auto-included on `mdbook build`).

{{#include {relative_path_from_this_md}}}
```

Include paths are **relative to the markdown file** under `src/`.  
From `src/libraries/contracts-circuit/cw-headstash.md` to monorepo root is `../../../../../../` (book/src → book → spectrum → plans → docs → root). Prefer verifying with `mdbook build`.

---

## 4. Build pipeline

```bash
cd docs/plans/spectrum/book
# optional: regenerate SUMMARY stubs from LIBRARIES.toml
python3 scripts/gen_from_registry.py   # agent-owned
mdbook build                           # static site → book/
mdbook serve --open                    # local review
```

`extra-watch-dirs` in `book.toml` rebuilds when upstream READMEs change.

---

## 5. Agent tracks (this epic)

| Track | Owns |
|-------|------|
| **REGISTRY-SUMMARY** | LIBRARIES.toml completeness, SUMMARY.md, gen script |
| **SHELL-PAGES** | All include-shell markdown pages |
| **PLATFORM-WASMVM** | Host floor / wasmd-safe chapter from prepare script + STATUS-WASM |
| **META-BOOK** | `mdbook build` green; link from spectrum README |

---

## 6. Success criteria

1. `mdbook build` exits 0.  
2. Every `LIBRARIES.toml` entry with a README has an include page.  
3. spectrum `README.md` links to the book.  
4. No large paste of README body into book sources (shells only).  
5. Categories match the table above.
