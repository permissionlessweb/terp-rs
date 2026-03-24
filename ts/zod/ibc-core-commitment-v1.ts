// Auto-generated from ibc.core.commitment.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.core.commitment.v1.rs
// Package: ibc.core.commitment.v1
import { z } from 'zod';

/** MerkleRoot defines a merkle root hash. In the Cosmos SDK, the AppHash of a block header becomes the root. */
export const MerkleRootSchema = z.object({
  hash: z.string(),
});
export type MerkleRoot = z.infer<typeof MerkleRootSchema>;

/** MerklePrefix is merkle path prefixed to the key. The constructed key from the Path and the key will be append(Path.KeyPath, append(Path.KeyPrefix, key...)) */
export const MerklePrefixSchema = z.object({
  key_prefix: z.string(),
});
export type MerklePrefix = z.infer<typeof MerklePrefixSchema>;

/** MerkleProof is a wrapper type over a chain of CommitmentProofs. It demonstrates membership or non-membership for an element or set of elements, verifiable in conjunction with a known commitment root. Proofs should be succinct. MerkleProofs are ordered from leaf-to-root */
export const MerkleProofSchema = z.object({
  proofs: z.array(z.unknown() /*  */),
});
export type MerkleProof = z.infer<typeof MerkleProofSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.commitment.v1' as const;

export const schemas = {
  MerkleRoot: MerkleRootSchema,
  MerklePrefix: MerklePrefixSchema,
  MerkleProof: MerkleProofSchema,
} as const;
