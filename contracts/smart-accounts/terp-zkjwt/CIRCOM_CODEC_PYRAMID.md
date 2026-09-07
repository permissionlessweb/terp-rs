# Circom JWT claim codec — verification pyramid

Aligned with o-line **gateway** Headscale coupler (`just test gateway` manifold).

## Pyramid (honest claims)

| Level | Command | Claim |
|-------|---------|-------|
| **L0+L1** | `cargo test -p terp-zkjwt --lib circom` + `cargo test -p terp-authenticator-suite --test zk_jwt_circom_codec` | Codec + **real snarkjs public** fixtures green |
| **L1 only** | `cargo test -p terp-zkjwt --lib circom_codec` | Full sound / sparse fail-closed |
| **L2 crypto (Node)** | `node scripts/l2_snarkjs_verify.mjs` | snarkjs.groth16.verify on committed fixtures |
| **L2 ark convert** | `cargo test -p zk-cosmwasm --features bn254 --lib snarkjs` | snarkjs JSON → ark-groth16 verify |
| **L2 e2e authenticator** | `cargo test -p terp-authenticator-suite --features zk-host --test zk_host zk_host_e2e_snarkjs` | **Real jwt-auth proof through terp-zkjwt Authenticate** |
| **L2 Path A square** | `cargo test -p cosmwasm-vm --features zk,bn254 --lib proof_instance_verify_bn254` | Host ark Path A (tiny golden) |
| **L2 chain** | (future) wasmd + host snarkjs→ark JWT blob | Full chain |
| **L3** | o-line `just test gateway chain hs` | Live Headscale join |

## L2 fixtures (committed)

| File | Content |
|------|---------|
| `src/fixtures/l2/public.json` | 40 Fr public signals from snarkjs fullProve |
| `src/fixtures/l2/proof.json` | snarkjs groth16 proof |
| `src/fixtures/l2/jwt-auth_vkey.json` | verification key (GCS demo-18-12-2024) |
| `src/fixtures/full_circom_publics.json` | copy of public.json for codec tests |

**Not committed:** `jwt-auth.zkey` (~700MB) — download when regenerating.

## Soundness API (gateway twin)

| Gateway (`o-line` auth_plane) | ZK-JWT codec |
|-------------------------------|--------------|
| `grant_is_sound(g, require_eph)` | `claim_is_sound(c)` |
| `MIN_SOUND_GRANT_RICHNESS` (8) | `MIN_SOUND_CLAIM_RICHNESS` (8) |
| full wasmd merge fixture | `full_circom_publics.json` (real) |
| sparse Response-only | `sparse_circom_publics_only.json` |
| `schema=v1` on access_granted | `CLAIM_EVENT_SCHEMA_V1` |
| `nullifier_hex` for bridge | `claim.nullifier_hex` |

## Codec v1 mapping (production 40-public layout)

From snarkjs `public.json` / circuit outputs:

| Index | Signal | Policy |
|-------|--------|--------|
| 3 | `publicKeyHash` | richness |
| **4** | **`jwtNullifier`** | **nullifier** |
| 5 | `timestamp` | richness |
| **26** | **`accountSalt`** | **claim_commitment (codec v1)** |
| 39 | `isCodeExist` | richness |

inclusion_set_root / msg_bind → **not** circuit-bound (D3).

## Regenerating L2

```bash
# tools
cd crates/zk-jwt && yarn install
cd packages/circuits

# inputs
ACCOUNT=0x1162ebff40918afe5305e68396f0283eb675901d0387f97d21928d423aaa0b54
yarn gen-input --account-code $ACCOUNT \
  --input-file ../contracts/test/build_integration/input.json

# params (one-time, ~700MB zkey)
mkdir -p /tmp/zk-jwt-params && cd /tmp/zk-jwt-params
curl -fsSL -o jwt-auth.zkey \
  https://storage.googleapis.com/zk-jwt-params/demo-18-12-2024/jwt-auth.zkey
curl -fsSL -o jwt-auth.wasm \
  https://storage.googleapis.com/zk-jwt-params/demo-18-12-2024/jwt-auth_js/jwt-auth.wasm
curl -fsSL -o jwt-auth_vkey.json \
  https://storage.googleapis.com/zk-jwt-params/demo-18-12-2024/jwt-auth_vkey.json

# prove + verify
npm install snarkjs@0.7.4
node -e "
const snarkjs=require('snarkjs'); const fs=require('fs');
(async()=>{
  const input=JSON.parse(fs.readFileSync('…/input.json'));
  const {proof,publicSignals}=await snarkjs.groth16.fullProve(input,'./jwt-auth.wasm','./jwt-auth.zkey');
  fs.writeFileSync('proof.json', JSON.stringify(proof,null,2));
  fs.writeFileSync('public.json', JSON.stringify(publicSignals,null,2));
  console.log(await snarkjs.groth16.verify(JSON.parse(fs.readFileSync('jwt-auth_vkey.json')), publicSignals, proof));
})();
"

# copy into repo
FIX=crates/terp-rs/contracts/smart-accounts/terp-zkjwt/src/fixtures
cp public.json proof.json jwt-auth_vkey.json $FIX/l2/
cp public.json $FIX/full_circom_publics.json

# gates
cd crates/terp-rs/contracts/smart-accounts/terp-zkjwt
npm install snarkjs@0.7.4
node scripts/l2_snarkjs_verify.mjs
cargo test -p terp-zkjwt --lib circom
cargo test -p terp-authenticator-suite --test zk_jwt_circom_codec
```

## Bridge handoff

```text
nullifier_hex = claim.nullifier_hex
principal     = claim.principal_hint_hex
schema        = v1
→ o-line grant_is_sound → bridge nullifier store
```
