# Blind-spot follow-up — IBC authenticity remediations

**Date:** 2026-07-20  
**Role:** Agent C (critic; no code changes)  
**Prior review:** `docs/superpowers/reviews/2026-07-20-ibc-auth-blindspots.md`  
**Claimed fix surface:** commit ~`4df2318` (verified by reading tree, not by trusting the claim list)  
**Spot-checked:**  
`tests/src/ibc/{graph,normalize,predict,observe,fixtures,routes,mod}.rs`,  
`tests/bin/ibc_info.rs`,  
`tests/tests/{ibc_unit,ibc_golden,ibc_multihop_harness}.rs`,  
`tests/data/ibc/golden/**`, `tests/data/ibc/harness/**`,  
`tests/README.md` IBC sections.

---

## Executive summary

- **None of the prior Top-5 remediations is fully closed.** Four are **partially closed** with real code wins; live authenticity remains **unproven**.
- Generator wiring is the largest win: binary now imports `terp_scripts::ibc`, builds `PredictedWorld`, runs `check_invariants` fail-closed for **routing/lookup/meta**, and exposes clap `generate|validate|compare`. Local premine/`hop_count: route.len()`/`dead_clients 0..31` are gone.
- Residual P0 on generate: **`public/ibc-data` + `assetlist` + UI state are written before invariants**. Fail-closed does not roll back those artifacts; “generate failed” can still leave half-published registry JSON.
- Offline goldens correctly split `audited_mainnet` vs `synthetic` and assert `expected_lookup`, but “audited” is still a **community path-hash pin** plus a mirrored `denom_traces.json` entry — **not** a live DenomTrace with height/RPC. Compare can be green without proving mainnet.
- Harness now builds `PredictedWorld` + `check_invariants` before transfers, but scenarios still predict via local `predict_after_hops`; **no attestation file exists under `harness/runs/`** beyond a template README. Spec success “harness green” is still unexecuted.
- Prefer-direct and port filtering are solid in the lib (`has_direct_active`, transfer-only ACTIVE edges, non-transfer never preferred). Channel-side “hard Error” is **self-consistency** (alpha order, empty/identical ids, path prefix matches hop nest) — **not** dual-query proof that Terp inbound vs counterparty ids are not swapped relative to live state.
- **False-confidence risk remains high if offline green is treated as authenticity shipped.** Lib + CLI structure is now trustworthy enough to *gate* regeneration; live proof and full fail-closed publish order are not.

---

## Top-5 remediation status

| # | Remediation | Status | Evidence | Residual risk |
|---|-------------|--------|----------|---------------|
| 1 | Wire generator to lib + fail closed (O2/O3) | **partial** | `tests/bin/ibc_info.rs`: `use terp_scripts::ibc::{… check_invariants, finalize…, PredictedWorld}`; clap `Generate\|Validate\|Compare`; generate end uses `PredictedWorld::from_inputs` + `check_invariants` and `bail!` before writing lookup/routing/meta; env `IBC_EXCLUDE_CHAIN_IDS` / `IBC_EXCLUDE_CLIENT_IDS` (no `0..31`); no local `premine` / `hop_count: route.len()` | `public/ibc-data`, `assetlist.json`, UI state written **before** invariants (~496–854 vs ~856–887). Fat orchestration still in binary. `validate` builds sparse `chain_assets` (counterparties often empty) so clean validate may not exercise real preferred routes. |
| 2 | Live mainnet sample + audited vs synthetic goldens | **partial** | `known_hashes.json` has `audited_mainnet` / `synthetic` + factory slash path; `ibc_golden.rs` asserts sections + `expected_lookup` pins; `denom_traces.json` loads as explicit observe | Attestation text is still `community-known-path-hash`; no height/RPC/`audited_meta.json`. `denom_traces` **mirrors** the pin (“audited_mainnet pin mirrored as observation”) — closed loop for compare on AKT, not independent live observe. |
| 3 | Run/attest 4-chain harness; call lib PredictedWorld | **partial** | Harness imports `PredictedWorld`/`check_invariants`; builds `harness_ibc_data` and asserts invariants before scenarios 1–6; post-scenario recheck; `tests/data/ibc/harness/runs/README.md` documents attestation fields | **No actual run attestation** (only template). Live test still `#[ignore]` + `feature = "docker"`. Scenario denoms still from local `predict_after_hops`, not `world.routes` / `graph.compute_ibc_denom_for_route` for the live hop list. Dual path geometry can drift again. |
| 4 | Real LiveQuery / snapshot compare (no invert-only observe) | **partial** | `FixtureBackend` default `invert_known_hashes: false`; unit test asserts none; `SnapshotBackend`; `CompareOptions.unobserved_as_error`; CLI `compare --snapshot --strict`; wrong explicit path → `path_mismatch` Error | **No `LiveQueryBackend`.** Default unobserved = Info; non-strict compare can “clean” with almost no observations. Snapshot path exists but no harness dump → snapshot promotion pipeline. |
| 5 | Hard-error channel side + multi-channel preferred + port filter | **partial** | Graph: transfer+ACTIVE only; `route_is_preferred` demotes multi-hop if **any** direct ACTIVE; normalize: explicit preferred second channel, non-transfer never preferred; predict: `alpha_order`/`preferred_port`/`preferred_contention`/`channel_side` Errors + path-prefix nest check | Path-prefix check is **tautological** with `compute_ibc_denom_for_route` (same `hop.to_channel` nest). Does not detect registry side-swap of real channel-139/9 vs live. No dual-sided client/channel query. Ordered-channel policy still soft. |

---

## Findings (new or still open)

#### Finding F1′: Offline suite can still greenwash authenticity

- **Blind spot:** Unit/golden/lib tests + labeled “audited” fixtures read as mainnet proof; they prove hash math, section labels, and self-consistent preferred routes on synthetic triangle data.
- **Evidence:** `tests/tests/ibc_golden.rs` hashes paths against fixtures; `denom_traces.json` documents source as mirrored pin; no RPC height field. Prior F1 partially mitigated by labels, not by external truth.
- **Risk:** Operators ship `public/` after offline green while DenomTrace diverges.
- **Next check:** One live `denom_trace` for `ibc/1480B8FD…` on osmosis-1 (and AKT on morocco-1 if claimed) stored with height + node URL; CI or weekly job fails if path ≠ fixture.

#### Finding F2′: Observe is honest by default but still mostly empty

- **Blind spot:** Stopping known_hashes invert is necessary but insufficient; compare without rich `denom_traces`/snapshot rarely Errors.
- **Evidence:** `observe.rs` returns `None` unless `denom_traces` or invert flag; `CompareOptions` default `unobserved_as_error: false`; golden compare only has osmosis AKT in `denom_traces.json`.
- **Risk:** `ibc compare` without `--strict` looks clean while preferred routes are unobserved (Info only).
- **Next check:** Require `--strict` for release gate on preferred routes present in lookup; expand `denom_traces` or attach harness `SnapshotBackend` dump.

#### Finding F3′: Generate fail-closed is incomplete publish semantics

- **Blind spot:** “Fail closed on generate” was sold as refuse success; partial files still land on disk earlier.
- **Evidence:** `ibc_info.rs` writes `../public/ibc-data/*` ~496–508, `assetlist.json` ~800–804, UI state ~848, **then** invariants ~875–887, then lookup/routing/meta. Bail after half-write leaves stale+new mix possible.
- **Risk:** Consumers read new ibc-data with old lookup or vice versa; scripts that ignore exit code still get partial artifacts.
- **Next check:** Stage under temp dir; only promote all artifacts after `check_invariants` clean (or delete/mark failed on bail).

#### Finding F4′: Channel-side Error is internal consistency, not authenticity

- **Blind spot:** Spec hard invariant #1 (Terp inbound never swapped for hashing) is only approximated.
- **Evidence:** `predict.rs` Errors on alpha order, empty/identical channel ids, and `trace_path.starts_with(expected_prefix)` from reverse hops; no fixture that loads swapped 139/9 and fails against independent expected path; dual-query still non-goal.
- **Risk:** Wrong-side channel IDs produce consistent wrong hashes — green invariants, wrong mainnet denoms (original product symptom class).
- **Next check:** Golden where channel sides are deliberately swapped vs `expected_lookup` / audited pin; assert Error or path ≠ audited. Optional: live channel endpoints query.

#### Finding F5′: Prefer-direct policy is fixed in lib; product still depends on correct ACTIVE tags

- **Blind spot:** Multi-hop no longer wins over any ACTIVE direct edge (`graph.rs` `route_is_preferred` + `has_direct_active`). Preferred **which** direct channel still follows tags / first-active finalize policy.
- **Evidence:** `normalize.rs` explicit preferred transfer wins; else first transfer when client Active. CLOSED skipped in graph build. No multi-ACTIVE-unpreferred “which direct is correct for mainnet” check beyond tags.
- **Risk:** Mis-tagged preferred among two ACTIVE directs still wrong hash for AKT-class assets.
- **Next check:** Keep multi-channel golden (already in normalize tests); add mainnet-oriented pin of preferred channel id for Terp↔Akash / Terp↔Osmosis in expected_lookup + live channel query.

#### Finding F6′: Harness PredictedWorld is a pre-flight, not the transfer oracle

- **Blind spot:** Shared lib path is used for topology invariants, not for per-scenario expected denoms.
- **Evidence:** `ibc_multihop_harness.rs` ~747–775 builds world; scenarios call `run_scenario` → `predict_after_hops` (~450 region). Local helpers + `predict_via_graph` exist but scenarios use helpers.
- **Risk:** Lib route nest and harness hop assembly can diverge; dual maintenance returns.
- **Next check:** For each scenario hop list, assert `predict_after_hops` == `graph.compute_ibc_denom_for_route` for hops built from discovered link sides; optionally lookup in `world.routes`.

#### Finding F7′: Live harness authenticity still unproven (ops)

- **Blind spot:** Documentation correctly says unproven without attestation; nothing records a green run.
- **Evidence:** `tests/data/ibc/harness/runs/README.md` is instructions only; no dated run files. Test remains `#[ignore]`.
- **Risk:** Bitrot of Docker images / Hermes / TF APIs; success criteria in design still unmet.
- **Next check:** One checked-in attestation (or CI artifact) with commit SHA, image digest, scenario table.

#### Finding F8′: `validate` can pass with hollow asset coverage

- **Blind spot:** CLI validate loads counterparties with empty asset lists if not in terp assetlist natives.
- **Evidence:** `minimal_assets_from_public` inserts empty `Vec` for chain names from ibc-data; only terp natives guaranteed. `PredictedWorld` then has few routes; hop/prefer-direct rarely fire.
- **Risk:** False “validate clean” on public ibc-data that would fail once real origin assets are premine’d.
- **Next check:** Validate must load origin natives for every chain with edges (registry/assetlist/state) or Error on empty asset set for connected chains.

#### Finding F9′: Harness README relative link still wrong

- **Blind spot:** Prior F16 not fully fixed.
- **Evidence:** `tests/data/ibc/harness/README.md` links `../../tests/ibc_multihop_harness.rs` → resolves under `tests/data/tests/…`, not `tests/tests/ibc_multihop_harness.rs` (needs `../../../tests/…`).
- **Risk:** Mild ops confusion during incidents.
- **Next check:** Fix link from package-relative path; prefer `tests/tests/ibc_multihop_harness.rs` from repo docs.

#### Finding F10′: No LiveQuery; SnapshotBackend unused by harness

- **Blind spot:** Remediation 4’s core capability is still missing; Snapshot type exists without producer.
- **Evidence:** `observe.rs` has Fixture + Snapshot only; harness panics AUTH_MISMATCH without writing snapshot JSON for offline replay.
- **Risk:** Cannot re-run compare offline after a green live run; CI cannot use captured dumps.
- **Next check:** On harness success, write `tests/data/ibc/harness/runs/<sha>/snapshot.json` in SnapshotBackend shape.

#### Finding F11′: Module docs lag policy

- **Blind spot:** `tests/src/ibc/mod.rs` still says “prefer direct ACTIVE **preferred** routes”; implementation prefers-direct on **any** ACTIVE.
- **Evidence:** mod.rs lines 3–4 vs `graph.rs` `route_is_preferred`.
- **Risk:** Future agents re-introduce preferred-only demotion from doc wording.
- **Next check:** Align module/spec text with `has_direct_active` policy (product decision already encoded).

#### Finding F12′: Fat binary / dual dialect remains

- **Blind spot:** Architecture goal “thin CLI” incomplete; asset derivation and reverse-native path construction still live in binary orchestration.
- **Evidence:** `ibc_info.rs` still ~1.3k+ lines; reverse Terp natives use local `format!("transfer/{}/{}", cp_channel, native)` + lib hash; AUTH_MISMATCH panic dialect vs DiffReport still split (harness vs lib).
- **Risk:** Maintainability; subtle path bugs outside PredictedWorld.
- **Next check:** CI grep gate: no second `fn compute_ibc_denom_hash` / premine; eventually stage all outputs through one world.

#### Finding F13′: Factory slash covered offline only as synthetic hash

- **Blind spot:** Factory path is in synthetic known_hashes and pure harness string tests; live TF matrix still unrun; on-chain denom_trace base for `factory/{addr}/sub` unproven in this effort.
- **Evidence:** golden synthetic factory path; harness pure tests; live scenarios use TF but never attested.
- **Risk:** Low if hash is pure SHA of path (correct); medium if path assembly omits segments for slash dens under live nest.
- **Next check:** After one harness green, assert intermediate hop denom_trace.base includes full factory string.

---

## Regressions or incomplete wiring introduced by the fix pass

1. **Half-publish generate (new incomplete wiring):** Fail-closed was added at the end without atomic promote — can be worse than old “always write everything” if operators treat partial public/ as authoritative after a failed run.
2. **Compare default path loads golden dir for any public/:** `cmd_compare` without `--snapshot` uses package golden fixtures, not observations derived from the `--public-dir` under test — easy to misread as “this public/ matches mainnet.”
3. **Denom_traces “observation” from the same pin** creates a second closed loop that looks like F2 was fixed (invert disabled) while compare on AKT still cannot fail for wrong live path without a true external dump.
4. **Harness dual prediction retained** after PredictedWorld was added — integration looks complete in README (“same lib path as generate”) while transfer oracle remains local helpers.
5. **README still mixes eras:** top of `tests/README.md` still describes fat `ibc_core` layout and old pipeline diagrams; lower sections document lib-backed CLI. Dual narrative raises false confidence about “fully migrated.”
6. **No regression of offline correctness found** for nest order / prefer-direct / non-transfer preferred — those lib fixes look genuine and better than pre-remediation.

---

## What is now solid (keep)

- **Lib is the single premine/predict path** for routing tables when generate succeeds: `PredictedWorld` + `IBCAssetRoutingTable::premine` + `hop_count_from_trace_path`.
- **Prefer-direct:** multi-hop never preferred if any direct ACTIVE edge exists (`has_direct_active`) — matches the original multi-hop-over-direct symptom better than preferred-tag-only.
- **Graph hygiene:** transfer-only + ACTIVE-only edges; CLOSED and wasm/ICS-27 channels excluded from routing.
- **Normalize preferred policy:** explicit preferred second channel honored; non-transfer never preferred; unit coverage in `normalize.rs`.
- **Invariant codes elevated:** `alpha_order`, `preferred_port`, `preferred_contention`, hop_count, hash_binding, prefer_direct as Errors.
- **Observe invert disabled by default** + unit test; wrong explicit path fails compare — architectural honesty improved.
- **CLI surface:** `generate|validate|compare` exists and documents fail-closed intent; `ibc_generation_meta.json` records skip/exclude metadata.
- **Goldens:** audited vs synthetic labels; `expected_lookup` asserted; factory slash synthetic pin.
- **Harness honesty docs:** runs/README states unproven without attestation; PredictedWorld pre-flight is real code.
- **Dead clients 0..31 removed** in favor of env-based exclude + undecodable skip — list no longer freezes at 31.

---

## Prioritized next remediations (top 5 for orchestrator)

1. **Atomic generate + full asset coverage for validate**  
   Stage all `public/` outputs; promote only after `check_invariants` clean. Feed real origin natives into validate/compare.  
   **Why:** Completes remediation #1; stops half-publish and hollow validate.

2. **One live DenomTrace attestation + true external observe file**  
   Query osmosis (and Terp if claimed) for AKT pin; store height/RPC; do **not** copy from known_hashes into denom_traces without that metadata. Optional weekly job.  
   **Why:** Breaks remaining F1′/F2′ false confidence.

3. **Run harness once; write attestation + SnapshotBackend dump; bind scenario expects to lib graph**  
   File under `tests/data/ibc/harness/runs/`; assert scenario paths via `compute_ibc_denom_for_route` on discovered hops.  
   **Why:** Completes remediation #3; only packet-level proof in the design.

4. **Release gate: `ibc compare --strict` with snapshot or expanded traces**  
   Prefer error on unobserved preferred routes for release; wire harness snapshot into offline compare.  
   **Why:** Makes compare a real authenticity gate, not Info spam.

5. **Adversarial side-swap golden + document remaining dual-query gap**  
   Fixture with swapped channel ids vs expected_lookup must fail; explicitly mark dual-sided live verification still open.  
   **Why:** Closes the authenticity-critical half of remediation #5 that self-consistency checks cannot.

---

## Open questions

1. Should generate **delete or quarantine** already-written ibc-data/assetlist if invariants fail, or only write after success (atomic)?
2. Is product policy still “any ACTIVE direct demotes multi-hop,” or should an unpreferred direct still allow a preferred multi-hop for UX? (Code chose any ACTIVE — confirm with product.)
3. Who owns the first harness attestation and image digest pin (`local-zk` vs ghcr tag)?
4. Is `ibc compare` without `--snapshot` intended for public/ regression or only package goldens? Current wiring mixes concerns.
5. For hollow validate: should missing counterparty natives be Error or should validate load chain-registry assets automatically?
6. When is dual-sided channel verification scheduled relative to claiming “channel side correctness” in release notes?
7. Should `docs/tests/ibc_info.md` be rewritten now that CLI is lib-backed (prior F23 still open for product docs)?

---

## Appendix — verification anchors (post-fix)

| Claim | Location |
|-------|----------|
| CLI clap + lib imports | `tests/bin/ibc_info.rs` ~12–65 |
| Fail-closed after world build | `tests/bin/ibc_info.rs` ~856–887 |
| Early public writes | `tests/bin/ibc_info.rs` ~496–508, ~800–804 |
| Env exclude, not 0..31 | `tests/bin/ibc_info.rs` ~210–223 |
| Transfer-only ACTIVE graph | `tests/src/ibc/graph.rs` ~70–111 |
| Prefer-direct any ACTIVE | `tests/src/ibc/graph.rs` ~159–171 |
| Nest dest-looking-back | `tests/src/ibc/graph.rs` ~229–249 |
| Explicit preferred + non-transfer | `tests/src/ibc/normalize.rs` ~28–77, tests ~263–309 |
| Invariant Errors (alpha, port, contention, side prefix) | `tests/src/ibc/predict.rs` ~49–197 |
| No invert by default | `tests/src/ibc/observe.rs` ~91–104, tests ~458–471 |
| SnapshotBackend + CompareOptions | `tests/src/ibc/observe.rs` ~64–80, ~232–341 |
| Audited/synthetic + factory | `tests/data/ibc/golden/known_hashes.json` |
| denom_traces mirror pin | `tests/data/ibc/golden/denom_traces.json` |
| expected_lookup asserted | `tests/tests/ibc_golden.rs` ~78–103 |
| Harness PredictedWorld pre-flight | `tests/tests/ibc_multihop_harness.rs` ~747–775 |
| Harness local predict still used | `tests/tests/ibc_multihop_harness.rs` ~53–86, scenario path |
| Attestation template only | `tests/data/ibc/harness/runs/README.md` |
| README CLI honesty | `tests/README.md` ~219–227 |

---

## Verdict for orchestrator

Treat the fix pass as **library + CLI structural remediation, mostly successful**, and **authenticity proof, still incomplete**. Do not close the IBC authenticity effort until: (1) atomic fail-closed generate, (2) at least one live-attested denom with metadata, (3) one harness attestation (or explicit waiver), (4) strict compare gate on preferred routes. Offline green alone must not be the release narrative.
