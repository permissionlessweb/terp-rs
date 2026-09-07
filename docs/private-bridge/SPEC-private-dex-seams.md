# SPEC: Private Multi-Pair DEX Action Seams

| Field | Value |
|-------|-------|
| **Status** | Draft — Domain D (Private DEX seams) |
| **Audience** | Domain E (end-to-end flow wiring), circuit authors, CosmWasm integrators |
| **SSOT alignment** | `crates/tacit/AMM.md`, `crates/tacit/spec/SPEC-CONFIDENTIAL-POOL.md` |
| **Oracle surface** | `x/hashmerchant` (VE + modular oracle attestations; **bounds only**) |
| **Program spine** | private-shielded-dex-bridge PLAN § private DEX + trust |
| **Scope** | Interfaces, state transitions, rejection rules, test scenarios — **not** full DEX contracts |

---

## 0. Intent

Define **deterministic action seams** for a Terp private multi-pair DEX that:

1. Spends **multi-asset notes** into a **virtual AMM** with **public reserves** (Tacit spirit).
2. Mints **private outputs** into the **same anonymity set** as Headstash claims and bridge mints.
3. Optionally consumes **hashmerchant VE / custody-oracle mids** only as **price bounds** — never as balance mints.

Solvency lives in note conservation + proven \(\Delta\) applied to reserves. Pricing honesty may use oracles; **value creation never does**.

```text
  PRIVATE                              TRANSPARENT
  ────────                             ───────────
  note amounts                         R_A, R_B (per pool)
  who claimed / who swapped            fee params γ, γ_den
  note lineage                         pool mid after settle
  encrypted memo / airdrop tag         oracle mid + staleness (if used)
  nullifier preimages                  nullifiers (spent markers)
```

---

## 1. Note I/O model for swaps

### 1.1 Multi-asset note (minimum fields)

Aligned with Tacit confidential-pool note language (MASP-shaped on Terp). Exact crypto suite (Pedersen on which curve, leaf hash) is out of this SPEC; **field roles are normative for seams**.

| Field | Type (logical) | Visibility | Role |
|-------|----------------|------------|------|
| `asset_id` | domain-separated bytes / u128 | private in note; **bound in circuit** | Which asset; must match pool leg |
| `value` / `v` | non-negative integer (base units) | private (commitment) | Amount of `asset_id` |
| `cm` | note commitment | public as Merkle leaf | Tree membership |
| `nullifier` material | preimage → public `ν` on spend | `ν` public on spend | Double-spend prevention |
| `owner` / diversifier | spend auth keys | private | Authorization |
| `rcm` / blinding | commitment randomness | private | Binding / opening |
| `memo` (optional) | encrypted bytes | ciphertext public | Airdrop tag, recovery, UX |
| `domain` (optional) | chain / pool instance id | public or fixed param | Domain separation across venues |

**Invariants (note plane):**

- One spend reveals exactly one nullifier \(\nu\); \(\nu\) enters a global (or set-scoped) nullifier set and cannot be reused.
- Conservation is over **value commitments by `asset_id`**, not over cleartext amounts on chain.
- All demo / product assets that participate in claim → swap → (optional) unshield share **one** commitment tree (or a manifold-linked set treated as one anonymity set for unlinkability claims).

### 1.2 Swap action I/O (minimum)

A private swap is one **action** (or action bundle) with:

**Private inputs (spent notes):**

| Field | Description |
|-------|-------------|
| `notes_in[]` | One or more notes of `asset_in` (same `asset_id` for simple v1 single-leg; multi-note merge allowed) |
| `paths[]` | Merkle authentication paths to current (or allowed lagged) root |
| `nullifiers[]` | Derived public \(\nu_i\) for each spent note |

**Private outputs (created notes):**

| Field | Description |
|-------|-------------|
| `note_out` | Note of `asset_out` with value \(\Delta_{\text{out}}\) (or \(\ge \textit{min_out}\), with residual policy below) |
| `note_change` (optional) | Residual of `asset_in` if inputs exceed \(\Delta_{\text{in}}\) |
| `cm_out`, `cm_change` | Commitments appended to the **same** tree |

**Public statement (action payload / public inputs):**

| Field | Description |
|-------|-------------|
| `pool_id` | Pair key (see §5) |
| `asset_in`, `asset_out` | Must equal pool legs (oriented) |
| `root` | Commitment root used for membership |
| `nullifiers[]` | Spent markers |
| `cm_out[]` | New leaves (commitments only) |
| `Δ_reserves` | Public reserve updates \((\Delta R_{\text{in}}, \Delta R_{\text{out}})\) consistent with curve |
| `min_out` | Slippage floor (cleartext u64; Tacit `T_SWAP_BATCH` / intent spirit) |
| `fee_params_id` or \(\gamma, \gamma_{\text{den}}\) | Bound to pool params |
| `oracle_bound` (optional) | See §3 — mid snapshot id / height, `max_slippage_bps` |
| `proof` | ZK / BP+ proof of spend + conservation + curve inequality |

**v1 amount posture (honest privacy table):**

| Revealed | Hidden |
|----------|--------|
| Pair / pool_id; reserve deltas (approx trade size) | Who traded |
| New public mid after trade | Note lineage (claim ↔ swap ↔ exit) |
| Nullifiers (unlinkable markers) | Cleartext per-note values (if commitments used) |

> **Note:** Tacit `T_SWAP_VAR` makes \(\Delta\) cleartext on Bitcoin; Terp v1 may choose **public \(\Delta_{\text{in}}/\Delta_{\text{out}}\)** for simpler circuits (matches transparent reserve math) or **hidden amounts with public net \(\Delta\)** (closer to batch privacy). Either way, **reserves and \(\Delta R\) are public**. Domain E MUST document which posture the demo uses; seams below support both if the public statement carries \(\Delta R\).

### 1.3 Conservation rules (normative)

For a single-pool A→B swap with input amount \(\Delta_{\text{in}} > 0\):

1. \(\sum v(\text{notes_in}) = \Delta_{\text{in}} + v(\text{change})\) (or equality under commitment algebra).
2. \(v(\text{note_out}) = \Delta_{\text{out}}\) (no silent mint of `asset_out` beyond curve output).
3. \(\Delta_{\text{out}}\) equals the AMM formula on **pre-state** public reserves (§2), or is \(\ge \textit{min_out}\) only when the **public** applied \(\Delta R\) matches the same formula (prefer **exact** equality for determinism).
4. **No oracle term appears in (1)–(3).** Oracle only gates acceptance via bounds (§3).

### 1.4 Determinism

Given:

- fixed pool state \((R_A, R_B, \gamma, \gamma_{\text{den}})\),
- fixed public inputs and proof,
- fixed nullifier set membership,

the transition either **rejects** or yields a **unique** post-state:

\[
R_A' = R_A + \Delta_{\text{in}},\quad
R_B' = R_B - \Delta_{\text{out}}
\]

(with fee embedding as in §2; direction flips for B→A).

No wall-clock, no non-deterministic oracle sampling inside the state transition: oracle data MUST be a **bound snapshot** already finalized on-chain (hashmerchant root / price bundle height).

---

## 2. Public AMM state + swap formula

### 2.1 Pool state (public, virtual)

Mirrors Tacit virtual-pool posture: **no custody UTXO / escrow of note value inside the pool**. Reserves are counters updated only by valid actions.

```text
Pool {
  pool_id:        PoolId,           // hash(asset_A, asset_B, fee_params) or hub-edge id
  asset_A:        AssetId,          // canonical order: e.g. min(bytes)
  asset_B:        AssetId,
  R_A:            u128,             // public reserve of asset_A
  R_B:            u128,             // public reserve of asset_B
  gamma:          u64,              // fee numerator   (γ)
  gamma_den:      u64,              // fee denominator (γ_den)
  S:              u128,             // LP share supply (optional v1; 0 if POL-only)
  fee_protocol_bps: u16,            // optional protocol cut of fees
  status:         Active | Paused,
}
```

**Invariants:**

- \(R_A > 0\), \(R_B > 0\) while Active (after init).
- Constant-product **spirit**: post-swap \(R_A' \cdot R_B' \ge R_A \cdot R_B\) after fees (Uniswap V2 / Tacit fee embedding).
- Only proven swaps / LP ops mutate \((R_A, R_B, S)\).

### 2.2 Swap formula (reference — Tacit `T_SWAP_VAR` spirit)

For a trade that **pays asset A, receives asset B**, with pre-trade reserves \(R_A, R_B\):

\[
\Delta_{\text{out}}
  = \left\lfloor
      \frac{R_B \cdot \gamma \cdot \Delta_{\text{in}}}
           {R_A \cdot \gamma_{\text{den}} + \gamma \cdot \Delta_{\text{in}}}
    \right\rfloor
\]

Reference: `crates/tacit/AMM.md` — spot curve

\[
\Delta_{\text{out}} = R_B \cdot \gamma \cdot \Delta / (R_A \cdot \gamma_{\text{den}} + \gamma \cdot \Delta).
\]

**Acceptance:**

\[
\Delta_{\text{out}} \ge \textit{min_out}
\]

else reject (`ErrMinOut`).

**State update (A→B):**

\[
R_A \leftarrow R_A + \Delta_{\text{in}},\qquad
R_B \leftarrow R_B - \Delta_{\text{out}}.
\]

Require \(R_B' > 0\) (and optionally a min-liquidity residual).

B→A is symmetric (swap labels). Multi-hop is **out of v1 seams** except as sequential independent actions (no atomic route opcode required for demo).

### 2.3 Fee representation

| Symbol | Meaning | Typical demo |
|--------|---------|--------------|
| \(\gamma / \gamma_{\text{den}}\) | Input-side fee factor (Tacit) | e.g. \(997/1000\) for 30 bps |
| Protocol fee | Optional skim from LP growth | v1 MAY be 0 |

Fees remain in reserves (LP-owned virtual inventory), not oracle-funded.

### 2.4 Interface (host / contract — not full impl)

```text
// Read-only
fn get_pool(pool_id) -> Pool
fn quote_exact_in(pool_id, asset_in, Δ_in) -> Δ_out   // pure math on public R

// State-changing seam (proof verified by host)
fn apply_swap(public: SwapPublic, proof: Proof) -> Result<(), SwapError>
  // 1. load pool; check Active
  // 2. optional oracle_bound check (§3)
  // 3. recompute Δ_out from R; check Δ_out >= min_out
  // 4. verify proof (membership, nullifiers fresh, conservation, asset_ids)
  // 5. insert nullifiers; append cms; update R_A,R_B
```

**Error codes (normative names for tests):**

| Code | Condition |
|------|-----------|
| `ErrMinOut` | \(\Delta_{\text{out}} < \textit{min_out}\) |
| `ErrOracleStale` | bound required and mid older than max age |
| `ErrOracleSlippage` | implied price outside mid ± max_slippage_bps |
| `ErrOracleDisabledMint` | any attempt to credit balance from oracle alone |
| `ErrNullifierExists` | \(\nu\) already in set |
| `ErrWrongAsset` | note / public `asset_id` ≠ pool leg |
| `ErrPoolPaused` | status ≠ Active |
| `ErrInsufficientReserve` | \(\Delta_{\text{out}} \ge R_{\text{out}}\) |
| `ErrBadProof` | verification failure |
| `ErrBadRoot` | root not in allowed window |

---

## 3. Oracle bound API (hashmerchant VE)

### 3.1 Hard rule

> **hashmerchant / vote extensions NEVER mint balances.**  
> They supply multi-source mid / TWAP-like digests used only as **acceptance bounds** on an otherwise self-contained AMM swap.

Analog: Tacit **cUSD uses oracles for pricing**; **cBTC conservation does not mint from oracles**. Private DEX **solvency = cBTC posture**; **optional quote guard = cUSD posture**.

### 3.2 Data plane (existing module)

From `x/hashmerchant`:

- Validators attest via ABCI++ vote extensions (`VoteExtensionHashData`).
- Modular **oracle sources** with custody authenticators (ed25519, scoped e.g. `price/oracle` / `price_feed`).
- Per-source `OracleAttestation { source_id, value, height, timestamp, custody_signature }`.
- Aggregated `root` (or HMOR-prefixed bundle in legacy `ics23_proof`).
- Quorum (≥ ~⅔ voting power) before on-chain confirmation / sudo on-ramp to CosmWasm.

DEX seams **consume finalized** price material (query or sudo-cached), not raw peer extensions mid-round.

### 3.3 Bound interface (normative for Domain E)

```text
struct OracleMid {
  pair_or_asset_key:  bytes,    // e.g. asset_id or (asset_A, asset_B) quote key
  mid:                u128,     // price in fixed-point (document scale, e.g. 1e18)
  observed_height:    u64,      // foreign or local height from attestation
  observed_time:      i64,      // unix seconds from attestation
  chain_uid:          string,   // hashmerchant registration key
  algo:               string,   // e.g. sha256 / poseidon of bundle
  root:               bytes,    // confirmed HashRoot commitment to bundle
  source_count:       u32,      // optional quorum metadata
}

struct OracleBoundParams {
  max_age_blocks:     u64,      // staleness vs local chain height
  max_age_secs:       u64,      // optional wall clock vs block time
  max_slippage_bps:   u32,      // allowed deviation of AMM implied price from mid
  require_oracle:     bool,     // if true, missing/stale mid => reject (not silent skip)
}

// Pure checks — no balance mutation
fn mid(asset_or_pair) -> Option<OracleMid>
fn is_fresh(mid, now_height, now_time, params) -> bool
fn implied_price(R_in, R_out, Δ_in, Δ_out) -> u128   // e.g. Δ_in/Δ_out scaled
fn within_slippage(implied, mid.mid, max_slippage_bps) -> bool
```

**Bound predicate for swap acceptance (when oracle path enabled):**

\[
\begin{align*}
&\texttt{is\_fresh}(\textit{mid}, \ldots) \\
&\land\quad
\textit{implied} \in
\bigl[\textit{mid}\cdot(1-s),\; \textit{mid}\cdot(1+s)\bigr]
\end{align*}
\]

where \(s = \texttt{max\_slippage\_bps} / 10\,000\).

Optional **asymmetric** bound: only protect taker via

\[
\Delta_{\text{out}} \ge g(\textit{mid}, \Delta_{\text{in}}, s)
\quad\text{(floor from mid − slippage)}
\]

**in addition to** AMM \(\textit{min_out}\). Implementers MUST take the **stricter** of curve \(\textit{min_out}\) and oracle floor when both present.

### 3.4 Rejection rules

| Rule | Result |
|------|--------|
| `require_oracle` and no confirmed mid for pair | `ErrOracleStale` / `ErrOracleMissing` |
| mid age > `max_age_blocks` or > `max_age_secs` | `ErrOracleStale` |
| custody / quorum not met (no HashRoot) | treat as missing mid |
| implied price outside band | `ErrOracleSlippage` |
| **Any message that credits notes or increases user balance solely from mid** | **hard reject** `ErrOracleDisabledMint` |
| Oracle updates reserves without a swap proof | **hard reject** (not a valid transition) |
| Stale mid with `require_oracle = false` | MAY allow swap on pure AMM; MUST NOT invent balances |

### 3.5 What oracles must not do

- Mint `asset_id` notes or transparent coins.
- Increase \(R_A\) or \(R_B\) without a corresponding proven in-flow.
- Override conservation in the circuit.
- Substitute for light-client / reflection bridge mint authorization.

---

## 4. Action seams: Headstash claim & bridge mint → same anonymity set

### 4.1 Shared anonymity set

```text
                    ┌─────────────────────────────────────┐
                    │  Commitment tree T (one set)         │
                    │  + nullifier set N                   │
                    │                                      │
   Headstash claim ─┤→ cm_claim (asset_drop)               │
   Bridge mint    ─┤→ cm_bridge (asset_bridged)          │
   Swap out/change─┤→ cm_swap*                           │
   (future) LP    ─┤→ cm_lp                              │
                    └─────────────────────────────────────┘
         spends prove membership in T; ν ∉ N before insert
```

**Normative:** claim, bridge mint, and swap outputs **append leaves to the same tree** (or explicitly linked trees with a published union policy). Separate trees for airdrop vs DEX **break** the “claim → swap unlinkability” demo narrative and are **non-compliant** for Phase 1 seams.

### 4.2 Headstash claim → note

```text
Action: ClaimHeadstash
  public: eligibility_root / distro_id, nullifier_or_claim_id, cm_out, asset_id, proof
  private: eligible identity witness, amount v, owner, rcm
  effect:
    - verify eligibility once (claim marker / nullifier-like claim id)
    - append cm_out to T
    - do NOT touch AMM reserves
```

Claimed notes are **ordinary** multi-asset notes (§1.1). Subsequent `apply_swap` does not learn claim provenance.

### 4.3 Bridge mint → note

```text
Action: BridgeMint
  public: foreign_burn_id / reflection commitment, lc_height, asset_id, cm_out, proof
  private: amount v, owner, rcm, burn witness
  effect:
    - verify LC / reflection / conservation gate (Phase 3 full; Phase 1 may stub with test mint)
    - append cm_out to T
    - do NOT touch AMM reserves
    - do NOT use hashmerchant price mid as mint authority
```

Bridge mint is **conservation-preserving** relative to a proven foreign burn / lock. Hashmerchant may attest **foreign roots** for membership; that is **not** price minting.

### 4.4 Swap as the only reserve-touching private action (v1)

| Action | Tree T | Nullifier set N | Reserves R |
|--------|--------|-----------------|------------|
| Headstash claim | append | claim marker | — |
| Bridge mint | append | burn/mint id | — |
| Private swap | spend + append | insert \(\nu\) | update |
| Transfer (optional) | spend + append | insert \(\nu\) | — |
| Unshield (optional) | spend | insert \(\nu\) | — |

### 4.5 Composition sequence (demo-canonical)

```text
1. ClaimHeadstash(asset_B)  or  BridgeMint(asset_B)   → note_B in T
2. (optional) seed pool with POL transparent/init       → R_A, R_B public
3. apply_swap(asset_B → hub asset_H)                    → note_H in T; R updates
4. apply_swap(asset_H → asset_C)                        → multi-pair story via hub
5. Observer sees ΔR and nullifiers; cannot link 1→3→4
```

All steps are **deterministic** given public inputs; Domain E wires one transcript.

---

## 5. Pair topology recommendation

### 5.1 Phase 1 demo: **star / hub**

```text
                    asset_1
                       \
        asset_2 —— ★ HUB ★ —— asset_3
                       /
                    asset_4
```

| Choice | Recommendation |
|--------|----------------|
| Hub asset | Shielded TERP **or** tacit-mapped cBTC-class unit — freeze in Domain E demo config |
| Edges | One pool per `(HUB, asset_i)` |
| Routing | User (or UI) performs two swaps for asset_i ↔ asset_j |
| Why | Minimal pools, simple UI multi-pair list, one deep hub liquidity boot, matches PLAN § pair topology working default |

**`pool_id` (star):** `H(domain, asset_hub, asset_i, fee_params)`.

### 5.2 Later: **full pair matrix**

- Explicit pool per unordered pair `{A,B}`.
- Same note set T; only graph of pools grows.
- Optional `T_SWAP_ROUTE`-class multi-hop (Tacit) — **not** required for seams freeze.

### 5.3 Freeze for this SPEC

| Item | Decision |
|------|----------|
| Demo topology | **Star hub** |
| Matrix | Target after star green |
| Shared notes | **Required** across all edges |
| LP notes | Optional; POL-init reserves OK for demo |

---

## 6. Test scenarios

Each scenario is an interface-level test Domain E (or cw-multi-test / unit harness) SHOULD encode. Expected codes from §2.4.

### 6.1 `swap_happy`

**Setup:** Pool `(H, B)` with \(R_H, R_B > 0\); note of asset B with value \(v \ge \Delta_{\text{in}}\); fresh nullifier; valid root.

**Action:** `apply_swap` A→B orientation as appropriate with \(\textit{min_out} = \Delta_{\text{out}}\) exact (or slightly below).

**Expect:**

- Success.
- \(R_{\text{in}}' = R_{\text{in}} + \Delta_{\text{in}}\), \(R_{\text{out}}' = R_{\text{out}} - \Delta_{\text{out}}\).
- \(\nu\) in nullifier set; `cm_out` (and change) in tree.
- \(\Delta_{\text{out}}\) matches §2.2 floor formula.

### 6.2 `min_out_fail`

**Setup:** Same as happy, but \(\textit{min_out} = \Delta_{\text{out}} + 1\).

**Expect:** `ErrMinOut`; no reserve change; nullifier **not** inserted; no new leaves.

### 6.3 `oracle_stale`

**Setup:** `require_oracle = true`; last `OracleMid.observed_height` older than `max_age_blocks` (or time older than `max_age_secs`).

**Action:** Otherwise-valid swap with oracle bound attached.

**Expect:** `ErrOracleStale`; no state mutation.

**Variant:** `require_oracle = false` + stale mid → swap MAY succeed on pure AMM (document choice); still no mint from oracle.

### 6.4 `oracle_cannot_inflate_balance`

**Setup:** Attacker submits:

- a fake “oracle mint” message, **or**
- a swap proof with \(\Delta_{\text{out}}\) larger than formula, “justified” by a high mid, **or**
- a host API that credits a note from `OracleMid` alone.

**Expect:**

- No note created from mid alone (`ErrOracleDisabledMint` or equivalent auth failure).
- Inflated \(\Delta_{\text{out}}\) fails proof / host recompute (`ErrBadProof` or conservation failure).
- Reserves unchanged if reject path is clean.
- **Property:** \(\forall\) oracle updates, \(\sum\) note values by `asset_id` over T increases only via claim/bridge/LP deposit seams with their own authorities — **never** via mid.

### 6.5 `double_spend_nullifier`

**Setup:** Valid swap consuming note N with nullifier \(\nu\).

**Action:** Second swap (or transfer) reusing same \(\nu\) / same note opening.

**Expect:** First succeeds; second `ErrNullifierExists`; reserves only reflect first trade.

### 6.6 `wrong_asset_id`

**Setup:** Note of `asset_X` not equal to pool `asset_in`; public statement claims pool leg `asset_in`.

**Expect:** `ErrWrongAsset` or `ErrBadProof` (circuit binding); no reserve change.

**Variant:** `asset_out` commitment opens to wrong asset id → reject.

### 6.7 Recommended extras (progression, not blockers)

| ID | Intent |
|----|--------|
| `claim_then_swap_unlinkable` | Claim + swap leave no public link beyond shared tree growth |
| `bridge_mint_then_swap` | Same with bridge mint stub |
| `hub_two_hop` | B→H then H→C updates two pools |
| `oracle_slippage_reject` | Fresh mid but implied price outside bps band |
| `paused_pool` | `ErrPoolPaused` |
| `bad_root` | Membership against non-allowed root |

---

## 7. Non-goals

This SPEC does **not**:

1. Implement full DEX CosmWasm / module contracts or production circuits.
2. Ship Penumbra-style sealed batch auctions or Tacit `T_SWAP_BATCH` UCP (Phase 2+).
3. Define LP bond/farm opcodes (Tacit farm amendment) for v1.
4. Treat hashmerchant as a mint authority or CDP engine.
5. Re-home full Tacit Bitcoin envelope opcodes on Terp.
6. Require full pair matrix, concentrated liquidity, or orderbook.
7. Finalize Pedersen curve / proof system (Groth16 vs BP+/SP1) — only seam boundaries.
8. Replace light-client bridge design (Domain LC / Phase 3); only the **mint-into-T** seam.
9. Encrypted reserves or FHE dark pools.
10. Cross-domain privacy claims without shared (or proven-linked) trees.

---

## 8. Progression-grade checklist

Use this to grade Domain D freeze and Domain E wiring readiness.

### 8.1 Spec freeze

- [ ] Note minimum fields agreed (§1.1) and mapped to Tacit names where possible
- [ ] Swap public statement fixed (§1.2) including `min_out` and optional `oracle_bound`
- [ ] Conservation rules exclude oracle terms (§1.3)
- [ ] AMM formula + fee symbols match Tacit spirit (§2.2)
- [ ] Error codes stable for harness (§2.4)
- [ ] Oracle bound API is mid + staleness + slippage only (§3)
- [ ] **Documented hard reject:** oracle cannot mint / inflate (§3.4–3.5, §6.4)
- [ ] Headstash + bridge mint append to **same** tree T (§4)
- [ ] Star hub topology chosen for demo; hub asset named in demo config (§5)
- [ ] Six primary test scenarios have owners / stubs (§6.1–6.6)

### 8.2 Domain E wiring (exit signal for this SPEC’s success)

- [ ] End-to-end flow: claim (or bridge stub) → swap on hub edge → public \(\Delta R\) observed
- [ ] Second asset edge demonstrates multi-pair without second anonymity set
- [ ] Oracle path: happy bound + stale reject + no balance inflation property test
- [ ] Nullifier double-spend and wrong `asset_id` covered in CI
- [ ] Privacy honesty table published in demo README (what is revealed vs hidden)

### 8.3 Explicit trust labels (do not overclaim)

| Layer | Trust |
|-------|-------|
| Note conservation + reserve \(\Delta\) | Core solvency (proof + host recompute) |
| Public AMM mid from reserves | Transparent math |
| VE / custody oracle mid | **Declared** bound only; ⅔ validators + sources |
| Bridge mint | LC / reflection phase gates (not this SPEC’s proof) |
| Batch sealed swaps | Future phase |

---

## 9. Mapping cheat-sheet (Tacit → Terp seam)

| Tacit surface | Terp private DEX seam |
|---------------|------------------------|
| Confidential note `(asset_id, C, …)` | §1.1 multi-asset note |
| Virtual pool `R_A, R_B, S` | §2.1 public Pool |
| `T_SWAP_VAR` curve | §2.2 formula |
| `min_out` on intents / batch | §1.2 / §2.2 |
| Mixer / pool same privacy set | §4 shared tree T |
| cUSD oracle vs cBTC conservation | §3 bounds vs never-mint |
| `T_SWAP_BATCH` UCP | Non-goal v1; Phase 2 |
| Reflection `bridge_mint` | §4.3 seam (LC later) |

---

## 10. References (in-repo)

| Path | Use |
|------|-----|
| `crates/tacit/AMM.md` | Virtual AMM, \(\Delta_{\text{out}}\) formula, privacy posture |
| `crates/tacit/spec/SPEC-CONFIDENTIAL-POOL.md` | Note, nullifier, bridge_mint/burn |
| `x/hashmerchant/spec/01_concepts.md` | VE + quorum + sudo on-ramp |
| `x/hashmerchant/spec/05_vote_extensions.md` | Extension lifecycle |
| `x/hashmerchant/keeper/oracle.go` | Custody attestation verify |
| `x/hashmerchant/keeper/vote_ext.go` | Sidecar multi-source attestations |
| Program PLAN (daily-driver) §§3–5, 8 | Trust matrix, pair topology, oracle hard rule |

---

## 11. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial Domain D SPEC: private multi-pair DEX action seams (interfaces + tests) |
