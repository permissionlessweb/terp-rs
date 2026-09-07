---
title: Team clarity — Additive Headstash privacy sets + bridge burn/mint vs escrow
status: frozen-guidance
date: 2026-07-20
consumers: Domains A–E, Sprint 0+, ZEC↔BTC IBC roadmap
---

# Team clarity: Headstash additive privacy sets & how bridging actually moves funds

Two points that unblock design conversations. Read this before treating Headstash as “one fixed tree forever” or treating every bridge as “escrow like a CEX.”

---

## 1. Headstash: each new Headstash is an **additive** privacy set

### What we mean

A **Headstash** is not a one-shot airdrop that dies after claim day.  
Each **new** Headstash deployment / drop / set **adds** another **public eligibility set** whose claimants join a **growing privacy surface**:

| Layer | Role |
|-------|------|
| **Public eligibility set \(i\)** | Merkle (or manifold-registered) distribution for drop \(i\) — who *may* claim how much of which denom |
| **Claim / nullifier** | One-shot per leaf — prevents double-claim of *that* eligibility |
| **Privacy set (anonymity / note pool)** | Where private value lives after claim (or after shield) — **should grow with each Headstash**, not reset to an empty island |

**Additive** means:

```text
Headstash 0 → eligibility root_0 → claims join privacy pool P
Headstash 1 → eligibility root_1 → claims also join P (or P′ that is a proven extension of P)
Headstash 2 → eligibility root_2 → same pattern

Anonymity set size grows with cumulative successful claims across drops.
```

This matches the older monorepo product note (`docs/bootstrap/expanded-project-scopes.md` §1):

- **Manifold** registers many Headstashes / assets  
- **Chained / offset privacy sets** (or a single multi-root manifold) so a user can prove membership in **any** of sets \(0..N\) **without revealing which**  
- Each new drop is **powered** (new genesis tree + VK/config as needed) and **additive** (does not orphan prior set anonymity)

### What we are *not* saying

| Wrong | Right |
|-------|--------|
| One global tree forever with no registry | Manifold-aware: many eligibility roots, one composition policy |
| New Headstash **replaces** old anonymity | New Headstash **extends** the private surface |
| Claim privacy = “recipient address is random” only | Claim privacy = eligibility ↔ spend/claim unlinkability **plus** growing set |
| Oracle / VE grows the privacy set | Only **claims and private transfers** grow the private set |

### Design requirements (Domain A + manifold)

1. **`root_i` is first-class** — each Headstash has its own eligibility anchor (or epoch).  
2. **Manifold** lists active roots and token strategies without forcing separate dead-end circuits per drop if avoidable.  
3. **Nullifiers** scoped so claim in set \(i\) cannot be replayed as set \(j\) (domain separation).  
4. **Privacy pool / note set** after claim is **shared** with private DEX and bridge mints where product allows (same anonymity island).  
5. **Optional later:** prove “member of any root in manifold” (OR-membership / root list commitment) without revealing which Headstash.

### Relation to ZSA

ZIP 227 (OrchardZSA) thinks in **Asset Identifiers + issuance**. Multi-drop Headstash should eventually map:

- each drop / asset → stable `asset_id` (ZSA-aligned later)  
- issuance transparency for **supply** of the drop, privacy for **who claimed**

See [`SPEC-airdrop-orchard-delta.md`](./SPEC-airdrop-orchard-delta.md) and [`zsas.md`](./zsas.md).

---

## 2. Bridging: not all bridges are escrow — Bitcoin burn/mint vs Zcash out

### The confusion (named)

Escrow mental model (familiar, often correct for “wrap ZEC off home chain”):

```text
Source chain:  LOCK / DEPOSIT funds into a contract or vault (escrow)
Destination:   MINT a claim / note / voucher for equal value
Back home:     BURN voucher → UNLOCK escrow
```

That **is** a real and useful model — especially for **Zcash → foreign shielded deposit**, because ZEC that “left” home must be **accounted for** under some conservation story (lock, burn, or protocol-level issuance of a wrap).

**Tacit’s primary cross-chain path is not “multisig escrow of ETH/BTC with a trusted custodian.”**  
It is closer to **note conservation + reflection**: a **private note is burned** on one lane into a **bridge-burn record**, and a **new note is minted** on the destination only when a **proof** shows that burn is final and unspent. Value is not invented; it is **moved by proof**.

### Three different mechanisms (do not mix labels)

#### A. Same-chain shield / unshield (escrow-like boundary)

Tacit: `OP_WRAP` / `OP_UNWRAP` (and similar).

```text
Public coins ──wrap──► private note   (escrow or lock binds v)
Private note ──unwrap► public coins
```

- **Mint note** = against locked public funds on *this* chain  
- **Burn note** = to release those funds  
- Feels like escrow because public value sits in a contract while shielded

#### B. Cross-chain **reflection burn → mint** (Tacit ConfidentialPool primary)

```text
Lane A (e.g. Bitcoin-homed notes or EVM notes):
  spend note + OP_BRIDGE_BURN
  → emit cross_out / claimId / nullifier ν in the **bridge-burn set**
  → note is gone (cannot spend again)

Prover / LC:
  prove burn ∈ bridge-burn set under final root + confirmation depth
  prove v_mint == v_burn + domain binding

Lane B (destination):
  OP_BRIDGE_MINT once per ν / claimId
  → new private note on destination
```

| Word | Meaning here |
|------|----------------|
| **Burn** | Destroy (or permanently spend) a **private note** (or envelope) on the **source** so it cannot double-spend; creates a **provable exit claim** |
| **Mint** | Create a **new private note** on the **destination** only when the exit claim is authenticated |
| **Not escrow (custodial)** | No trusted party holds both sides; **soundness** is SP1/LC + conservation + one-shot nullifiers |
| **Still “escrow-like” intuition** | Value is “parked” in the cryptographic claim until the destination accepts it |

**Critical Tacit invariant (Domain B):** only **bridge-burn set** membership authorizes mint — ordinary spends do **not**.

See [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md).

#### C. Lock / wrap for native L1 coins (Bitcoin cBTC, classic wrap)

```text
BTC locked in Taproot / vault (user or protocol key path)
  → mint cBTC-class note 1:1 with proven lock
Unlock / reclaim paths have their own trust tier (see Tacit Tier 0/2)
```

This **is** closer to escrow/vault for **native BTC**.  
Still not “multisig bridge attestors” if the lock is self-custody + proof of lock.

### Escrow vs reflection — one table

| Question | Escrow wrap | Tacit reflection burn→mint |
|----------|-------------|----------------------------|
| Where is “the money” mid-flight? | Locked in source contract/vault | Encoded in **burn claim** + conservation proof |
| What authorizes destination mint? | Attestor / light client / custodian that “lock happened” | Proof of **bridge burn** under final root + finality |
| What is burned? | Often the **wrapped token** on dest when going home | The **private note** (or conf-burn) on source when leaving |
| Double-spend risk | Unlock without burn, or mint without lock | Mint without burn-set membership, or reuse ν |
| Feels like | Bank deposit then IOU | ZK conservation ticket |

### Zcash specifically (your scenario)

**“Bridge ZEC out of home into our shielded privacy deposit”** is **not** free on Zcash consensus today without an **issuance / lock / burn story**:

| Approach | What happens to ZEC | What we mint | Auth |
|----------|---------------------|--------------|------|
| **Escrow wrap** | ZEC locked in Zcash-side vault / pool | Note / ZSA-like wrap on Terp (or Tacit) | Zcash LC + membership of lock/deposit |
| **Burn-to-exit** (if protocol allows) | ZEC destroyed or unusable on Zcash | Wrap elsewhere | Stronger conservation if true burn |
| **OrchardZSA bridge-as-issuer** (ZIP 227 spirit) | Foreign asset issued under bridge issuer key | Custom shielded asset representing ZEC | Issuance rules + bridge honesty |

So your escrow intuition **is correct for ZEC leaving home**: something must **account for** the ZEC (lock or burn) before a foreign private note is honest.  
What we **reject** is unauthenticated mint (“oracle said you have ZEC”).

**Bitcoin side of the same product** may use:

- Tacit **Bitcoin-homed notes** + reflection to Ethereum/Terp, and/or  
- **cBTC-class lock**, and/or  
- Future **IBC-class light clients** carrying proven state both ways  

### Target product: direct **IBC path Bitcoin ↔ Zcash**

In scope as a **program goal** (not Sprint 0 implement):

```text
                    ┌─────────────┐
                    │ Terp / app  │  private notes, Headstash, DEX
                    │ 08-wasm LCs │
                    └──────┬──────┘
           Bitcoin LC      │      Zcash / Crosslink LC
           (headers,       │      (headers, finality,
            pool/burn      │       shielded anchors)
            roots, …)      │
                    ┌──────┴──────┐
                    │  IBC-class  │  packets optional;
                    │  evidence   │  mint gated by LC membership
                    └─────────────┘
         Bitcoin (Tacit notes / BTC locks)    Zcash (Orchard / ZSA / lock)
```

**Deterministic authentication** on that path always means:

1. Source finality (LC tip)  
2. Membership / burn / lock under that tip  
3. Domain binding (asset, chain, contract)  
4. One-shot nullifier / claim id  
5. Conservation \(v_{mint} = v_{burn|lock}\)  

Oracles may bound **prices** for private DEX; they **never** authorize bridge mint.

See [`SPEC-lc-hinge-private-bridge.md`](./SPEC-lc-hinge-private-bridge.md) and [`FLOW-private-bridge-auth.md`](./FLOW-private-bridge-auth.md).

### ZEC egress: light clients + TZE (ZIP-222) design loop — **not a blocker**

**Intent:** power Zcash egress with the **same design loop** we already invest in for light clients, plus **ZIP-222 TZE** (Transparent Zcash Extensions) where the grant path already couples TZE + headers + vote extensions / IBC demos (`docs/grant/zcash/*`).

```text
Zcash side (future sprint family)
  ZIP-222 TZE  ──► structured transparent extension data
  headers / Crosslink finality ──► LC tip on Terp
  lock or burn accounting     ──► conservation for mint
        │
        ▼
  Terp LC hinge (Domain C) + Tacit-class mint gate (Domain B)
        │
        ▼
  private note in shared set (additive with Headstash)
```

| Rule | Meaning |
|------|---------|
| **Leverage** | Reuse LC hinge + domain binding + one-shot claim; add TZE as Zcash-native **evidence channel** when available |
| **Not blocked** | Headstash additive sets, Tacit mapping, private DEX seams, hashmerchant bounds, suite/harness work ship **without** waiting on ZEC egress |
| **North star** | IBC-class Bitcoin ↔ Zcash still the long arc; TZE is a Zcash-side instrument, not a rewrite of Terp note algebra |

Grant anchors: ZIP-222 TZE closed loop + IBC + VE demos in `docs/grant/zcash/grant.md` / `integration-domain-feedback.md`.

---

## 3. How the two points compose

| Headstash additive sets | Bridge models |
|-------------------------|---------------|
| Each drop adds eligibility **and** should grow shared private surface | Bridge mints should land in **that same** private surface when product wants one anonymity set |
| Manifold registers many roots | LC + domain binding register many **foreign assets** (ZEC, BTC-class, …) |
| Claim is one-shot per leaf | Bridge mint is one-shot per burn ν / claimId |

```text
Headstash_i claim ──┐
                    ├──► shared private note set P ──► private DEX
BTC/ZEC bridge mint ┘         ▲
                              │ authenticity from LC + conservation
                              │ (not from escrow brand name alone)
```

---

## 4. Scope flags for the team

| Item | Scope |
|------|--------|
| Additive / manifold Headstash privacy sets | **In design scope** — Domain A + manifold; implement after single-set H1–H6 green |
| Document escrow vs reflection burn/mint | **In scope** — this note + Domain B/C/E |
| ZEC escrow/lock → Terp private deposit | **In product scope** — Domain C Zcash LC + Domain B conservation; not Sprint 0 circuit |
| Direct IBC Bitcoin ↔ Zcash | **Program north star** — requires both LCs + note conservation; phased |
| Custodial multisig “bridge” | **Out** as primary soundness model |

---

## 5. One-liners for standups

1. **Headstash:** every new Headstash is a **new public eligibility set** that **adds** to the private anonymity surface (manifold-powered, not a reset).  
2. **Bridge:** **burn** = destroy source private claim under a proven set; **mint** = create dest note only under **finality + membership + conservation**. Escrow/lock is one way to back native ZEC/BTC; it is not the only Tacit path, and it is not “oracle mint.”  
3. **North star:** authenticatable **IBC-class evidence** between **Bitcoin and Zcash**, with Terp as private composition (Headstash + DEX + notes).

---

## Related SPECs

| Doc | Use |
|-----|-----|
| [`SPEC-airdrop-orchard-delta.md`](./SPEC-airdrop-orchard-delta.md) | Claim circuit delta, H1–H6 |
| [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) | Burn/mint ops, trust tiers |
| [`SPEC-lc-hinge-private-bridge.md`](./SPEC-lc-hinge-private-bridge.md) | What LC must prove |
| [`SPEC-private-dex-seams.md`](./SPEC-private-dex-seams.md) | Shared anonymity with swaps |
| [`FLOW-private-bridge-auth.md`](./FLOW-private-bridge-auth.md) | End-to-end steps |
| [`zsas.md`](./zsas.md) | OrchardZSA issuance parent |
