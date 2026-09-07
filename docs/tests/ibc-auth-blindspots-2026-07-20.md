# Blind-spot review — IBC authenticity effort

**Date:** 2026-07-20  
**Role:** Agent C (critic; no code changes)  
**Plan:** `docs/superpowers/plans/2026-07-20-ibc-data-authenticity.md`  
**Spec:** `docs/superpowers/specs/2026-07-20-ibc-data-authenticity-design.md`  
**Prompts:** `docs/superpowers/prompts/ibc-auth-shared.md`, `ibc-auth-agent-a-lib.md`, `ibc-auth-agent-b-harness.md`  

**Spot-checked code:**  
`tests/src/ibc/{predict,routes,normalize,observe,graph,hash,fixtures,schema}.rs`,  
`tests/src/ibc_core.rs`, `tests/tests/{ibc_unit,ibc_golden,ibc_multihop_harness}.rs`,  
`tests/bin/ibc_info.rs`, `tests/data/ibc/golden/**`, `tests/data/ibc/harness/README.md`,  
`tests/README.md` multichain section.

---

## Executive summary

- Offline lib tests prove **self-consistency of derivation + synthetic graph invariants**, not that mainnet `public/` or live DenomTrace match prediction.
- The **fat generator binary still owns the live path** and retains pre-fix `premine` (`hop_count: route.len()`, preferred = all hops preferred) while the lib fixed those bugs — green unit/golden is **orthogonal** to what `cargo run -p terp-scripts --bin terp-ibc` emits.
- Fixture “observe” inverts `known_hashes` written by the same hash function — **compare_predict_observe cannot fail closed on path authenticity** for goldens.
- Harness reimplements path geometry locally, never builds `PredictedWorld` / `check_invariants` / `ObserveBackend`, and the live Docker test is **unrun** in agent environments — authenticity success criteria remain unproven.
- Ownership fences (A vs B vs O) deliberately deferred CLI thin-out, Cargo merge, generator wiring, and dual-sided client checks — those gaps are **structural**, not accidental.
- Hard invariant “channel side correctness” is **underspecified and under-tested**: alpha-order is a Warn; no DiffReport Error for swapped Terp inbound channel used in hashing.
- Adversarial surface (multiple transfer channels, non-transfer ports, closed clients, ordered channels, slash-heavy factory dens as registry assets) is almost entirely outside test and prompt scope.
- Ops/CI has no owner for `--ignored` runs, no golden refresh protocol with external audit, and `expected_lookup.json` is **orphaned** (not asserted by any test).
- Spec success criteria (`ibc validate` clean on regenerated `public/`, audited mainnet denom, harness green) are **mostly still Track O / unexecuted**.
- Framing actively discouraged touching the binary and Cargo.toml during A/B — correct for parallelism, but it created a **false “authenticity shipped” narrative** if green offline counts as done.

---

## Findings

### 1. Authenticity false confidence

#### Finding F1: Golden/unit suite is largely a closed loop on the same pure functions

- **Blind spot:** Passing offline tests can be read as “IBC data is authentic” when they mostly re-hash paths the suite already constructed.
- **Framing cause:** Spec success criteria mix “unit + golden pass without Docker” with real authenticity claims; Agent A mission is “prove hard invariants **without Docker**”; shared claim wording equates fixture equality with observe truth. Plan Task A5 pins “known Osmosis AKT hash” but does not require an independent live query step in the same CI gate.
- **Risk:** Operators ship or trust registry-shaped JSON because CI is green, while live DenomTrace / bank denoms still disagree.
- **Next check:** For each entry in `tests/data/ibc/golden/known_hashes.json`, run one mainnet `denom-trace` (or LCD) query and record chain-id + height + raw JSON next to the fixture; fail CI job `ibc_golden_live_sample` (manual or weekly) if path≠fixture. Confirm multi-hop synthetic vector is labeled non-mainnet in the fixture file itself.

#### Finding F2: `FixtureBackend` reconstructs observation from the prediction source

- **Blind spot:** `observe_denom_trace` walks `known_hashes` and returns the path key when the hash matches — observation is the inverse of prediction inputs (`tests/src/ibc/observe.rs` ~93–102).
- **Framing cause:** Design lists backends `Fixture | LiveQuery | Harness`, but Agent A deliverable only required “FixtureBackend (minimal)” and “compare can be thin if observe is fixture-only.” No LiveQuery implementation was in Track A or B.
- **Risk:** `compare_predict_observe` and golden test `golden_fixture_backend_compare_on_known` only prove hash↔path invertibility, not external truth; missing observations are **Info**, not Error.
- **Next check:** Inject a deliberate wrong path for a known hash into a fixture dump (not regenerable from `compute_ibc_denom_hash` alone) and assert compare emits Error; implement a `LiveQueryBackend` stub that cannot fall back to inverting known_hashes.

#### Finding F3: Live harness is treated as optional proof while docs present it as the authenticity model

- **Blind spot:** The only path that can satisfy “predict == denom_trace == balance denom” on real packets is `#[ignore]`d and was not executed in agent environments.
- **Framing cause:** Plan B2 and Agent B allow shipping “compile-clean with `#[ignore]`” and documenting blockers; shared success metrics still list harness scenarios as criteria; README says “implemented, run with --ignored” without a required green attestation.
- **Risk:** False confidence that multi-hop geometry is production-proven; silent bitrot of Docker image tags / Hermes / TF mint APIs.
- **Next check:** One recorded successful run log (commit SHA, image digests, scenario table) checked into `tests/data/ibc/harness/` or CI artifact store; until then treat authenticity claim as **unproven** in release notes.

#### Finding F4: Hard “channel side correctness” is not an Error invariant in code

- **Blind spot:** Design hard invariant #1 (sides under alpha `chain_1`/`chain_2`; Terp inbound never swapped for hashing) is not implemented as a DiffReport Error in `check_invariants` — only alpha-order **Warn** (`predict.rs` ~49–59).
- **Framing cause:** Agent A “must have tests” listed hop_count, prefer-direct, and finalize roundtrip — not an independent side-swap invariant against hashing. Spec text is strong; plan Task A3 step 4 says “implementing design hard invariants” without enumerating side-swap as a fail-closed check.
- **Risk:** Swapped channel IDs can pass schema + hop_count + hash self-binding while producing wrong mainnet denoms for consumers.
- **Next check:** Fixture pair with deliberately swapped chain_1/chain_2 channel IDs; assert `check_invariants` or a dedicated `validate_channel_sides` errors when Terp receive channel used in path is the counterparty’s id. Cross-check golden `akash-terp` channel-139/9 against a live channel query.

---

### 2. API / path semantics

#### Finding F5: Prefer-direct only considers ACTIVE **preferred** direct edges

- **Blind spot:** `has_direct_preferred_active` requires `e.preferred && status == ACTIVE` (`graph.rs` ~102–114). An ACTIVE non-preferred direct transfer channel still allows a longer preferred multi-hop to win.
- **Framing cause:** Spec wording: “If an ACTIVE **preferred** transfer channel exists…” — literal reading matches code, but the original symptom was “direct channels exist yet lookup routes multi-hop,” which may include preferred-tag failures / first-channel tagging bugs in `finalize_channels_for_ibc_entry`.
- **Risk:** First transfer channel gets preferred=true; a second, actually-used mainnet channel stays non-preferred; multi-hop via Osmosis can remain preferred for UX lookup if the direct edge is mis-tagged.
- **Next check:** Fixture with two ACTIVE transfer channels A↔B (only second preferred) + longer path A–C–B all preferred; assert preferred lookup uses the preferred direct channel id, and separately document policy when **no** preferred tag exists but ACTIVE direct does.

#### Finding F6: Re-export / multi-hop hop_count semantics differ between lib and binary; re-export prefer-direct is subtle

- **Blind spot:** Lib derives `hop_count` from path (`hop_count_from_trace_path`); binary still uses `hop_count: route.len()` (`ibc_info.rs` ~1697). For IBC assets already on a chain, full_trace nests prior path + new hops so path hops ≠ leg length. Prefer-direct in `check_invariants` uses physical first hop origin with a nested condition that can skip error if only `origin_chain` (counterparty) has a direct edge.
- **Framing cause:** Generator bugfixes are Track O only; Agent A fixed pure lib; binary duplication was left in place by design fences.
- **Risk:** Regenerated lookup `hop_count` and preferred flags still wrong for multi-hop re-exports and triangles; consumers of `public/` unchanged.
- **Next check:** Diff lib `IBCAssetRoutingTable::premine` vs binary local `premine` on the same golden triangle state; any preferred multi-hop or hop_count mismatch is a P0 generator bug. Add golden with asset that has `traces` already on intermediate chain.

#### Finding F7: Nest order is tested only in synthetic helpers, not against ICS-20 packet data

- **Blind spot:** Path construction “dest looking back” appears in lib `compute_ibc_denom_for_route` and harness `predict_after_hops`, with pure tests — no assert that intermediate voucher denoms equal prediction after hop 1 before hop 2 uses them as send denoms beyond balance of predicted hash.
- **Framing cause:** Spec emphasizes hop-by-hop transfers as proof; Agent B implements that, but offline plane cannot see packet events; design non-goal excludes PFM single-tx multi-hop (fine) without requiring intermediate denom equality fixtures offline.
- **Risk:** Off-by-one channel (send vs recv) only fails when Docker runs; pure tests still pass with wrong side convention if both sides use same id (common in local Hermes).
- **Next check:** In harness, force asymmetric channel ids (already true if Hermes assigns differently) and fail if send channel is ever used in looking-back path; offline unit test with asymmetric ids only (already partially in triangle goldens — extend to 3-hop line fixture).

#### Finding F8: Non-transfer ports get `preferred: true` in finalize

- **Blind spot:** `finalize_channels_for_ibc_entry` sets preferred true for non-transfer ports (`normalize.rs` ~53–57), and graph build does not filter on `port_id` when adding edges (`graph.rs` ~79–93).
- **Framing cause:** Ported from binary behavior; neither prompt nor plan called out non-transfer ports as adversarial cases; schema allows any port_id string.
- **Risk:** ICS-27 / wasm / fee middleware channels can enter routing graph and produce nonsense preferred routes.
- **Next check:** Synthetic ibc_data with transfer + wasm channel; assert graph edges and preferred lookup ignore non-`transfer` ports (policy decision + test).

---

### 3. Harness realism

#### Finding F9: Harness does not use the authenticity library end-to-end

- **Blind spot:** Live test uses local `predict_trace_path` / `predict_after_hops` + `compute_ibc_denom_hash` only — not `PredictedWorld`, `check_invariants`, `compare_predict_observe`, or harness `IbcDataSet` recorded in registry schema. Spec post-scenario “full PredictedWorld premine with max_hops=3 … assert preferred-route invariants” is missing (`ibc_multihop_harness.rs` ends after scenarios 1–6 ~780).
- **Framing cause:** Agent B forbidden to edit `tests/src/ibc/**`; prompt explicitly allows “local prediction helper if lib not ready”; plan B depends only on “public API names” and hash function.
- **Risk:** Lib and harness can diverge on nest order / preferred semantics while both look green in isolation; parallel ownership permanently splits the authenticity model.
- **Next check:** After channels discovered, build in-memory ibc_data JSON → `PredictedWorld::from_inputs` → `check_invariants` must be clean; scenario expected denoms must equal `graph.compute_ibc_denom_for_route` for the same hops.

#### Finding F10: Timing, gas, image, and determinism are soft

- **Blind spot:** Fixed `RELAY_WAIT_BLOCKS = 12`, `gas_prices: "0uterp"`, default image `terpnetwork/terp-core:local-zk`, README also mentions `ghcr.io/terpnetwork/terp-core:v5.2.0-zk-localterp` — no image digest pin, no retry/backoff on packet timeout, no flake budget.
- **Framing cause:** Agent B success = compilable ignored test + docs honesty; non-goals and “prefer maximal real code” do not require hermetic image pins or soak tests.
- **Risk:** Intermittent AUTH_MISMATCH on slow CI; “works on my machine” with local-zk; unreproducible authenticity proofs.
- **Next check:** Document required image digest; one soak of scenarios 1–6 × N; measure failure modes (zero balance vs wrong hash vs missing denom_trace). Capture Hermes path create channel ids across runs for determinism.

#### Finding F11: TF matrix undersampled relative to documentation

- **Blind spot:** 16 denoms created; scenarios use 6 origins only; unused ta3/tb1… never transfer; no reverse of same denom after multi-hop (unwind), no same-channel re-entry.
- **Framing cause:** Spec scenarios 1–6 are the minimum and were treated as the maximum; Agent B prompt stops at those six.
- **Risk:** Factory denom slash parsing only covered in pure string tests; on-chain TF + multi-hop edge cases untested.
- **Next check:** At least one scenario with base `factory/{addr}/sub` verified in denom_trace base_denom field equality; optional unwind hop back toward origin.

---

### 4. Generator divergence

#### Finding F12: Binary is still the product path and duplicates pre-fix logic

- **Blind spot:** `tests/bin/ibc_info.rs` still defines local `finalize_channels_for_ibc_entry`, `compute_ibc_denom_hash`, full graph/premine/routing (~1.4k–1.8k+), hardcoded `dead_clients` 0..31, no clap `validate|compare`, no auto `check_invariants` at end of generate, no `ibc_generation_meta.json`.
- **Framing cause:** Explicit parallel ownership: A and B **must not** edit `tests/bin/**`; Track O owns thin CLI and generator bugfixes; Agent A “except pure-function fixes in the lib that generate will later call” assumed a later call that has not happened. Critic prompt facts state this divergence.
- **Risk:** Highest — **lib green, mainnet artifacts still wrong** (prefer multi-hop, hop_count lies, dead client list). Authenticity effort can be declared “done” while user-facing binary is unchanged.
- **Next check:** `rg`/diff for `hop_count: route.len()` and prefer-without-`route_is_preferred` in binary; run generate against fixtures and feed output into lib `check_invariants` — expect errors until O3 lands. Measure: binary must call `terp_scripts::ibc::{..., check_invariants}` with zero local premine copy.

#### Finding F13: Binary `#[cfg(test)]` still holds large validator/tests

- **Blind spot:** Spec migration said remove large in-binary test module once integration tests own coverage; binary still embeds validate_* and extensive tests.
- **Framing cause:** Out of scope for A/B; O2/O4 incomplete.
- **Risk:** Two schemas/validators drift; “which validate is real?” confusion.
- **Next check:** Count duplicate symbols between `bin/ibc_info.rs` and `src/ibc/*`; require single implementation before next generate release.

---

### 5. Parallel ownership costs

#### Finding F14: File fences blocked the authenticity loop from closing

- **Blind spot:** predict→observe→diff is split: A owns predict/fixture observe; B owns live transfers; O owns wiring. Nothing forces one DiffReport type through generate and harness.
- **Framing cause:** Shared prompt ownership table; plan “Do not let A and B both edit bin or Cargo.toml”; Agent B cannot improve lib API for harness dumps.
- **Risk:** Integration debt accumulates; orchestrator is a single-point bottleneck; partial delivery looks complete per-track.
- **Next check:** Orchestrator checklist: (1) binary uses lib, (2) harness uses lib PredictedWorld, (3) one golden dump from harness promoted offline — all three checked or marked blocked with owner.

#### Finding F15: `expected_lookup.json` is dead weight; Cargo discovery unowned mid-flight

- **Blind spot:** `tests/data/ibc/golden/expected_lookup.json` is not referenced by any test (grep finds no loaders). Package auto-discovery of `tests/tests/*.rs` worked, so Cargo.toml silence is fine, but orphan fixtures rot.
- **Framing cause:** Agent A marked expected_lookup optional; no task required asserting it; golden refresh protocol is manual prose only.
- **Risk:** Maintainers update wrong file; pins silently diverge from assert path in `ibc_golden.rs` (which hardcodes AKT expectations in Rust).
- **Next check:** Either delete `expected_lookup.json` or add a golden test that loads it and diffs against `world.lookup` / preferred routes.

#### Finding F16: Harness README path relative link may confuse readers

- **Blind spot:** `tests/data/ibc/harness/README.md` links to `../../../tests/ibc_multihop_harness.rs` (odd relative from `data/ibc/harness`); actual file is `tests/tests/ibc_multihop_harness.rs`.
- **Framing cause:** Track B docs accuracy focused on `tests/README.md` multichain section more than relative link math under data/.
- **Risk:** Mild ops confusion; wrong file opened during incident response.
- **Next check:** Resolve link from package root; fix in O4 doc pass.

---

### 6. Missing adversarial cases

#### Finding F17: Multiple ACTIVE transfer channels / preferred contention

- **Blind spot:** No test where two transfer channels exist and preferred flips or contention occurs; `finalize` marks **first** transfer preferred only.
- **Framing cause:** Design open follow-up “preferred-channel contention tests”; prompts did not require it; non-goal mesh topology reduced exposure but pairwise multi-channel is still common on mainnet.
- **Risk:** Wrong channel preferred → wrong ibc hash for AKT/ATONE-class assets (core symptom of the effort).
- **Next check:** Golden ibc_data with channel-A and channel-B both transfer ACTIVE; preferred on second; assert map + paths use second.

#### Finding F18: Closed / expired clients and OPEN channel filters

- **Blind spot:** Binary still skips clients via hard-coded 0..31; lib never sees client status — only channel tags.STATUS string. No test for CLOSED channel still present in JSON.
- **Framing cause:** Spec requires capability-based filtering in generator bugfixes (O3); dual-sided client verification is non-goal/follow-up; Agent A invariants assume ACTIVE tags already correct.
- **Risk:** Stale channels in state produce phantom routes; dead_clients list goes stale as new dead clients appear above 31.
- **Next check:** Fixture channel status CLOSED must not appear in `find_routes`; generate dry-run count of clients skipped with reason codes in meta file.

#### Finding F19: Factory denoms with slashes and non-ICS20 ports

- **Blind spot:** Pure harness tests use `factory/terp1abc/ta0` in path strings only; graph/premine asset bases are simple `uakt`/`uterp`. Ordered vs unordered channels not differentiated for authenticity (ordering only schema/stringified).
- **Framing cause:** Spec TF matrix is for harness live path; offline adversarial list never appeared in Agent A unit list; “no over-engineering” discourages extra cases.
- **Risk:** Path parsing / hop_count that counts `transfer/` is OK for factory bases, but registry assets with unusual base denoms or `ibc-cw20` may break schema or nest rules unnoticed.
- **Next check:** Unit: hop_count and hash for `transfer/channel-0/factory/addr/sub.with.dots`; schema fixture with non-transfer port rejected from routing (not necessarily schema).

#### Finding F20: Unordered-only mental model

- **Blind spot:** All goldens and harness assume unordered ICS-20; ordered channels accepted by schema after string convert but never exercised.
- **Framing cause:** Mainnet transfer is unordered; prompts copy that; not a stated non-goal but effectively ignored.
- **Risk:** Low for transfer authenticity; medium if ordered ports ever enter ibc_data and get preferred true (see F8).
- **Next check:** Confirm product policy: non-unordered transfer channels warn or error in `check_invariants`.

---

### 7. Ops / CI

#### Finding F21: No CI owner for `--ignored` authenticity

- **Blind spot:** Default CI will stay green forever while live proof never runs; no scheduled job, no badge, no failure page.
- **Framing cause:** Explicit “CI default must not run this test”; success criteria still require harness green — contradiction resolved by human memory, not automation.
- **Risk:** Authenticity claim decays; Docker/Hermes/Terp image changes break harness unnoticed for months.
- **Next check:** Define job `ibc-multihop-nightly` with image pin + artifact logs; if too expensive, require weekly manual attestation filed under `tests/data/ibc/harness/runs/`.

#### Finding F22: Golden refresh attack / silent overwrite

- **Blind spot:** Spec says “never silent overwrite of goldens” but there is no tooling guard (no `UPDATE_GOLDEN` flag, no hash of audit metadata). Goldens can be regenerated from predict alone (including known_hashes multi-hop vector).
- **Framing cause:** Manual process in design; no Track task for refresh CLI; Agent A could write goldens from the same functions under test.
- **Risk:** A “fix” that changes hash algorithm silently rewrites expected vectors and keeps tests green.
- **Next check:** Split `known_hashes.json` into `audited_mainnet` (require external signature/height) vs `synthetic`; refresh script must refuse to rewrite audited keys without `--i-audited-live`.

#### Finding F23: README honesty improved but still oversells model completeness

- **Blind spot:** `tests/README.md` correctly points at harness and offline coverage, but product docs/`docs/tests/ibc_info.md` rewrite is Track O incomplete — dual docs may still claim old paths.
- **Framing cause:** Agent B only allowed harness-only README edits; O4 full doc rewrite pending.
- **Risk:** External readers follow obsolete `ibc_info` docs.
- **Next check:** Grep repo for `test_multichain_ibc_info_routing` and aspirational multichain claims outside tests/README.

---

### 8. Spec/plan contradictions or underspec

#### Finding F24: Success criteria vs non-goals vs track boundaries

- **Blind spot:** Success includes “`ibc validate` clean on regenerated `public/`” and audited mainnet denom, but validate CLI and generate fixes are Track O, while A/B “done” can be celebrated earlier. Dual-sided verification non-goal undercuts channel side hard invariant language.
- **Framing cause:** Plan parallelization map defers the only user-visible authenticity gate; design hard invariants list side correctness as Error while open follow-ups defer dual-query.
- **Risk:** Metric gaming: Track A+B complete ⇒ effort “implemented” while mainnet outputs unchanged.
- **Next check:** Redefine “effort done” as O2+O3 green + one live harness attestation; keep A/B as “library + scaffold landed.”

#### Finding F25: Observe trait surface too thin for harness dumps

- **Blind spot:** `ObserveBackend` only has denom_trace + channels; no balance observation API — design Diff examples include “missing post-transfer balance” as Error severity, only enforceable in harness panic paths, not DiffReport.
- **Framing cause:** Agent A minimal trait; harness not required to use DiffReport.
- **Risk:** Two reporting dialects (AUTH_MISMATCH panic vs DiffItem); tooling cannot unify offline/live reports.
- **Next check:** Extend trait or separate `ObserveBalances`; harness converts mismatches into DiffReport for logging.

#### Finding F26: “One authenticity model” vs three code paths

- **Blind spot:** Spec architecture diagram shows one pipeline; shipped reality: lib predict, binary premine, harness local predict.
- **Framing cause:** Migration plan assumed fold-then-wire; parallel agents inverted to extract-then-defer-wire.
- **Risk:** Maintainability and correctness (see F12/F9).
- **Next check:** Architecture conformance test: binary and harness must not define their own `compute_ibc_denom_hash` / premine (compile-time or CI grep gate).

---

### 9. What the prompts actively discouraged that we may need

#### Finding F27: Discouraged work that is now on the critical path

| Discouraged by framing | Why we may need it now |
|------------------------|------------------------|
| Editing `tests/bin/ibc_info.rs` (A/B) | Only path that writes `public/`; still wrong preferred/hop_count |
| Editing `Cargo.toml` | Features/docker test wiring, future `[[bin]]` clap features |
| LiveQuery backend (A “minimal fixture”) | Without it offline “observe” is fake |
| Dual-sided client/connection checks (non-goal) | Side-swap authenticity for mainnet channels |
| Mesh / preferred contention (follow-up) | Real Osmosis/Akash multi-channel world |
| PFM single-tx multi-hop (non-goal) | OK to defer, but document that hop-by-hop ≠ user wallet multi-hop UX |
| Making harness non-ignored | Correct for default CI, but no alternate attestation path specified |
| “No over-engineering” / surgical scope | Skipped balance-in-DiffReport, audited golden metadata, image digests |

- **Next check:** Orchestrator backlog explicitly re-opens each row with priority (see remediations).

---

### 10. Top 5 prioritized remediations

See section below (same ordering).

---

## Prioritized remediations (top 5)

1. **Wire generator to lib + fail closed (Track O2/O3)**  
   Delete or thin binary-local graph/premine/hash/finalize; call `terp_scripts::ibc`; run `check_invariants` (+ schema) at end of generate; refuse success on Error. Replace dead_clients 0..31 with capability filters.  
   **Why first:** User-facing authenticity is still the binary; offline green does not fix mainnet JSON.

2. **Prove one live mainnet sample + separate audited vs synthetic goldens**  
   Query live DenomTrace for AKT (and ideally ATONE) on Osmosis/Terp; store height/RPC in fixture metadata; label synthetic multi-hop vectors so they never count as audited.  
   **Why:** Breaks F1/F2/F22 false confidence.

3. **Run and attest the 4-chain harness once; then call lib PredictedWorld from harness**  
   Record image digests + logs; add post-scenario `PredictedWorld` premine invariants; remove parallel path geometry.  
   **Why:** Only real packet-level proof; closes F3/F9/F21.

4. **Implement real LiveQuery (or snapshot dumps from harness) for compare**  
   FixtureBackend must not invert known_hashes as sole observation; unobserved preferred mainnet routes should be configurable Error under `--strict`.  
   **Why:** Makes `compare` subcommand meaningful for success criterion “ibc validate/compare clean.”

5. **Hard-error channel side + multi-channel preferred policy + port_id filter**  
   Tests for swapped sides, two transfer channels, non-transfer ports excluded from graph.  
   **Why:** Directly targets original side-swap / wrong-route class of bugs underspecified in A tests (F4/F5/F8/F17).

---

## Valid strengths of the current framing

- Clear **predict → observe → diff** vocabulary and severity table give a durable architecture once wired.
- Parallel A/B ownership avoided merge chaos on the fat binary; lib extraction is real and re-export via `ibc_core` preserves compile compatibility.
- Concrete hard invariants (hop_count honesty, prefer-direct on synthetic triangle) are unit-tested in the lib and would catch the original derivation class of bugs **if generate used them**.
- Agent B fixed the prior README lie about a non-existent multichain test and documented honest `#[ignore]` status.
- Line topology + hop-by-hop (not PFM-first) is the right minimal live proof shape.
- Golden AKT Osmosis vector `ibc/1480B8FD…` is a community-known path hash (good pin candidate once live-attested).
- Critic/review task (O5) was planned rather than left implicit — framing can absorb this report into backlog.

---

## Open questions for the team

1. When is Track O scheduled relative to any consumer release that still runs `cargo run -p terp-scripts --bin terp-ibc`?
2. Is preferred policy “prefer any ACTIVE direct transfer” or only “ACTIVE + preferred tag”? Product answer drives F5.
3. Who owns weekly/nightly `--ignored` harness runs and image rebuilds (`local-zk` vs ghcr tag)?
4. Should `public/` regeneration be blocked in CI until `check_invariants` is clean (policy), or only documented?
5. Do we need dual-query of counterparty channel state before claiming side correctness as a hard Error?
6. Is `expected_lookup.json` intentional future work or should it be deleted to avoid dual sources of truth with `ibc_golden.rs` literals?
7. For factory and multi-hop assets in registry output: is Terp-only generate scope enough, or must non-Terp pair files (akash-osmosis) be first-class in generate too?
8. What is the bar for “audited mainnet denom” — one chain height snapshot, or continuous monitoring?

---

## Appendix — evidence anchors

| Claim | Location |
|-------|----------|
| Lib prefer-direct + hop_count from path | `tests/src/ibc/graph.rs` `route_is_preferred`; `routes.rs` premine |
| Binary hop_count = route.len(), preferred = all hops preferred | `tests/bin/ibc_info.rs` ~1685–1698 |
| Fixture observe inverts known_hashes | `tests/src/ibc/observe.rs` ~93–102 |
| check_invariants lacks side-swap Error | `tests/src/ibc/predict.rs` ~39–160 |
| Non-transfer preferred true | `tests/src/ibc/normalize.rs` ~53–57 |
| Harness local predict, no PredictedWorld | `tests/tests/ibc_multihop_harness.rs` ~49–77, ~720–781 |
| dead_clients 0..31 | `tests/bin/ibc_info.rs` ~110–115 |
| expected_lookup unused | `tests/data/ibc/golden/expected_lookup.json`; no test loaders |
| ibc_core is re-export only | `tests/src/ibc_core.rs` |
| Binary barely imports lib | `use terp_scripts::ibc_core::TerpChannelInfo` only at top of `ibc_info.rs` |
