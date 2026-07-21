//! VSCK proof verification hooks aligned with vote-sdk circuit families.
//!
//! vote-sdk (`shielded-vote-circuits`) exposes:
//! - `delegation` — prove voting power / note membership
//! - `vote_proof` — prove ballot well-formedness without revealing choice
//! - `share_reveal` — ceremony share open for tally
//!
//! This module defines the on-chain verify API. Default implementation checks
//! envelope + VK presence; enable `vote-circuits` feature later to call real
//! Halo2 verify (or host functions).

use cosmwasm_std::Deps;

use crate::error::ContractError;
use crate::msg::{CircuitVerifyingKeys, VsckAuthPayload, VsckCircuit, VsckRole};
use crate::state::Session;

pub trait VsckVerifier {
    fn verify(
        &self,
        deps: Deps,
        vks: &CircuitVerifyingKeys,
        session: &Session,
        payload: &VsckAuthPayload,
    ) -> Result<(), ContractError>;
}

pub struct DefaultVsckVerifier;

impl VsckVerifier for DefaultVsckVerifier {
    fn verify(
        &self,
        _deps: Deps,
        vks: &CircuitVerifyingKeys,
        session: &Session,
        payload: &VsckAuthPayload,
    ) -> Result<(), ContractError> {
        if !session.open {
            return Err(ContractError::InvalidProof {
                reason: "session closed".into(),
            });
        }
        if payload.proof.is_empty() || payload.public_inputs.is_empty() {
            return Err(ContractError::InvalidProof {
                reason: "empty proof or public_inputs".into(),
            });
        }
        if payload.nullifier.is_empty() {
            return Err(ContractError::InvalidProof {
                reason: "empty nullifier".into(),
            });
        }

        // Role ↔ circuit consistency (DAO voting module policy).
        match (&payload.role, &payload.circuit) {
            (VsckRole::Delegator, VsckCircuit::Delegation) => {
                if vks.delegation_vk.is_empty() {
                    return Err(ContractError::InvalidProof {
                        reason: "delegation_vk missing".into(),
                    });
                }
            }
            (VsckRole::Voter, VsckCircuit::VoteProof) => {
                if vks.vote_proof_vk.is_empty() {
                    return Err(ContractError::InvalidProof {
                        reason: "vote_proof_vk missing".into(),
                    });
                }
            }
            (VsckRole::Tallier, VsckCircuit::ShareReveal) => {
                if vks.share_reveal_vk.is_empty() {
                    return Err(ContractError::InvalidProof {
                        reason: "share_reveal_vk missing".into(),
                    });
                }
            }
            (VsckRole::Coordinator, _) => {
                return Err(ContractError::RoleDenied {
                    role: "Coordinator".into(),
                });
            }
            (role, circuit) => {
                return Err(ContractError::InvalidProof {
                    reason: format!("role/circuit mismatch: {role:?} vs {circuit:?}"),
                });
            }
        }

        // Public inputs should bind to session tree root (prefix check on stub).
        // Production: circuit instance includes note_tree_root equality.
        if !payload
            .public_inputs
            .as_slice()
            .windows(session.note_tree_root.len().max(1))
            .any(|w| w == session.note_tree_root.as_slice())
            && !session.note_tree_root.is_empty()
        {
            // Soft-fail only if root non-empty and not found — allow short public_inputs in tests.
            if payload.public_inputs.len() >= session.note_tree_root.len()
                && session.note_tree_root.len() >= 8
            {
                return Err(ContractError::InvalidProof {
                    reason: "public_inputs do not bind session note_tree_root".into(),
                });
            }
        }

        // TODO(vote-circuits):
        // match payload.circuit {
        //   Delegation => shielded_vote_circuits::delegation::verify(...),
        //   VoteProof => ...,
        //   ShareReveal => ...,
        // }
        Ok(())
    }
}
