# ADR: Poseidon for Headstash **public inclusion** sets (not Sinsemilla)

| Field | Value |
|-------|--------|
| **Status** | Accepted (product) — implement now |
| **Date** | 2026-07-20 |
| **Deciders** | program / Headstash |
| **Related** | SPEC-airdrop, CLARITY additive sets, SEAM-NOTE-OUT |

## Context

Headstash uses a **public eligibility / distribution Merkle set** so claimants prove inclusion without revealing which leaf until claim. Today the leaf hash / tree CRH path is built on **Sinsemilla** (Orchard-family), which is elegant for Pasta circuits but is **discrete-log / elliptic-curve based** in a way that is fragile under a future with large-scale quantum computers (and under “retroactive break of DL-like hardness” concerns discussed in prior sessions).

That is the **wrong trust posture** for a **public inclusion set**:

- Inclusion lists often hold **identities / eligibility pubkeys** of real people and communities.
- Creators of Headstashes publish roots that must remain meaningful for a long time.
- A future break of Sinsemilla’s algebraic assumptions should not retroactively undermine the integrity story of “who was in the set.”

## Decision

**Use Poseidon (Pasta-friendly sponge / fixed-width hash over `pallas::Base`) for all hashing in the Headstash public inclusion domain:**

1. **Leaf hash** of eligibility records (epk / binding, `nd`, `v`, `fdi`, …)  
2. **Merkle CRH** parent nodes for the distribution tree  
3. **Root** committed on-chain as `genesis_root` / manifold roots  

**Do not use Sinsemilla** for the public distro tree going forward.

### What stays Sinsemilla / Orchard (for now)

| Domain | Hash | Rationale |
|--------|------|-----------|
| **Public inclusion set** | **Poseidon** | Long-lived public integrity; respect listed individuals; PQ-friendlier posture for this surface |
| **Private note commit / nullifier (Orchard path)** | Orchard/Sinsemilla (or later migration) | Private note algebra can follow Orchard; separate threat model; migrate later if needed |
| **SEAM-NOTE-OUT** | Encoding only | Unchanged field roles; distro `provenance_anchor` is still a 32-byte root — algorithm tagged |

### Domain separation

```text
POSEIDON_DISTRO_LEAF  = Poseidon( personalization="terp-hs-distro-leaf-v1",  fields… )
POSEIDON_DISTRO_CRH   = Poseidon( personalization="terp-hs-distro-crh-v1",   layer, left, right )
```

Exact arity / padding: fixed in circuit + suite pure helpers; **must match** between off-chain tree builders and in-circuit verify.

## Consequences

- **Breaking change** for existing distro trees / VKs that used Sinsemilla leaves — version roots with `distro_hash_domain = poseidon-v1`.  
- Contract surface must carry **hash domain** (or assume poseidon-v1 for new Headstashes).  
- Additive multi-root manifold only registers **Poseidon-v1** roots for new drops.  
- Circuit Part I: replace `derive_leaf` / merkle path for **genesis inclusion only** with Poseidon gadgets; keep note commit path separate until a later ADR.  
- Pure fixtures can implement Poseidon offline tree builders for tests without Halo2.

## Non-goals (this ADR)

- Full post-quantum SNARK  
- Replacing Orchard note commit in the same PR  
- IVC / Tachyon  

## Workstreams

| Agent | Focus | Status (2026-07-20) |
|-------|--------|---------------------|
| **Circuit** | Poseidon distro leaf + CRH + circuit inclusion path; pure suite helpers; tests without full K=18 if possible | **Done:** pure SSOT, suite defaults Poseidon, in-circuit gadgets, `Circuit::synthesize` wired, Part T H1 MockProver green |
| **Contract** | Instantiation / manifold multi-root + `distro_hash_domain`; claim verifies root under poseidon-v1 policy | Landed: default Poseidon-v1; additive roots Poseidon-only; nullifier key `{root_id}:{nf}` |

---
