//! Domain B pure seams + Tacit↔Terp private-bridge hinge (Round 1).
//!
//! No SP1, no halo2 — pure Rust encoding:
//! - H-1: only bridge-burn set membership authorizes mint (not generic spent set)
//! - A9 / conservation: `v_mint == v_burn`
//! - A2: tip + confirmation maturity + lag policy stub
//! - A6: destCommitment bind
//! - A13: once-per-ν / claim (double mint reject)
//! - A15 / A16: asset registry + domain_binding
//!
//! Round-1 API: [`authorize_bridge_mint`] maps a reflection snapshot + mint claim
//! into a SEAM-NOTE-OUT-compatible [`NoteOutSketch`] (or reject).

use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

/// Domain-separated tag for Terp ↔ Tacit asset mapping (SPEC §5.2).
pub const TERP_ASSET_DOMAIN_TAG: &[u8] = b"terp-tacit-asset-v1";

/// Domain tag for private-bridge binding (Domain C §3.1).
pub const TERP_PRIVATE_BRIDGE_DOMAIN_TAG: &[u8] = b"terp-private-bridge-v1";

/// Domain tag for SEAM-NOTE-OUT V0 (structural; matches `seam_note_out`).
pub const DOMAIN_TAG_NOTE_OUT: [u8; 16] = *b"terp-note-out-v0";

/// SEAM-NOTE-OUT origin: Tacit bridge mint.
pub const ORIGIN_BRIDGE_MINT: u8 = 0x02;
/// SEAM-NOTE-OUT nullifier domain: bridge-burn ν.
pub const NF_BRIDGE_BURN: u8 = 0x02;
/// SEAM-NOTE-OUT cm encoding: abstract harness leaf (no keccak crypto here).
pub const CM_ABSTRACT_LEAF_V0: u8 = 0x03;

/// Default confirmation depth K (REFLECTION_CONFIRMATIONS spirit; mainnet-like).
pub const DEFAULT_CONFIRMATIONS_K: u64 = 6;
/// Default max LC lag window (heights behind tip still allowed for mint).
pub const DEFAULT_MAX_LC_LAG: u64 = 64;

/// Opaque 32-byte digest / id.
pub type Hash32 = [u8; 32];

/// A bridge-burn claim that may authorize a destination mint (legacy pure gate).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BurnClaim {
    /// Re-derived binding: `H(dest_domain ‖ ν ‖ asset_id ‖ value)` (v0 derive).
    pub claim_id: Hash32,
    /// Nullifier of the burned source note.
    pub nu: Hash32,
    /// Opened / proven burn value (conservation witness side).
    pub value: u64,
    /// Foreign / mapped asset id (must resolve via registry for mint).
    pub asset_id: Hash32,
    /// Destination domain / chain binding (anti-replay across deployments).
    pub dest_domain: Hash32,
}

/// Bridge-burn set: only members of this set authorize mint (H-1 / A5).
#[derive(Clone, Debug, Default)]
pub struct BurnSet {
    /// Index by nullifier → burn claim record.
    by_nu: HashMap<Hash32, BurnClaim>,
}

impl BurnSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, claim: BurnClaim) {
        self.by_nu.insert(claim.nu, claim);
    }

    pub fn get(&self, nu: &Hash32) -> Option<&BurnClaim> {
        self.by_nu.get(nu)
    }

    pub fn contains_nu(&self, nu: &Hash32) -> bool {
        self.by_nu.contains_key(nu)
    }
}

/// Generic spent set (ordinary transfers/spends). Membership alone MUST NOT mint.
#[derive(Clone, Debug, Default)]
pub struct SpentSet {
    nullifiers: HashSet<Hash32>,
}

impl SpentSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, nu: Hash32) {
        self.nullifiers.insert(nu);
    }

    pub fn contains(&self, nu: &Hash32) -> bool {
        self.nullifiers.contains(nu)
    }
}

/// Destination-side once-per-burn tracker (`bridgeMinted[ν]` analogue).
#[derive(Clone, Debug, Default)]
pub struct MintedSet {
    nullifiers: HashSet<Hash32>,
}

impl MintedSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark(&mut self, nu: Hash32) {
        self.nullifiers.insert(nu);
    }

    pub fn contains(&self, nu: &Hash32) -> bool {
        self.nullifiers.contains(nu)
    }
}

/// Frozen registry: mapped `asset_id` → allowed for bridge mint.
#[derive(Clone, Debug, Default)]
pub struct AssetRegistry {
    mapped: HashSet<Hash32>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, asset_id: Hash32) {
        self.mapped.insert(asset_id);
    }

    pub fn is_mapped(&self, asset_id: &Hash32) -> bool {
        self.mapped.contains(asset_id)
    }
}

/// Rejection reasons (fail-closed mint gate — legacy pure path).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MintReject {
    /// ν not in bridge-burn set (A5 / H-1). Ordinary spend is not enough.
    NotInBurnSet,
    /// ν already minted on destination (A13).
    AlreadyMinted,
    /// LC tip not advanced / not trusted (A1/A3 posture).
    TipNotOk,
    /// Confirmation depth immature (A2).
    ImmatureConfirmation,
    /// `v_mint ≠ v_burn` or claim value disagrees with burn record (A9).
    ValueMismatch,
    /// Asset not in frozen domain map (A15).
    UnmappedAsset,
    /// Destination domain / chain binding mismatch (A16).
    DomainMismatch,
    /// claim_id does not re-derive from fields (A14) — optional seam.
    ClaimIdMismatch,
}

/// Rejection reasons for full hinge `authorize_bridge_mint` (Round 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BridgeMintError {
    /// ν not in bridge-burn set (A5 / H-1). Ordinary spent-set alone never mints.
    NotInBurnSet,
    /// Burned note not under source pool root (A7) — stub pin mismatch.
    NotInPoolRoot,
    /// ν already minted on destination (A13).
    AlreadyMinted,
    /// LC tip not advanced / not trusted (A1/A3).
    TipNotOk,
    /// Confirmation depth immature (A2).
    ImmatureConfirmation,
    /// Source height outside lag window (LC-STALE / lag policy stub).
    LagExceeded,
    /// Stale or zero burn root vs snapshot (A17).
    StaleBurnRoot,
    /// Stale pool root pin.
    StalePoolRoot,
    /// `v_mint ≠ v_burn` (A9).
    ValueMismatch,
    /// Asset not in frozen domain map (A15).
    UnmappedAsset,
    /// Destination domain / chain binding mismatch (A16).
    DomainMismatch,
    /// claim_id does not re-derive (A14).
    ClaimIdMismatch,
    /// destCommitment on claim ≠ burn-bound dest (A6 / T9 / LC-DOM-06).
    DestCommitmentMismatch,
    /// domain_binding public input does not re-derive (C §3.1).
    DomainBindingMismatch,
    /// Zero / empty-unauthorized burn root.
    ZeroBurnRoot,
    /// Oracle-only path rejected (A12).
    OracleOnlyRejected,
}

/// Inputs for a pure mint gate (no proof system).
#[derive(Clone, Debug)]
pub struct MintRequest {
    /// Claim presented for mint (must match burn-set record for `nu`).
    pub claim: BurnClaim,
    /// Value the destination wants to mint (must equal burn value).
    pub mint_value: u64,
    /// Expected destination domain for this deployment.
    pub expected_dest_domain: Hash32,
}

/// Domain-bind helper: stable Terp-side asset id from foreign parts.
///
/// `H( "terp-tacit-asset-v1" ‖ asset_id ‖ chain_id ‖ mixer )`
pub fn domain_bind(asset_id: &[u8], chain_id: &[u8], mixer: &[u8]) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(TERP_ASSET_DOMAIN_TAG);
    hasher.update(asset_id);
    hasher.update(chain_id);
    hasher.update(mixer);
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Re-derive claim_id = H(dest_domain ‖ nu ‖ asset_id ‖ value_be).
pub fn derive_claim_id(dest_domain: &Hash32, nu: &Hash32, asset_id: &Hash32, value: u64) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"terp-bridge-claim-v1");
    hasher.update(dest_domain);
    hasher.update(nu);
    hasher.update(asset_id);
    hasher.update(value.to_be_bytes());
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Pure mint gate encoding Domain B conservation / binding / burn≠spend.
///
/// Rules (all required):
/// 1. `tip_ok` — LC tip / client state accepts the source height band
/// 2. `conf_ok` — confirmation depth mature (REFLECTION_CONFIRMATIONS spirit)
/// 3. claim `nu` ∈ **burn_set** (not merely spent_set)
/// 4. `nu` not already in `minted` (once-per-burn)
/// 5. registry maps `asset_id`
/// 6. `claim.dest_domain == expected_dest_domain`
/// 7. burn record value == `mint_value` == claim.value (conservation)
/// 8. claim fields match burn-set record for that `nu`
///
/// **Explicit non-authority:** `spent_set` membership alone is never sufficient.
/// The spent set is accepted as an optional cross-check that a note was spent,
/// but mint authority is **only** burn-set membership.
pub fn mint_allowed(
    req: &MintRequest,
    burn_set: &BurnSet,
    _spent_set: &SpentSet,
    minted: &MintedSet,
    registry: &AssetRegistry,
    tip_ok: bool,
    conf_ok: bool,
) -> Result<(), MintReject> {
    if !tip_ok {
        return Err(MintReject::TipNotOk);
    }
    if !conf_ok {
        return Err(MintReject::ImmatureConfirmation);
    }

    let claim = &req.claim;

    // H-1 / A5: bridge-burn set only — spent_set is intentionally ignored for authority.
    let recorded = match burn_set.get(&claim.nu) {
        Some(c) => c,
        None => return Err(MintReject::NotInBurnSet),
    };

    if minted.contains(&claim.nu) {
        return Err(MintReject::AlreadyMinted);
    }

    if !registry.is_mapped(&claim.asset_id) {
        return Err(MintReject::UnmappedAsset);
    }

    if claim.dest_domain != req.expected_dest_domain
        || recorded.dest_domain != req.expected_dest_domain
    {
        return Err(MintReject::DomainMismatch);
    }

    // Conservation: presented mint value, claim value, and burn-set record must agree.
    if req.mint_value != claim.value
        || claim.value != recorded.value
        || req.mint_value != recorded.value
    {
        return Err(MintReject::ValueMismatch);
    }

    // Asset / nu / claim_id consistency vs burn-set record.
    if claim.asset_id != recorded.asset_id || claim.nu != recorded.nu {
        return Err(MintReject::ValueMismatch);
    }
    if claim.claim_id != recorded.claim_id {
        return Err(MintReject::ClaimIdMismatch);
    }
    let expected_id = derive_claim_id(&claim.dest_domain, &claim.nu, &claim.asset_id, claim.value);
    if claim.claim_id != expected_id {
        return Err(MintReject::ClaimIdMismatch);
    }

    Ok(())
}

/// Convenience: accept mint and mark ν consumed.
pub fn mint_apply(
    req: &MintRequest,
    burn_set: &BurnSet,
    spent_set: &SpentSet,
    minted: &mut MintedSet,
    registry: &AssetRegistry,
    tip_ok: bool,
    conf_ok: bool,
) -> Result<(), MintReject> {
    mint_allowed(req, burn_set, spent_set, minted, registry, tip_ok, conf_ok)?;
    minted.mark(req.claim.nu);
    Ok(())
}

// =============================================================================
// Round-1 hinge: reflection snapshot + BridgeMintPublic + authorize_bridge_mint
// =============================================================================

/// LC / reflection snapshot of mint-critical public values (class A).
///
/// Aligns with Tacit `BitcoinRelayPublicValues` mint-critical subset + Terp tip policy:
/// `(pool, spent, burn, height)` plus tip/conf/lag for A1–A4.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReflectionSnapshot {
    /// Current attested source pool root (`bitcoinPoolRoot`).
    pub pool_root: Hash32,
    /// Current attested spent root (`bitcoinSpentRoot`) — **never** mint authority alone.
    pub spent_root: Hash32,
    /// Current attested bridge-burn root (`bitcoinBurnRoot`) — **mint authority** (H-1).
    pub burn_root: Hash32,
    /// Confirmed source height H.
    pub source_height: u64,
    /// LC tip height (≥ source_height + K for mature mint).
    pub tip_height: u64,
    /// Confirmation depth parameter K.
    pub confirmations_k: u64,
    /// Max lag: mint only if `tip_height - source_height ≤ max` after K? Or
    /// `tip - claim_height ≤ max`. Policy: reject if `tip_height.saturating_sub(source_height) > max_lc_lag`
    /// *after* requiring maturity (tip ≥ source + K). Lag is additional freshness window.
    /// Stub: reject if `tip_height.saturating_sub(source_height + confirmations_k) > max_lc_lag`.
    pub max_lc_lag: u64,
    /// Client frozen (misbehaviour) — reject all mints.
    pub frozen: bool,
}

impl ReflectionSnapshot {
    /// True when tip is at least `source_height + confirmations_k`.
    pub fn conf_mature(&self) -> bool {
        self.tip_height >= self.source_height.saturating_add(self.confirmations_k)
    }

    /// Lag residual beyond the confirmation floor.
    pub fn lag_ok(&self) -> bool {
        if !self.conf_mature() {
            return false;
        }
        let residual = self
            .tip_height
            .saturating_sub(self.source_height.saturating_add(self.confirmations_k));
        residual <= self.max_lc_lag
    }

    pub fn tip_ok(&self) -> bool {
        !self.frozen && self.tip_height > 0 && self.burn_root != [0u8; 32]
    }
}

/// Mint-critical public statement presented to Terp (Domain B §9 + C §3.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BridgeMintPublic {
    /// Source chain tag string (e.g. `"bitcoin-mainnet"`, `"eip155:1"`).
    pub source_chain_tag: String,
    /// Foreign / Tacit asset id (32 bytes).
    pub tacit_asset_id: Hash32,
    /// Opened burn value (must equal v_burn).
    pub value_u64: u64,
    /// Nullifier ν of burned source note.
    pub nullifier: Hash32,
    /// Destination commitment bound by the burn record (A6).
    pub dest_commitment: Hash32,
    /// Destination chain / domain tag hash (Terp dest).
    pub dest_domain: Hash32,
    /// Re-derived claim id (A14).
    pub claim_id: Hash32,
    /// Membership pin: source pool root at burn time.
    pub source_pool_root: Hash32,
    /// Membership pin: source **burn** root (must equal current snapshot burn root for A17).
    pub source_burn_root: Hash32,
    /// Source height of the burn inclusion.
    pub source_height: u64,
    /// Domain binding digest (C §3.1).
    pub domain_binding: Hash32,
    /// Unit scale for asset map (optional pin; default 1).
    pub unit_scale: u64,
    /// Terp dest pool / note-set domain.
    pub pool_domain: Hash32,
    /// Destination leaf fingerprint to emit on note sketch (harness).
    pub cm_public: Hash32,
    /// Optional Pedersen / commitment blinding for DEX openings.
    /// All-zero → emit `rcm_flag = 0` (not DEX-consumable).
    pub rcm: Hash32,
}

/// Claim side for hinge: burn-set record + presented public statement.
#[derive(Clone, Debug)]
pub struct BridgeMintClaim {
    /// Full mint-critical public packet.
    pub public: BridgeMintPublic,
    /// Value the destination wants to mint (must equal burn / public value).
    pub mint_value: u64,
    /// Expected dest domain for this Terp deployment.
    pub expected_dest_domain: Hash32,
    /// Expected domain_binding re-derive inputs (src/dst chain tags as bytes).
    pub src_chain_id: Hash32,
    pub dst_chain_id: Hash32,
    pub lc_client_id: Hash32,
    /// Burn-bound destCommitment recorded in the bridge-burn set (A6 authority).
    pub burn_dest_commitment: Hash32,
    /// True if ν is present in the **bridge-burn set** under snapshot.burn_root (mock LC).
    pub in_burn_set: bool,
    /// True if burned note membership under pool root holds (mock LC).
    pub in_pool_root: bool,
    /// True if ν is only in spent set (for H-1 explicit reject tests).
    pub spent_only: bool,
}

/// SEAM-NOTE-OUT-compatible note sketch produced by an authorized bridge mint.
///
/// Field roles align with `SeamNoteOutV0` (Domain R3-A / SEAM-NOTE-OUT.md).
/// Round-2 fills `rcm` / `rcm_flag` / `memo` / `memo_flag` so the sketch is
/// structurally DEX-consumable when `rcm_flag == 1` (compose C7).
/// Fixed layout remains 382 bytes when serialized via [`NoteOutSketch::to_seam_bytes`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteOutSketch {
    pub version: u8,
    pub domain_tag: [u8; 16],
    pub origin: u8,
    pub asset_id: Hash32,
    pub value: u64,
    pub owner_binding: Hash32,
    pub cm_public: Hash32,
    pub cm_encoding: u8,
    pub nullifier_lineage: Hash32,
    pub nullifier_domain: u8,
    pub provenance_anchor: Hash32,
    pub claim_id: Hash32,
    pub source_chain_tag_hash: Hash32,
    /// Commitment trapdoor / blinding; zero if `rcm_flag == 0`.
    pub rcm: Hash32,
    /// `0` = rcm not supplied; `1` = rcm present (required for DEX consumability).
    pub rcm_flag: u8,
    /// Optional memo ciphertext / tag; zero-filled when unused.
    pub memo: [u8; 64],
    /// `0` = no memo; `1` = memo bytes meaningful.
    pub memo_flag: u8,
    pub pool_domain: Hash32,
}

impl NoteOutSketch {
    /// Structural DEX consumability (SEAM §4.3 spirit): valid origin/nf pair,
    /// non-zero asset/cm, and `rcm_flag == 1`.
    pub fn is_dex_consumable(&self) -> bool {
        if self.version != 0 {
            return false;
        }
        if self.domain_tag != DOMAIN_TAG_NOTE_OUT {
            return false;
        }
        if self.origin != ORIGIN_BRIDGE_MINT || self.nullifier_domain != NF_BRIDGE_BURN {
            return false;
        }
        if self.cm_encoding == 0 {
            return false;
        }
        if self.asset_id == [0u8; 32] || self.cm_public == [0u8; 32] {
            return false;
        }
        if self.rcm_flag != 1 {
            return false;
        }
        if self.rcm_flag > 1 || self.memo_flag > 1 {
            return false;
        }
        true
    }

    /// Canonical 382-byte SEAM-NOTE-OUT V0 serialization order.
    pub fn to_seam_bytes(&self) -> [u8; 382] {
        let mut out = [0u8; 382];
        let mut o = 0usize;
        out[o] = self.version;
        o += 1;
        out[o..o + 16].copy_from_slice(&self.domain_tag);
        o += 16;
        out[o] = self.origin;
        o += 1;
        out[o..o + 32].copy_from_slice(&self.asset_id);
        o += 32;
        out[o..o + 8].copy_from_slice(&self.value.to_le_bytes());
        o += 8;
        out[o..o + 32].copy_from_slice(&self.owner_binding);
        o += 32;
        out[o..o + 32].copy_from_slice(&self.cm_public);
        o += 32;
        out[o] = self.cm_encoding;
        o += 1;
        out[o..o + 32].copy_from_slice(&self.nullifier_lineage);
        o += 32;
        out[o] = self.nullifier_domain;
        o += 1;
        out[o..o + 32].copy_from_slice(&self.provenance_anchor);
        o += 32;
        out[o..o + 32].copy_from_slice(&self.claim_id);
        o += 32;
        out[o..o + 32].copy_from_slice(&self.source_chain_tag_hash);
        o += 32;
        out[o..o + 32].copy_from_slice(&self.rcm);
        o += 32;
        out[o] = self.rcm_flag;
        o += 1;
        out[o..o + 64].copy_from_slice(&self.memo);
        o += 64;
        out[o] = self.memo_flag;
        o += 1;
        out[o..o + 32].copy_from_slice(&self.pool_domain);
        o += 32;
        debug_assert_eq!(o, 382);
        out
    }
}

/// Re-derive claim_id including destCommitment (B A14 / SPEC §3.1):
/// `H("terp-bridge-claim-v1" ‖ dest_domain ‖ dest_commitment ‖ ν ‖ asset_id ‖ value_be)`.
pub fn derive_claim_id_with_dest(
    dest_domain: &Hash32,
    dest_commitment: &Hash32,
    nu: &Hash32,
    asset_id: &Hash32,
    value: u64,
) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(b"terp-bridge-claim-v1");
    hasher.update(dest_domain);
    hasher.update(dest_commitment);
    hasher.update(nu);
    hasher.update(asset_id);
    hasher.update(value.to_be_bytes());
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Domain binding (C §3.1):
/// `H("terp-private-bridge-v1" ‖ src ‖ dst ‖ lc_client_id ‖ asset_id ‖ claim_or_ν ‖ height_be ‖ commitment_root)`.
pub fn derive_domain_binding(
    src_chain_id: &Hash32,
    dst_chain_id: &Hash32,
    lc_client_id: &Hash32,
    asset_id: &Hash32,
    claim_or_nu: &Hash32,
    consensus_height: u64,
    commitment_root: &Hash32,
) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(TERP_PRIVATE_BRIDGE_DOMAIN_TAG);
    hasher.update(src_chain_id);
    hasher.update(dst_chain_id);
    hasher.update(lc_client_id);
    hasher.update(asset_id);
    hasher.update(claim_or_nu);
    hasher.update(consensus_height.to_be_bytes());
    hasher.update(commitment_root);
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Hash a source-chain tag string to 32 bytes.
pub fn hash_source_chain_tag(tag: &str) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(tag.as_bytes());
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Map foreign asset into Terp asset id (B §5.2 spirit; SHA-256 fixture-stable).
pub fn terp_asset_id_from_tacit(
    source_chain_tag: &str,
    tacit_asset_id: &Hash32,
    unit_scale: u64,
) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(TERP_ASSET_DOMAIN_TAG);
    hasher.update(source_chain_tag.as_bytes());
    hasher.update(tacit_asset_id);
    hasher.update(unit_scale.to_be_bytes());
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Pure hinge: authorize a bridge mint from reflection snapshot + claim.
///
/// Accept/reject gates align with Domain B T1–T7 / A1–A17 and Domain C H-1:
/// 1. Snapshot tip_ok, not frozen
/// 2. Confirmation maturity (A2)
/// 3. Lag window stub
/// 4. Burn root non-zero + pin matches current snapshot (A17)
/// 5. Pool root pin matches snapshot
/// 6. **H-1:** `in_burn_set` required; `spent_only` without burn → reject
/// 7. Pool membership pin (A7 stub)
/// 8. Once-per-ν (A13)
/// 9. Asset registry map (A15)
/// 10. Dest domain match (A16)
/// 11. destCommitment bind (A6)
/// 12. claim_id re-derive (A14)
/// 13. domain_binding re-derive (C §3.1)
/// 14. Conservation v_mint == value (A9)
///
/// On accept: returns [`NoteOutSketch`] (SEAM-NOTE-OUT origin bridge) without
/// mutating `minted` — call [`minted.mark`] after success if applying state.
pub fn authorize_bridge_mint(
    snapshot: &ReflectionSnapshot,
    claim: &BridgeMintClaim,
    minted: &MintedSet,
    registry: &AssetRegistry,
) -> Result<NoteOutSketch, BridgeMintError> {
    let p = &claim.public;

    // A1/A3 tip / freeze
    if !snapshot.tip_ok() {
        if snapshot.frozen || snapshot.tip_height == 0 {
            return Err(BridgeMintError::TipNotOk);
        }
        if snapshot.burn_root == [0u8; 32] {
            return Err(BridgeMintError::ZeroBurnRoot);
        }
        return Err(BridgeMintError::TipNotOk);
    }
    if snapshot.burn_root == [0u8; 32] {
        return Err(BridgeMintError::ZeroBurnRoot);
    }

    // Height currency: claim source height must match snapshot band
    if p.source_height != snapshot.source_height {
        // Allow claim height ≤ snapshot.source_height only if still conf-mature at tip.
        // Stub: require exact pin to current attested height for simplicity.
        return Err(BridgeMintError::StaleBurnRoot);
    }

    // A2 confirmations
    if !snapshot.conf_mature() {
        return Err(BridgeMintError::ImmatureConfirmation);
    }
    // Lag policy stub (LC-STALE-02)
    if !snapshot.lag_ok() {
        return Err(BridgeMintError::LagExceeded);
    }

    // A17 burn root pin
    if p.source_burn_root != snapshot.burn_root {
        return Err(BridgeMintError::StaleBurnRoot);
    }
    // Pool root pin
    if p.source_pool_root != snapshot.pool_root {
        return Err(BridgeMintError::StalePoolRoot);
    }

    // H-1 / A5: burn set membership is authority. spent_only alone rejects.
    if claim.spent_only && !claim.in_burn_set {
        return Err(BridgeMintError::NotInBurnSet);
    }
    if !claim.in_burn_set {
        return Err(BridgeMintError::NotInBurnSet);
    }
    // A7 pool membership stub
    if !claim.in_pool_root {
        return Err(BridgeMintError::NotInPoolRoot);
    }

    // A13 once-per-burn
    if minted.contains(&p.nullifier) {
        return Err(BridgeMintError::AlreadyMinted);
    }

    // Mapped Terp asset
    let terp_asset = terp_asset_id_from_tacit(&p.source_chain_tag, &p.tacit_asset_id, p.unit_scale);
    if !registry.is_mapped(&terp_asset) {
        // Also accept if registry maps the raw tacit id (fixture flexibility).
        if !registry.is_mapped(&p.tacit_asset_id) {
            return Err(BridgeMintError::UnmappedAsset);
        }
    }
    let asset_for_note = if registry.is_mapped(&terp_asset) {
        terp_asset
    } else {
        p.tacit_asset_id
    };

    // A16 domain
    if p.dest_domain != claim.expected_dest_domain {
        return Err(BridgeMintError::DomainMismatch);
    }

    // A6 destCommitment: claim public must match burn-bound dest
    if p.dest_commitment != claim.burn_dest_commitment {
        return Err(BridgeMintError::DestCommitmentMismatch);
    }

    // A9 conservation
    if claim.mint_value != p.value_u64 {
        return Err(BridgeMintError::ValueMismatch);
    }

    // A14 claim_id re-derive
    let expected_claim = derive_claim_id_with_dest(
        &p.dest_domain,
        &p.dest_commitment,
        &p.nullifier,
        &p.tacit_asset_id,
        p.value_u64,
    );
    if p.claim_id != expected_claim {
        return Err(BridgeMintError::ClaimIdMismatch);
    }

    // C §3.1 domain_binding
    let expected_binding = derive_domain_binding(
        &claim.src_chain_id,
        &claim.dst_chain_id,
        &claim.lc_client_id,
        &p.tacit_asset_id,
        &p.nullifier,
        p.source_height,
        &p.source_burn_root,
    );
    if p.domain_binding != expected_binding {
        return Err(BridgeMintError::DomainBindingMismatch);
    }

    // Emit SEAM-NOTE-OUT sketch (origin bridge mint).
    // Round-2: rcm present when non-zero → rcm_flag=1 (DEX-consumable).
    let (rcm, rcm_flag) = if p.rcm != [0u8; 32] {
        (p.rcm, 1u8)
    } else {
        ([0u8; 32], 0u8)
    };

    Ok(NoteOutSketch {
        version: 0,
        domain_tag: DOMAIN_TAG_NOTE_OUT,
        origin: ORIGIN_BRIDGE_MINT,
        asset_id: asset_for_note,
        value: p.value_u64,
        owner_binding: p.dest_commitment,
        cm_public: p.cm_public,
        cm_encoding: CM_ABSTRACT_LEAF_V0,
        nullifier_lineage: p.nullifier,
        nullifier_domain: NF_BRIDGE_BURN,
        provenance_anchor: p.source_burn_root,
        claim_id: p.claim_id,
        source_chain_tag_hash: hash_source_chain_tag(&p.source_chain_tag),
        rcm,
        rcm_flag,
        memo: [0u8; 64],
        memo_flag: 0,
        pool_domain: p.pool_domain,
    })
}

/// Apply authorized mint: run gate then mark ν consumed.
pub fn authorize_bridge_mint_apply(
    snapshot: &ReflectionSnapshot,
    claim: &BridgeMintClaim,
    minted: &mut MintedSet,
    registry: &AssetRegistry,
) -> Result<NoteOutSketch, BridgeMintError> {
    let note = authorize_bridge_mint(snapshot, claim, minted, registry)?;
    minted.mark(claim.public.nullifier);
    Ok(note)
}

// =============================================================================
// Public harness fixtures (L0 re-export for PrivateBridgeSuite / demo-e2e-l0)
// =============================================================================

/// Stable 32-byte label hash for pure fixtures (SHA-256 of label bytes).
pub fn label_hash(label: &str) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    let out = hasher.finalize();
    let mut id = [0u8; 32];
    id.copy_from_slice(&out);
    id
}

/// Happy-path hinge world: mature tip, mapped asset, burn-set member, empty minted set.
///
/// Used by suite L0 methods (`assert_policy_bridge_mint_happy`, H-1, double mint)
/// so harness and `bridge_auth_seams` unit tests share one world builder.
pub fn hinge_happy_fixture() -> (
    ReflectionSnapshot,
    BridgeMintClaim,
    MintedSet,
    AssetRegistry,
    Hash32, // terp_asset
) {
    let dest = label_hash("terp-chain-1");
    let tacit_asset = label_hash("tacit-btc-etch-1");
    let unit_scale = 1u64;
    let terp_asset = terp_asset_id_from_tacit("bitcoin-mainnet", &tacit_asset, unit_scale);
    let nu = label_hash("nu-hinge-happy");
    let dest_cm = label_hash("dest-commitment-A");
    let pool_root = label_hash("pool-root-1");
    let spent_root = label_hash("spent-root-1");
    let burn_root = label_hash("burn-root-1");
    let height = 100u64;
    let tip = height + DEFAULT_CONFIRMATIONS_K;
    let claim_id = derive_claim_id_with_dest(&dest, &dest_cm, &nu, &tacit_asset, 1_000_000);
    let src_chain = label_hash("src-bitcoin-mainnet");
    let dst_chain = dest;
    let lc_client = label_hash("lc-client-reflection-0");
    let domain_binding = derive_domain_binding(
        &src_chain,
        &dst_chain,
        &lc_client,
        &tacit_asset,
        &nu,
        height,
        &burn_root,
    );

    let snapshot = ReflectionSnapshot {
        pool_root,
        spent_root,
        burn_root,
        source_height: height,
        tip_height: tip,
        confirmations_k: DEFAULT_CONFIRMATIONS_K,
        max_lc_lag: DEFAULT_MAX_LC_LAG,
        frozen: false,
    };

    let public = BridgeMintPublic {
        source_chain_tag: "bitcoin-mainnet".into(),
        tacit_asset_id: tacit_asset,
        value_u64: 1_000_000,
        nullifier: nu,
        dest_commitment: dest_cm,
        dest_domain: dest,
        claim_id,
        source_pool_root: pool_root,
        source_burn_root: burn_root,
        source_height: height,
        domain_binding,
        unit_scale,
        pool_domain: label_hash("terp-pool-0"),
        cm_public: label_hash("cm-leaf-dest-1"),
        // Phase-1 opening material so happy path is DEX-consumable (C7 / D2).
        rcm: label_hash("rcm-hinge-happy"),
    };

    let claim = BridgeMintClaim {
        public,
        mint_value: 1_000_000,
        expected_dest_domain: dest,
        src_chain_id: src_chain,
        dst_chain_id: dst_chain,
        lc_client_id: lc_client,
        burn_dest_commitment: dest_cm,
        in_burn_set: true,
        in_pool_root: true,
        spent_only: false,
    };

    let mut registry = AssetRegistry::new();
    registry.register(terp_asset);

    (snapshot, claim, MintedSet::new(), registry, terp_asset)
}

// =============================================================================
// Contract surface sketch (types only — future `cw-bridge-mint` / headstash ext)
// =============================================================================

/// CosmWasm-style instantiate message sketch (not a deployable contract).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstantiateMsgSketch {
    /// Terp destination domain / chain bind.
    pub dest_domain: Hash32,
    /// Confirmation depth K.
    pub confirmations_k: u64,
    /// Max LC lag for mint freshness.
    pub max_lc_lag: u64,
    /// Optional LC client id for reflection corridor.
    pub lc_client_id: String,
    /// Admin for registry register-once (governance or empty = immutable init set).
    pub admin: Option<String>,
}

/// Execute messages sketch for bridge mint surface.
///
/// Round-2 production surface lives on **`cw-headstash`** (not a freestanding
/// `cw-bridge-mint`). See `crates/headstash/contracts/cw-headstash/src/msg.rs`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecuteMsgSketch {
    /// Advance reflection / LC attested roots (who posts LC updates — ops Q).
    UpdateReflection {
        pool_root: Hash32,
        spent_root: Hash32,
        burn_root: Hash32,
        source_height: u64,
        tip_height: u64,
    },
    /// Full reflection snapshot replace (K, lag, frozen).
    SetReflectionSnapshot {
        pool_root: Hash32,
        spent_root: Hash32,
        burn_root: Hash32,
        source_height: u64,
        tip_height: u64,
        confirmations_k: u64,
        max_lc_lag: u64,
        frozen: bool,
    },
    /// Register foreign asset once (A15 registry) — internal map.
    RegisterAsset {
        asset_id: Hash32,
        local_denom: String,
        origin: Option<String>,
        status: String, // "active" | "paused" | "frozen"
    },
    /// Optional external registry contract address.
    SetExternalAssetRegistry {
        addr: Option<String>,
    },
    /// One-shot bridge mint → SEAM-NOTE-OUT note append (proof bytes opaque / mock).
    BridgeMintNote {
        public: BridgeMintPublic,
        /// Opaque membership / reflection receipt (mock or SP1 later).
        proof: Vec<u8>,
    },
    /// Freeze corridor (misbehaviour / gov).
    Freeze {},
}

/// Query sketch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryMsgSketch {
    ReflectionTip {},
    IsMinted { nullifier: Hash32 },
    AssetMapped {
        source_chain_tag: String,
        tacit_asset_id: Hash32,
    },
    Config {},
}

/// Logical LC ingress certificate (Domain C §6.1) — types only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LcIngressCertificateSketch {
    pub class: String, // "Reflection" | "Crosslink" | ...
    pub client_id: String,
    pub height: u64,
    pub pool_root: Hash32,
    pub spent_root: Hash32,
    pub burn_root: Hash32,
    pub domain_binding: Hash32,
    pub proof_kind: String, // "BurnSetMembership" | "ReflectionReceipt"
    pub value: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(label: &str) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update(label.as_bytes());
        let out = hasher.finalize();
        let mut id = [0u8; 32];
        id.copy_from_slice(&out);
        id
    }

    fn sample_claim(nu_label: &str, value: u64, asset: Hash32, dest: Hash32) -> BurnClaim {
        let nu = h(nu_label);
        let claim_id = derive_claim_id(&dest, &nu, &asset, value);
        BurnClaim {
            claim_id,
            nu,
            value,
            asset_id: asset,
            dest_domain: dest,
        }
    }

    fn fixture_world() -> (
        BurnClaim,
        BurnSet,
        SpentSet,
        MintedSet,
        AssetRegistry,
        Hash32,
    ) {
        let dest = h("terp-chain-1");
        let asset = domain_bind(b"btc-etch-asset", b"bitcoin-mainnet", b"pool-genesis-0");
        let claim = sample_claim("nu-happy-1", 1_000_000, asset, dest);

        let mut burn_set = BurnSet::new();
        burn_set.insert(claim.clone());

        // Ordinary spends may share the spent set, but that alone never mints.
        let mut spent_set = SpentSet::new();
        spent_set.insert(claim.nu);
        spent_set.insert(h("nu-ordinary-only"));

        let minted = MintedSet::new();
        let mut registry = AssetRegistry::new();
        registry.register(asset);

        (claim, burn_set, spent_set, minted, registry, dest)
    }

    /// T1 — Happy-path mint: burn membership + tip + conf + conservation + mapped asset.
    #[test]
    fn t1_happy_path_mint() {
        let (claim, burn_set, spent_set, mut minted, registry, dest) = fixture_world();
        let req = MintRequest {
            claim: claim.clone(),
            mint_value: claim.value,
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_apply(&req, &burn_set, &spent_set, &mut minted, &registry, true, true),
            Ok(())
        );
        assert!(minted.contains(&claim.nu));
    }

    /// T2 — Double mint / double-claim: same ν after first accept → reject A13.
    #[test]
    fn t2_double_mint_reject() {
        let (claim, burn_set, spent_set, mut minted, registry, dest) = fixture_world();
        let req = MintRequest {
            claim: claim.clone(),
            mint_value: claim.value,
            expected_dest_domain: dest,
        };
        mint_apply(&req, &burn_set, &spent_set, &mut minted, &registry, true, true)
            .expect("first mint");
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::AlreadyMinted)
        );
    }

    /// T3 — Ordinary spend is not a burn: ν in spent_set only → reject A5 / H-1.
    #[test]
    fn t3_ordinary_spend_not_burn() {
        let (_claim, burn_set, mut spent_set, minted, registry, dest) = fixture_world();
        let asset = domain_bind(b"btc-etch-asset", b"bitcoin-mainnet", b"pool-genesis-0");
        // A note that was spent ordinarily, never bridge-burned.
        let ordinary = sample_claim("nu-ordinary-only", 50, asset, dest);
        spent_set.insert(ordinary.nu);
        assert!(spent_set.contains(&ordinary.nu));
        assert!(!burn_set.contains_nu(&ordinary.nu));

        let req = MintRequest {
            claim: ordinary,
            mint_value: 50,
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::NotInBurnSet)
        );
    }

    /// T4 — Immature confirmation: tip may be ok but conf depth not mature → A2.
    #[test]
    fn t4_immature_confirmation() {
        let (claim, burn_set, spent_set, minted, registry, dest) = fixture_world();
        let req = MintRequest {
            claim: claim.clone(),
            mint_value: claim.value,
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, false),
            Err(MintReject::ImmatureConfirmation)
        );
        // Tip failure is also reject (separate from conf).
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, false, true),
            Err(MintReject::TipNotOk)
        );
    }

    /// T5 — Value mismatch: witness tries to mint ≠ burn value → A9.
    #[test]
    fn t5_value_mismatch() {
        let (claim, burn_set, spent_set, minted, registry, dest) = fixture_world();
        let req = MintRequest {
            claim: claim.clone(),
            mint_value: claim.value + 1, // inflate
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::ValueMismatch)
        );
    }

    /// T6 — Unmapped asset: valid burn shape but asset not in Terp registry → A15.
    #[test]
    fn t6_unmapped_asset() {
        let dest = h("terp-chain-1");
        let unknown = domain_bind(b"unknown-asset", b"eip155:1", b"pool-x");
        let claim = sample_claim("nu-unmapped", 100, unknown, dest);
        let mut burn_set = BurnSet::new();
        burn_set.insert(claim.clone());
        let spent_set = SpentSet::new();
        let minted = MintedSet::new();
        let registry = AssetRegistry::new(); // empty — nothing mapped

        let req = MintRequest {
            claim,
            mint_value: 100,
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::UnmappedAsset)
        );
    }

    /// T7 — Domain mismatch: claim bound to chain/pool X submitted to Y → A16.
    #[test]
    fn t7_domain_mismatch() {
        let (claim, burn_set, spent_set, minted, registry, _dest) = fixture_world();
        let wrong_dest = h("terp-chain-OTHER");
        let req = MintRequest {
            claim: claim.clone(),
            mint_value: claim.value,
            expected_dest_domain: wrong_dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::DomainMismatch)
        );
    }

    #[test]
    fn domain_bind_is_domain_separated() {
        let a = domain_bind(b"asset", b"bitcoin-mainnet", b"mixer-0");
        let b = domain_bind(b"asset", b"eip155:1", b"mixer-0");
        let c = domain_bind(b"asset", b"bitcoin-mainnet", b"mixer-1");
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        // Deterministic
        assert_eq!(a, domain_bind(b"asset", b"bitcoin-mainnet", b"mixer-0"));
    }

    #[test]
    fn spent_set_membership_alone_never_authorizes() {
        // Explicit invariant: burn_set empty, spent_set has ν → still reject.
        let dest = h("terp-chain-1");
        let asset = domain_bind(b"btc-etch-asset", b"bitcoin-mainnet", b"pool-genesis-0");
        let claim = sample_claim("nu-spent-only", 9, asset, dest);
        let burn_set = BurnSet::new();
        let mut spent_set = SpentSet::new();
        spent_set.insert(claim.nu);
        let mut registry = AssetRegistry::new();
        registry.register(asset);
        let minted = MintedSet::new();
        let req = MintRequest {
            claim,
            mint_value: 9,
            expected_dest_domain: dest,
        };
        assert_eq!(
            mint_allowed(&req, &burn_set, &spent_set, &minted, &registry, true, true),
            Err(MintReject::NotInBurnSet)
        );
    }

    // -------------------------------------------------------------------------
    // Round-1 hinge helpers + T8–T11-class tests
    // -------------------------------------------------------------------------
    // Happy world builder: public `hinge_happy_fixture()` (shared with suite L0).

    /// Hinge happy path → NoteOutSketch (SEAM-NOTE-OUT origin bridge).
    #[test]
    fn hinge_t1_authorize_bridge_mint_happy() {
        let (snapshot, claim, mut minted, registry, terp_asset) = hinge_happy_fixture();
        let note = authorize_bridge_mint_apply(&snapshot, &claim, &mut minted, &registry)
            .expect("happy mint");
        assert_eq!(note.origin, ORIGIN_BRIDGE_MINT);
        assert_eq!(note.nullifier_domain, NF_BRIDGE_BURN);
        assert_eq!(note.value, 1_000_000);
        assert_eq!(note.asset_id, terp_asset);
        assert_eq!(note.owner_binding, claim.public.dest_commitment);
        assert_eq!(note.provenance_anchor, snapshot.burn_root);
        assert_eq!(note.nullifier_lineage, claim.public.nullifier);
        assert_eq!(note.rcm_flag, 1);
        assert_eq!(note.rcm, claim.public.rcm);
        assert_eq!(note.memo_flag, 0);
        assert!(note.is_dex_consumable());
        assert_eq!(note.to_seam_bytes().len(), 382);
        assert!(minted.contains(&claim.public.nullifier));
    }

    /// Zero rcm on public → rcm_flag=0 → not DEX-consumable (phase-0 posture).
    #[test]
    fn hinge_rcm_absent_not_dex_consumable() {
        let (snapshot, mut claim, mut minted, registry, _) = hinge_happy_fixture();
        claim.public.rcm = [0u8; 32];
        let note = authorize_bridge_mint_apply(&snapshot, &claim, &mut minted, &registry)
            .expect("mint still ok without rcm");
        assert_eq!(note.rcm_flag, 0);
        assert_eq!(note.rcm, [0u8; 32]);
        assert!(!note.is_dex_consumable());
    }

    /// T9 / A6 / LC-DOM-06 — destCommitment mismatch rejects.
    #[test]
    fn hinge_dest_commitment_bind_reject() {
        let (snapshot, mut claim, minted, registry, _) = hinge_happy_fixture();
        // Mint tries leaf/dest B while burn pins A.
        claim.public.dest_commitment = h("dest-commitment-B");
        // Re-derive claim_id for the *wrong* dest so we isolate A6 not A14.
        claim.public.claim_id = derive_claim_id_with_dest(
            &claim.public.dest_domain,
            &claim.public.dest_commitment,
            &claim.public.nullifier,
            &claim.public.tacit_asset_id,
            claim.public.value_u64,
        );
        // burn_dest_commitment stays A → A6 fires.
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::DestCommitmentMismatch)
        );
    }

    /// Lag policy stub: tip far beyond source+K+max_lc_lag → LagExceeded.
    #[test]
    fn hinge_lag_policy_stub_reject() {
        let (mut snapshot, claim, minted, registry, _) = hinge_happy_fixture();
        // conf mature but residual lag > max
        snapshot.max_lc_lag = 2;
        snapshot.tip_height = snapshot.source_height
            + snapshot.confirmations_k
            + snapshot.max_lc_lag
            + 1;
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::LagExceeded)
        );
    }

    /// Immature confirmation still rejects (A2) even if lag max is large.
    #[test]
    fn hinge_immature_confirmation_reject() {
        let (mut snapshot, claim, minted, registry, _) = hinge_happy_fixture();
        snapshot.tip_height = snapshot.source_height + snapshot.confirmations_k - 1;
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::ImmatureConfirmation)
        );
    }

    /// Domain binding mismatch (C §3.1) rejects.
    #[test]
    fn hinge_domain_binding_reject() {
        let (snapshot, mut claim, minted, registry, _) = hinge_happy_fixture();
        claim.public.domain_binding = h("wrong-domain-binding");
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::DomainBindingMismatch)
        );
    }

    /// H-1 on hinge: spent_only without burn set → reject.
    #[test]
    fn hinge_h1_spent_only_reject() {
        let (snapshot, mut claim, minted, registry, _) = hinge_happy_fixture();
        claim.in_burn_set = false;
        claim.spent_only = true;
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::NotInBurnSet)
        );
    }

    /// Stale burn root pin (A17).
    #[test]
    fn hinge_stale_burn_root_reject() {
        let (snapshot, mut claim, minted, registry, _) = hinge_happy_fixture();
        claim.public.source_burn_root = h("old-burn-root");
        // domain_binding still pins old happy burn root — expect StaleBurnRoot first
        // (order: root pin checked before domain_binding re-derive).
        assert_eq!(
            authorize_bridge_mint(&snapshot, &claim, &minted, &registry),
            Err(BridgeMintError::StaleBurnRoot)
        );
    }

    /// Contract surface sketch types construct (compile + basic equality).
    #[test]
    fn contract_surface_sketch_constructs() {
        let inst = InstantiateMsgSketch {
            dest_domain: h("terp-chain-1"),
            confirmations_k: DEFAULT_CONFIRMATIONS_K,
            max_lc_lag: DEFAULT_MAX_LC_LAG,
            lc_client_id: "08-wasm-tacit-reflection-0".into(),
            admin: None,
        };
        let exec = ExecuteMsgSketch::BridgeMintNote {
            public: hinge_happy_fixture().1.public,
            proof: vec![0xde, 0xad],
        };
        match exec {
            ExecuteMsgSketch::BridgeMintNote { public, proof } => {
                assert_eq!(public.dest_domain, inst.dest_domain);
                assert_eq!(proof.len(), 2);
            }
            _ => panic!("expected BridgeMintNote"),
        }
    }
}
