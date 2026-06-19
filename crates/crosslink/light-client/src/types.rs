// //! Core Zcash Crosslink types for the light client.
// //!
// //! These types mirror the zebra-crosslink chain types but are self-contained,
// //! avoiding the heavy zebra-chain dependency. They are designed to be
// //! serializable and usable in a CosmWasm contract.
// //!
// //! Note: Fixed-size arrays larger than 32 bytes use `Vec<u8>` instead of
// //! `[u8; N]` because serde only supports `[u8; N]` up to N=32.

// use serde::{Deserialize, Serialize};

// /// A BLAKE3 hash (32 bytes).
// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct Blake3Hash(pub [u8; 32]);

// impl std::fmt::Debug for Blake3Hash {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         for b in &self.0 {
//             write!(f, "{:02x}", b)?;
//         }
//         Ok(())
//     }
// }

// /// Zcash Crosslink protocol parameters.
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct ZcashCrosslinkParameters {
//     /// The best-chain confirmation depth, σ.
//     pub bc_confirmation_depth_sigma: u64,
//     /// The depth of unfinalized PoW blocks past which "Stalled Mode" activates, L.
//     pub finalization_gap_bound: u64,
// }

// /// Crosslink parameters chosen for prototyping / testing.
// pub const PROTOTYPE_PARAMETERS: ZcashCrosslinkParameters = ZcashCrosslinkParameters {
//     bc_confirmation_depth_sigma: 3,
//     finalization_gap_bound: 7,
// };

// /// A minimal PoW block header for the light client.
// ///
// /// Contains only the fields needed for IBC verification:
// /// the block hash and timestamp.
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct PowHeader {
//     /// The block hash (double-SHA256).
//     pub hash: [u8; 32],
//     /// Unix timestamp (seconds) of the block.
//     pub timestamp: u64,
//     /// The block height.
//     pub height: u32,
// }

// /// A vote signature for a BFT block.
// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct FatPointerSignature2 {
//     /// Ed25519 public key (32 bytes).
//     pub public_key: [u8; 32],
//     /// Ed25519 signature (64 bytes, stored as Vec<u8> for serde compat).
//     pub vote_signature: Vec<u8>,
// }

// /// A bundle of signed votes for a block (Fat Pointer).
// ///
// /// Contains the vote template (with the block hash embedded) and the
// /// aggregated signatures from the TFL finalizer set.
// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct FatPointerToBftBlock2 {
//     /// The vote template without the finalizer public key (44 bytes, stored as Vec<u8>).
//     /// First 32 bytes = the BLAKE3 hash of the BFT block being voted on.
//     pub vote_for_block_without_finalizer_public_key: Vec<u8>,
//     /// The aggregated signatures from the TFL finalizer set.
//     pub signatures: Vec<FatPointerSignature2>,
// }

// impl FatPointerToBftBlock2 {
//     /// Create a null (empty) fat pointer.
//     #[must_use]
//     pub fn null() -> Self {
//         Self {
//             vote_for_block_without_finalizer_public_key: vec![0u8; 44],
//             signatures: Vec::new(),
//         }
//     }

//     /// Extract the BLAKE3 hash of the block this fat pointer votes for.
//     #[must_use]
//     pub fn points_at_block_hash(&self) -> Blake3Hash {
//         if self.vote_for_block_without_finalizer_public_key.len() < 32 {
//             return Blake3Hash([0u8; 32]);
//         }
//         let mut hash = [0u8; 32];
//         hash.copy_from_slice(&self.vote_for_block_without_finalizer_public_key[0..32]);
//         Blake3Hash(hash)
//     }

//     /// Batch-verify all ed25519 signatures in the fat pointer.
//     ///
//     /// Returns `true` if all signatures are valid.
//     #[must_use]
//     pub fn validate_signatures(&self) -> bool {
//         use ed25519_zebra::batch::Verifier;
//         use ed25519_zebra::VerificationKeyBytes;

//         if self.signatures.is_empty() {
//             return false;
//         }

//         if self.vote_for_block_without_finalizer_public_key.len() < 44 {
//             return false;
//         }

//         let mut batch = Verifier::new();
//         for sig in &self.signatures {
//             if sig.vote_signature.len() != 64 {
//                 return false;
//             }

//             let vk_bytes = VerificationKeyBytes::from(sig.public_key);
//             let mut sig_bytes = [0u8; 64];
//             sig_bytes.copy_from_slice(&sig.vote_signature);
//             let ed_sig = ed25519_zebra::Signature::from(sig_bytes);

//             // The signed message is the vote template WITHOUT the validator
//             // address (44 bytes). The public key is in the certificate
//             // envelope, not the signed payload. This matches zebra-crosslink's
//             // FatPointerToBftBlock2::validate_signatures() in malctx.rs.
//             let msg = &self.vote_for_block_without_finalizer_public_key[0..44];
//             batch.queue((vk_bytes, ed_sig, msg));
//         }
//         batch.verify(rand08::rngs::OsRng).is_ok()
//     }
// }

// /// The BFT block content for Crosslink.
// ///
// /// Each BFT block bundles σ PoW block headers and a reference to the
// /// previous BFT block's fat pointer.
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct BftBlock {
//     /// The version number.
//     pub version: u32,
//     /// The height of this BFT payload.
//     pub height: u32,
//     /// Hash of the previous BFT block (via its fat pointer).
//     pub previous_block_fat_ptr: FatPointerToBftBlock2,
//     /// The height of the PoW block that is the finalization candidate.
//     pub finalization_candidate_height: u32,
//     /// The PoW headers in reverse chain order (most recent first).
//     pub headers: Vec<PowHeader>,
// }

// impl BftBlock {
//     /// The PoW anchor block (finalization candidate) — the last header.
//     ///
//     /// # Panics
//     ///
//     /// Panics if the headers vector is empty.
//     #[must_use]
//     pub fn finalization_candidate(&self) -> &PowHeader {
//         self.headers
//             .last()
//             .expect("BftBlock headers should never be empty")
//     }

//     /// Compute the BLAKE3 hash of this BFT block.
//     #[must_use]
//     pub fn blake3_hash(&self) -> Blake3Hash {
//         let mut hasher = blake3::Hasher::new();
//         let mut buf = Vec::new();
//         buf.extend_from_slice(&self.version.to_le_bytes());
//         buf.extend_from_slice(&self.height.to_le_bytes());
//         buf.extend_from_slice(
//             &self
//                 .previous_block_fat_ptr
//                 .vote_for_block_without_finalizer_public_key,
//         );
//         let sig_count = self.previous_block_fat_ptr.signatures.len() as u16;
//         buf.extend_from_slice(&sig_count.to_le_bytes());
//         for sig in &self.previous_block_fat_ptr.signatures {
//             buf.extend_from_slice(&sig.public_key);
//             buf.extend_from_slice(&sig.vote_signature);
//         }
//         buf.extend_from_slice(&self.finalization_candidate_height.to_le_bytes());
//         buf.extend_from_slice(&(self.headers.len() as u32).to_le_bytes());
//         for h in &self.headers {
//             buf.extend_from_slice(&h.hash);
//             buf.extend_from_slice(&h.timestamp.to_le_bytes());
//             buf.extend_from_slice(&h.height.to_le_bytes());
//         }
//         hasher.update(&buf);
//         Blake3Hash(hasher.finalize().into())
//     }
// }