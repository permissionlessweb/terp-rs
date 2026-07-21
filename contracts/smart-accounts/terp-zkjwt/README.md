# terp-zkjwt

CosmWasm authenticator: **prove you hold a valid JWT from a trusted issuer** without revealing the token on-chain.

## Why

Web2 apps already identify users with OAuth/OIDC JWTs (Google, GitHub, Auth0, enterprise SSO). Smart accounts need those identities for recovery, org membership, and progressive onboarding — but dumping JWTs on-chain leaks PII and enables replay.

This authenticator expects a **ZK proof** that:

1. A JWT was signed under a registered issuer verifying key  
2. Selected claims commit to a public `claim_commitment`  
3. A nullifier binds the proof so it cannot be replayed  

Verification is funneled through [`verify::ZkJwtVerifier`](src/verify.rs). Today’s default checks proof envelope structure; plug Halo2/JWT circuits (or CosmWasm zk host APIs) without changing sudo routing.

## Lifecycle

```
Admin: register issuers (JWKS / VK)
   │
User: OAuth login (web2) → compute claim_commitment → RegisterClaim
   │
Tx: AuthenticationRequest.signature = ZkJwtAuthPayload { proof, public_inputs, … }
   │
on_auth_request → ZkJwtVerifier::verify
on_auth_track   → record nullifier
```

## Real-world applications (web2 fluent)

| Pattern | How zk-jwt helps |
|---------|------------------|
| **SSO account recovery** | User proves “I control Google `sub` linked at signup” to rotate keys without seed phrase. |
| **Org / email-domain gates** | Circuit `zkjwt.email_domain.v1`: prove `@acme.com` membership to join a DAO vault or company workspace account. |
| **Progressive profile linking** | Existing website session → `RegisterClaim` → same browser can authorize CosmWasm txs as that profile. |
| **Age / KYC attestation reuse** | Issuer is a KYC provider JWT; proof shows attestation still valid without republishing documents. |
| **Session continuity for dapps** | Web app keeps OIDC cookie; extension builds ZK proof for chain actions while cookie never touches RPC. |
| **Support / admin break-glass** | Dual authenticator: passkey day-to-day + zk-jwt from corporate IdP for recovery multisig. |

## Circuit IDs

- `zkjwt.membership.v1` — generic issuer membership  
- `zkjwt.email_domain.v1` — domain claim  
- `zkjwt.profile_link.v1` — bind to prior `RegisterClaim`  

## Related

- Suite interface: `terp-authenticator-suite`  
- VSCK private voting: `terp-vsck` (DAO voting module path)  
- Types/trait: `terp-auth` / `terp-account` in this workspace  
