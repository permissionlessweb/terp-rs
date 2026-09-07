# Prep: unify Blossom/BUD + S3-IPFS, then calendar Nostr e2e

**Status:** Phase A L0 **green** · **Phase B implemented** (content bind + NIP-52 egress + suite powered)  
**Date:** 2026-07-20  
**Related:** `content-distribution.md`, `e2e-capability-matrix.md`, calendar mesh design, `nostr_orch_suite`

---

## Why this order

Calendar Nostr + e2e depend on a **single content plane** for:

| Consumer | Needs |
|----------|--------|
| **dao-calendar** off-chain events | `MetadataExt.cid` → resolvable blob (BUD sha256 and/or IPFS) |
| **Nostr relay egress** | Full NIP-52 JSON often off-chain; chain holds `cid` or on-chain `e` |
| **Connect / mesh query** | Optional media / event bodies via same URLs as mint trees |
| **oline S3 → IPFS** | `content.available` webhooks into hash-market |

If S3-IPFS / Blossom remain half-wired, calendar e2e will invent a second storage path and diverge from BUD ecosystems.

---

## Already in place (do not re-litigate)

| Surface | Location | Notes |
|---------|----------|--------|
| BUD-01/02/06/07 traits + blossom router | `crates/nips` `buds/*` | Auth, BlobStore, HTTP routes |
| TreeStore = sole BlobStore | `hash-market/src/store.rs` | `/f/{sha256}` + content-ID index |
| Dual-index distribution (opt-in) | `hash-market` `[distribution]`, `src/content/` | BUD label + optional IPFS pin + webhooks |
| MinioIpfsClient (partial) | `client/minio_ipfs.rs` | Kubo pin/fetch; MinIO durable hook TODO |
| oline webhook-collector | `o-line/plays/instant-replay/` | S3 → IPFS pin path |
| NostrTestEnv + chain_event_to_nostr | `terp-rs/tests/src/environments/nostr.rs` | Harness ready; suite **commented** |
| FE Nostr subscribe only | `terp-docs/lib/mesh/nostr.ts` | Egress listen; no production publisher |
| dao-calendar NIP-52 storage | `dao-contracts/.../dao-calendar` | On-chain meta; hooks = Wasm, not Nostr |

**Gap:** no production **chain → Nostr** publisher; no **Nostr → chain** ingress; e2e suite not powered.

---

## Phase A — Finish content plane refactor (this track)

**Goal:** one compatible address space for Blossom ecosystems + oline S3-IPFS.

### A1 — Contract freeze

Lock these invariants in code comments + `content-distribution.md`:

1. **Primary key:** BUD **sha256** (hex) everywhere public  
2. **Secondary:** IPFS CID (optional pin)  
3. **Announce:** `content.available` event (sha256, ipfs_cid, urls.bud / urls.ipfs / urls.s3)  
4. **Default off:** `distribution.enabled = false` keeps mint/VE hosts simple  
5. **Single store:** all blobs through `TreeStore` / `BlobStore` — no parallel in-memory blossom store

### A2 — Wire remaining S3-IPFS hooks

| Task | Done when |
|------|-----------|
| MinIO webhook → `POST /webhooks/s3` | Ingest creates dual-index row without manual register |
| Upload path `pin_on_upload` | BUD POST optionally pins Kubo; failure policy = `pin_required` |
| Public resolve | `GET /content/{sha256}` local → optional IPFS redirect |
| MinioIpfsClient durable root | `on_root_confirmed` can write S3 + pin (today logs only) |
| oline collector | Posts `content.available` to bud-host (or dual notify) |

### A3 — Compatibility smoke (no calendar yet)

```text
L0: unit tests for dual-index register / resolve
L1: POST /blobs → registry row + optional ipfs_cid
L2: POST /webhooks/s3 with fixture → GET /content/{sha256} hits
L3: optional Docker MinIO + Kubo + hash-market mint image
```

Add a capability flag to the matrix when ready: `content_distribution` (not default CI).

### A4 — API surface for other ecos

Document for calendar / Nostr authors:

```http
# Store off-chain event JSON (authenticated BUD upload)
POST /blobs  → { sha256, url }

# Resolve later (public if public_get)
GET /content/{sha256}  → bytes or 307 to IPFS
GET /content/{sha256}/meta → { sha256, ipfs_cid, urls }
```

Off-chain calendar events should set `MetadataExt { on_chain: false, cid: Some(sha256_or_ipfs), kind: 31922|31923, … }` with **one documented preference** (prefer sha256 as cid string or `bud:sha256:` / `ipfs:bafy…` prefix — pick in A1).

---

## Phase B — Calendar + Nostr (**shipped**)

Only start when dual-index resolve works in at least L1 smoke. **A is green.**

### B1 — Content binding for dao-calendar ✅

| Mode | Chain holds | Body lives |
|------|-------------|------------|
| On-chain | full NIP-52 in `e` | chain |
| Off-chain | `cid` + kind + d_tag | BUD/S3/IPFS via content plane |

Helpers: `hash_market::bind_offchain_event` / `resolve_local` / `resolve_http` / `resolve_url`  
(`src/content/calendar.rs`). Bridge: `cid` → `GET /content/{sha256}`.

### B2 — Egress: chain → Nostr (production-shaped) ✅

```text
ChainEventWatcher (wasm-*)
  → parse_calendar_action (create/update/cancel)
  → load MetadataExt (on-chain e or fetch cid via content plane)
  → metadata_to_nip52_event → publish NIP-52 EVENT
```

| Surface | Location |
|---------|----------|
| Pure egress | `hash-market` `client/nostr/calendar_egress.rs` |
| Harness bridge | `terp_scripts::environments::nostr` (`calendar_meta_to_nostr`, `bind_and_bridge_offchain`) |
| Suite | `tests/nostr_orch_suite.rs` L0–L2 |

```bash
# L0 always
cargo test -p hash-market --lib content:: --features server
cargo test -p terp-scripts --test nostr_orch_suite

# L2 Docker (optional)
cargo test -p terp-scripts --test nostr_orch_suite -- --ignored --nocapture
```

### B3 — Ingress (optional, later)

Nostr → chain is **not** required for mesh design (commit stays on-chain). Defer unless product needs relay-originated events.

### B4 — FE ✅

Connect: HTTP query first, content-plane resolve for off-chain `cid`, Nostr subscribe progressive.  
`websites/terp-docs/lib/mesh/content.ts` + `nostr.ts` (kinds corrected: 31922 date / 31923 time).

---

## Explicit non-goals until A done

- Powering `nostr_orch_suite` as default CI  
- Second blob store for calendar only  
- Folding Skip Connect price oracle into blossom  
- Claiming headstash WSS already republishes dao-calendar creates  
- Putting **private** SEAM/claim note bodies on dual-index `/content/*` (use auth-gated
  `HeadstashStore` `/notes/{hs_id}/{addr}` — see `headstash.md`)

---

## Success criteria for “ready for calendar Nostr e2e”

- [x] `[distribution]` dual-index: upload → meta → get round-trip (`content::tests::dual_index_upload_meta_get_smoke`)
- [x] S3 webhook / register path coded (`POST /webhooks/s3`, collector fanout) — exercise with oline env when ready  
- [x] Documented `cid` encoding for dao-calendar off-chain events (`content::cid` + content-distribution.md)  
- [x] Capability matrix entry `content_distribution` with L0 commands  
- [x] No regression: mint host + VE with `distribution` off (default)  
- [x] MinioIpfsClient durable root hooks (local dir + distribution register)

**Phase A L0 green when:** `cargo test -p hash-market --lib content:: --features server` passes.

Then open B2 e2e PR against `nostr_orch_suite` + DaoCalendar.

---

## Cold-start prompt (next session)

```
Continue Blossom/BUD + S3-IPFS unification for hash-market content plane.
Read tools/hash-market/docs/content-distribution.md and
PREP-BLOSSOM-S3-CALENDAR-NOSTR.md.

Do not start calendar Nostr e2e until dual-index resolve smoke passes.
Invariants: sha256 primary, IPFS secondary, TreeStore sole BlobStore, distribution opt-in.
```
