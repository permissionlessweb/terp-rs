# ZK-JWT Authentication Flow (expected support)

This is the **contract-facing spec** for how JWT authentication is expected to work with
`terp-zkjwt` and CosmWasm host verification. It reduces friction for product/crypto design
by fixing the on-chain interface while leaving circuit internals open.

---

## 1. Mental model

```
Web2 OIDC/JWT  →  off-chain prover  →  ZkJwtAuthPayload  →  x/smart-account Authenticate
                         │                      │
                         │                      ▼
                         │              proof_instance_verify(zkid, proof, instances)
                         │                      │
                         └──── never put raw JWT on-chain ────┘
```

The contract does **not** parse JWT headers/payloads. It verifies a **ZK proof** that
such a JWT existed and satisfied circuit constraints (issuer, claims, nullifier).

### Primary product target: Headscale

Headscale already uses OIDC for mesh identity. **The same OIDC JWT / claims** that
admit a user to the mesh become the **witness** for zk-jwt on Terp:

- Headscale plane: network membership, ACL, join material  
- Terp plane: smart-account tx authority via `terp-zkjwt`  
- Bridge: ZK proof + public instances (never the raw JWT)

Team focus doc: `reviews/ZKJWT-HEADSCALE-FOCUS.md`.

---

## 2. Parties

| Party | Responsibility |
|-------|----------------|
| **Issuer** | OIDC provider (Google, Auth0, enterprise IdP). Registered with `IssuerConfig`. |
| **User / client** | Completes OAuth, runs prover, submits Authenticate. |
| **Chain / VM** | Holds circuit VK under `zkid`; implements `proof_instance_verify`. |
| **Contract** | Policy: issuer allowlist, claim binding, nullifier spend, circuit_id allowlist. |

---

## 3. Registration (optional but recommended)

### 3a. Admin: register issuer

```json
{
  "issuer": "https://accounts.google.com",
  "verifying_key": "<client JWKS thumbprint or empty>",
  "zkid": 42,
  "audience": "terp-app"
}
```

- **`zkid`**: required for production (`zk-host`). Host resolves VK.
- **`verifying_key`**: optional client hint; **not** used for host PLONK verify.
- **`audience`**: product policy; circuit should bind `aud` into instances when enforced.

### 3b. User: RegisterClaim (link)

```json
{ "claim_commitment": "<32 bytes>", "issuer": "https://accounts.google.com" }
```

- Stores `claim_commitment → msg.sender`.
- When `require_registered_claim = true`, Authenticate requires  
  `CLAIMS[commitment] == AuthenticationRequest.account`.

**Claim commitment** (circuit / product agreement — not enforced algebraically on-chain beyond equality):

```text
claim_commitment = Poseidon/SHA256( issuer || subject || salt )  // circuit-defined
```

---

## 4. Authenticate (core path)

### 4.1 Payload (`AuthenticationRequest.signature` = JSON)

```json
{
  "issuer": "https://accounts.google.com",
  "claim_commitment": "<32 bytes>",
  "public_inputs": "<nullifier||claim_commitment||rest>",
  "proof": "<circuit proof bytes>",
  "circuit_id": "zkjwt.membership.v1"
}
```

### 4.2 Public instance layout (frozen)

| Offset | Len | Field | Required |
|--------|-----|--------|----------|
| 0 | 32 | `nullifier` | yes |
| 32 | 32 | `claim_commitment` | yes (must match payload field when len=32) |
| 64 | 32 | `inclusion_set_root` | **yes if** `IssuerConfig.inclusion_set_root` is set |
| 96 | 32 | `msg_bind` (recommended) | circuit-optional |
| 128+ | … | aud_hash, … | circuit-defined |

**Inclusion set root:** issuer/operator registers the merkle root of allowed identities
(Headscale admitted `sub`s, JWKS epoch, etc.). Circuit proves leaf membership; contract
checks the public root equals the registered root. Rotate via `RotateInclusionRoot`.

```text
msg_bind = SHA256(
  "terp-auth/zkjwt/v1" || chain_id || account || authenticator_id ||
  msg_index_le64 || SHA256(sign_mode_direct)
)
```

On-chain **today** parses offsets 0..64. Columns after 64 are passed through to the host
as part of `public_inputs` but not yet policy-checked (specialist fills enforcement).

### 4.3 Contract steps (implementation)

1. Decode payload; load `IssuerConfig` by `issuer`.
2. Structural: non-empty proof; known `circuit_id`; parse instances.
3. **Verify**
   - `zk-host`: `deps.api.proof_instance_verify(zkid, proof, public_inputs)`
   - else: structural only (CI).
4. If `require_registered_claim`: owner match.
5. Reject if nullifier already in `NULLIFIERS`; else store height.
6. Return ok attributes (`verify`, `zkid`, `circuit_id`).

### 4.4 What the circuit must prove (product/crypto intent)

At minimum for `zkjwt.membership.v1`:

1. JWT signature valid under issuer keys (or Merkle of JWKS).
2. Selected claims map to `claim_commitment`.
3. `nullifier` is bound to unique JWT identity (e.g. `sub` + domain) so same JWT cannot mint many nullifiers.
4. Optionally `msg_bind` equals host-computed bind (prevents cross-msg reuse).

---

## 5. Circuit IDs (policy strings)

| ID | Intent |
|----|--------|
| `zkjwt.membership.v1` | Generic issuer membership |
| `zkjwt.email_domain.v1` | Prove email domain without full email |
| `zkjwt.profile_link.v1` | Bind to prior RegisterClaim profile |

Host selection uses **`IssuerConfig.zkid`**, not these strings. Strings are product/circuit catalog labels.

---

## 6. Explicit non-goals (for now)

- Parsing raw JWT on-chain  
- Full OIDC discovery in contract  
- Putting `id_token` in tx data  
- VSCK vote proofs (separate authenticator)

---

## 7. Client checklist

1. Register / know `zkid` for issuer circuit.  
2. After OAuth: build witness; prove; assemble `public_inputs`.  
3. Optionally `RegisterClaim` once.  
4. Submit tx with CosmWasm authenticator → Authenticate(payload).  
5. On failure: distinguish structural vs host-false vs nullifier-replay vs claim mismatch.

---

## 8. Open items (design, not blocking contract API)

- Exact JWT circuit (circom in `crates/zk-jwt` vs Halo2)  
- Whether `aud` is PI column or issuer-config-only  
- Default `require_registered_claim` for production  
- Composite with passkey (AnyOf / AllOf)

The **on-chain API above is the expected support surface** for JWT authentication until those open items close.
