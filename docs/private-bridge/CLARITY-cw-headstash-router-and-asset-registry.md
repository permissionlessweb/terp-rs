# CLARITY — `cw-headstash` as mint router + asset registry posture

| Field | Value |
|-------|--------|
| **Status** | Accepted product decision |
| **Date** | 2026-07-20 |
| **Deciders** | program / Headstash |
| **Resolves** | ROUND1-META-REVIEW D8 (contract home), D3 (asset_id SSOT path), HARNESS/BRIDGE suite home |

---

## 1. Mint and on-chain router home

**Decision:** **`cw-headstash` is the mint.**

Unify the Headstash product as the **on-chain router** for private-note entry surfaces, including:

| Ingress | Role on `cw-headstash` |
|---------|------------------------|
| **Eligibility / airdrop claim** | Existing `ProcessHeadstash` + additive distro roots (`poseidon-v1`) |
| **Bridge mint** (Tacit reflection / LC hinge) | **New execute path(s)** on the same contract (or clearly versioned msg family), not a separate `cw-bridge-mint` product |
| **Future swap-related mints / note outs** | Routed through the same contract’s note/nullifier/root authority (DEX settle may call into or settle under this router’s rules) |

### Consequences

1. **No parallel mint package** for production bridge mint. Round-2 BRIDGE CosmWasm work targets **`cw-headstash` msg/state extensions** (or a feature-gated module *inside* the same crate), not a freestanding factory mint.
2. **Nullifier and note authority stay one product surface** — domain separation remains *logical* (`root_id`, origin tags, SEAM nullifier domains `0x01` claim / `0x02` bridge / `0x03` pool), not *separate contracts*.
3. **Manifold** continues as factory/index for multi-Headstash instances; each instance is still a `cw-headstash` that owns mint routing for its pool of sets.
4. **HARNESS** `PrivateBridgeSuite` / test-press suites should extend **`HeadstashSuite`** (or compose headstash + optional Tacit RPC), not invent a second deployable mint.

### Non-goals (this decision)

- Implementing full bridge-mint execute in the same PR as this clarity note.
- Merging Tacit EVM `ConfidentialPool` into CosmWasm.

---

## 2. Asset registry: internal **or** external

**Decision:** Asset registry may be **internal** (state on `cw-headstash`) **and/or** **external** (another contract/module). We will **expand usage here with cross-chain wiring**.

### 2.1 Internal registry (default for single-drop demos)

On `cw-headstash` (or shared package types used by it):

| Field (logical) | Role |
|-----------------|------|
| `asset_id` | Domain-separated bytes / fixed-width id used in notes and circuits |
| `local_denom` / proof representation | CosmWasm denom + existing `NoteDenom` / blake3 proof form where claim payouts need bank/tf |
| `origin` | e.g. native, tacit lane, IBC denom trace |
| `status` | active / paused / frozen |

Today’s `TokenStrategy` / `HeadstashTokenObject` is the **seed** of internal registry (claim payout assets). Expand rather than replace blindly.

### 2.2 External registry (cross-chain / multi-venue)

An external registry (contract or chain module) may own:

- Canonical **cross-chain asset keys** (Tacit `assetId`, IBC denom, Ethereum address+chain, etc.)
- **Mapping** into Terp `asset_id` for notes
- Governance / upgrade of maps without redeploying every Headstash instance

`cw-headstash` then **queries or is configured with** the external registry address (or module query path) and **caches or pins** mappings at mint time.

### 2.3 Cross-chain wiring expansion

Wiring layers (progressive):

```text
1. Local map only (internal table: tacit_id → terp asset_id → denom)
2. External registry contract (versioned maps, multi-Headstash share)
3. LC / reflection–gated updates (new maps only after authentic foreign state)
4. IBC / hashmerchant-class attestations as *map inputs* — never as balance mints
```

**Invariant (with Domain D / B):** registry and oracles **never mint note balances**. They only resolve **which asset_id / denom** a mint or swap leg may use.

### 2.4 Round-2 freeze for pure seams

Until a shared package exists:

| Layer | Temporary SSOT |
|-------|----------------|
| Claim payout | Existing `TokenStrategy` / `NoteDenom` |
| Bridge pure hinge | Keep `terp_asset_id_from_tacit` (or successor) labeled **fixture map v0**; document domain string |
| Swap pure seams | 32-byte `asset_in` / `asset_out`; demo HUB/B/C only in tests |
| **Target** | One `AssetRegistryView` trait used by hinge + swap + contract |

---

## 3. Answers to open Round-1 questions (this note)

| Q | Answer |
|---|--------|
| Contract home for bridge mint? | **`cw-headstash`** (unified product router) |
| Separate `cw-bridge-mint`? | **No** as product default |
| Asset registry location? | **Internal and/or external**; expand with **cross-chain wiring** |
| Suite home for bridge e2e? | Extend **Headstash / PrivateBridge suite around `cw-headstash`** |

---

## 4. Round-2 agent implications

| Agent | Implication |
|-------|-------------|
| **BRIDGE** | Sketch `ExecuteMsg` variants on **cw-headstash** (`BridgeMintNote`, `UpdateReflection`, `RegisterAsset` internal, `SetAssetRegistry` external addr). Emit SEAM-NOTE-OUT with `rcm` for DEX consumability. |
| **HARNESS** | L1/L3 always deploy **one headstash code_id**; no second mint wasm. E2E burn→mint hits headstash. |
| **SWAP** | Consume notes whose `asset_id` comes from registry view; do not invent a fourth id scheme. |
| **COMPOSE (if any)** | `authorize_bridge_mint` → SEAM note out → optional `SwapActionV0` under **same** product tree narrative. |

---

## 5. Related docs

- `CLARITY-headstash-sets-and-bridge-models.md` — additive sets; burn/mint vs escrow  
- `SPEC-tacit-bridge-mapping.md` — mint-critical public values  
- `POSEIDON-DISTRO-SURFACE.md` — distro roots (eligibility ≠ asset registry)  
- `ROUND1-META-REVIEW.md` — D3/D8 closed by this note  
