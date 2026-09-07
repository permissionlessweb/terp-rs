// Auto-generated from ibc.lightclients.attestations.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.lightclients.attestations.v1.rs
// Package: ibc.lightclients.attestations.v1
import { z } from 'zod';

/** ClientState defines an attestor-based light client that tracks the current consensus state and if the client is frozen. */
export const ClientStateSchema = z.object({
  /** trusted attestor set (EOA addresses) */
  attestor_addresses: z.array(z.string()),
  /** quorum threshold (minimum number of unique attestor signatures required) */
  min_required_sigs: z.number().int(),
  /** highest height that has been trusted */
  latest_height: z.string(),
  /** when true, all verification and updates MUST fail */
  is_frozen: z.boolean(),
});
export type ClientState = z.infer<typeof ClientStateSchema>;

/** ConsensusState defines an attestor consensus state. The timestamp of a consensus state is stored per height. */
export const ConsensusStateSchema = z.object({
  /** trusted UNIX timestamp (nanoseconds) for the height */
  timestamp: z.string(),
});
export type ConsensusState = z.infer<typeof ConsensusStateSchema>;

/** AttestationProof is used for client updates and membership verification. All attestor signatures cover sha256(attestationData). */
export const AttestationProofSchema = z.object({
  /** the attestation data that was signed (ABI-encoded StateAttestation or PacketAttestation) */
  attestation_data: z.string(),
  /** array of 65-byte ECDSA signatures (r||s||v) */
  signatures: z.array(z.string()),
});
export type AttestationProof = z.infer<typeof AttestationProofSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.lightclients.attestations.v1' as const;

export const schemas = {
  ClientState: ClientStateSchema,
  ConsensusState: ConsensusStateSchema,
  AttestationProof: AttestationProofSchema,
} as const;
