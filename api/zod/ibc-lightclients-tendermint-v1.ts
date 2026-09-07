// Auto-generated from ibc.lightclients.tendermint.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.lightclients.tendermint.v1.rs
// Package: ibc.lightclients.tendermint.v1
import { z } from 'zod';

/** ClientState from Tendermint tracks the current validator set, latest height, and a possible frozen height. */
export const ClientStateSchema = z.object({
  chain_id: z.string(),
  trust_level: z.lazy(() => FractionSchema).optional(),
  /** duration of the period since the LatestTimestamp during which the submitted headers are valid for upgrade */
  trusting_period: z.unknown() /* Duration */.optional(),
  /** duration of the staking unbonding period */
  unbonding_period: z.unknown() /* Duration */.optional(),
  /** defines how much new (untrusted) header's Time can drift into the future. */
  max_clock_drift: z.unknown() /* Duration */.optional(),
  /** Block height when the client was frozen due to a misbehaviour */
  frozen_height: z.unknown() /*  */.optional(),
  /** Latest height the client was updated to */
  latest_height: z.unknown() /*  */.optional(),
  /** Proof specifications used in verifying counterparty state */
  proof_specs: z.array(z.unknown() /*  */),
  /** Path at which next upgraded client will be committed. Each element corresponds to the key for a single CommitmentProof in the chained proof. NOTE: ClientState must stored under `{upgradePath}/{upgradeHeight}/clientState` ConsensusState must be stored under `{upgradepath}/{upgradeHeight}/consensusState` For SDK chains using the default upgrade module, upgrade_path should be \[\]string{"upgrade", "upgradedIBCState"}\` */
  upgrade_path: z.array(z.string()),
  /** allow_update_after_expiry is deprecated */
  allow_update_after_expiry: z.boolean(),
  /** allow_update_after_misbehaviour is deprecated */
  allow_update_after_misbehaviour: z.boolean(),
});
export type ClientState = z.infer<typeof ClientStateSchema>;

/** ConsensusState defines the consensus state from Tendermint. */
export const ConsensusStateSchema = z.object({
  /** timestamp that corresponds to the block height in which the ConsensusState was stored. */
  timestamp: z.unknown() /* Timestamp */.optional(),
  /** commitment root (i.e app hash) */
  root: z.unknown() /*  */.optional(),
  next_validators_hash: z.string(),
});
export type ConsensusState = z.infer<typeof ConsensusStateSchema>;

/** Misbehaviour is a wrapper over two conflicting Headers that implements Misbehaviour interface expected by ICS-02 */
export const MisbehaviourSchema = z.object({
  /** ClientID is deprecated */
  client_id: z.string(),
  header_1: z.lazy(() => HeaderSchema).optional(),
  header_2: z.lazy(() => HeaderSchema).optional(),
});
export type Misbehaviour = z.infer<typeof MisbehaviourSchema>;

/** Header defines the Tendermint client consensus Header. It encapsulates all the information necessary to update from a trusted Tendermint ConsensusState. The inclusion of TrustedHeight and TrustedValidators allows this update to process correctly, so long as the ConsensusState for the TrustedHeight exists, this removes race conditions among relayers The SignedHeader and ValidatorSet are the new untrusted update fields for the client. The TrustedHeight is the height of a stored ConsensusState on the client that will be used to verify the new untrusted header. The Trusted ConsensusState must be within the unbonding period of current time in order to correctly verify, and the TrustedValidators must hash to TrustedConsensusState.NextValidatorsHash since that is the last trusted validator set at the TrustedHeight. */
export const HeaderSchema = z.object({
  signed_header: z.unknown() /*  */.optional(),
  validator_set: z.unknown() /*  */.optional(),
  trusted_height: z.unknown() /*  */.optional(),
  trusted_validators: z.unknown() /*  */.optional(),
});
export type Header = z.infer<typeof HeaderSchema>;

/** Fraction defines the protobuf message type for tmmath.Fraction that only supports positive values. */
export const FractionSchema = z.object({
  numerator: z.string(),
  denominator: z.string(),
});
export type Fraction = z.infer<typeof FractionSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.lightclients.tendermint.v1' as const;

export const schemas = {
  ClientState: ClientStateSchema,
  ConsensusState: ConsensusStateSchema,
  Misbehaviour: MisbehaviourSchema,
  Header: HeaderSchema,
  Fraction: FractionSchema,
} as const;
