// Auto-generated from ibc.lightclients.solomachine.v3 — do not edit.
// Source: terp-rs/src/gen/ibc.lightclients.solomachine.v3.rs
// Package: ibc.lightclients.solomachine.v3
import { z } from 'zod';

/** ClientState defines a solo machine client that tracks the current consensus state and if the client is frozen. */
export const ClientStateSchema = z.object({
  /** latest sequence of the client state */
  sequence: z.string(),
  /** frozen sequence of the solo machine */
  is_frozen: z.boolean(),
  consensus_state: z.lazy(() => ConsensusStateSchema).optional(),
});
export type ClientState = z.infer<typeof ClientStateSchema>;

/** ConsensusState defines a solo machine consensus state. The sequence of a consensus state is contained in the "height" key used in storing the consensus state. */
export const ConsensusStateSchema = z.object({
  /** public key of the solo machine */
  public_key: z.unknown() /* Any */.optional(),
  /** diversifier allows the same public key to be reused across different solo machine clients (potentially on different chains) without being considered misbehaviour. */
  diversifier: z.string(),
  timestamp: z.string(),
});
export type ConsensusState = z.infer<typeof ConsensusStateSchema>;

/** Header defines a solo machine consensus header */
export const HeaderSchema = z.object({
  timestamp: z.string(),
  signature: z.string(),
  new_public_key: z.unknown() /* Any */.optional(),
  new_diversifier: z.string(),
});
export type Header = z.infer<typeof HeaderSchema>;

/** Misbehaviour defines misbehaviour for a solo machine which consists of a sequence and two signatures over different messages at that sequence. */
export const MisbehaviourSchema = z.object({
  sequence: z.string(),
  signature_one: z.lazy(() => SignatureAndDataSchema).optional(),
  signature_two: z.lazy(() => SignatureAndDataSchema).optional(),
});
export type Misbehaviour = z.infer<typeof MisbehaviourSchema>;

/** SignatureAndData contains a signature and the data signed over to create that signature. */
export const SignatureAndDataSchema = z.object({
  signature: z.string(),
  path: z.string(),
  data: z.string(),
  timestamp: z.string(),
});
export type SignatureAndData = z.infer<typeof SignatureAndDataSchema>;

/** TimestampedSignatureData contains the signature data and the timestamp of the signature. */
export const TimestampedSignatureDataSchema = z.object({
  signature_data: z.string(),
  timestamp: z.string(),
});
export type TimestampedSignatureData = z.infer<typeof TimestampedSignatureDataSchema>;

/** SignBytes defines the signed bytes used for signature verification. */
export const SignBytesSchema = z.object({
  /** the sequence number */
  sequence: z.string(),
  /** the proof timestamp */
  timestamp: z.string(),
  /** the public key diversifier */
  diversifier: z.string(),
  /** the standardised path bytes */
  path: z.string(),
  /** the marshaled data bytes */
  data: z.string(),
});
export type SignBytes = z.infer<typeof SignBytesSchema>;

/** HeaderData returns the SignBytes data for update verification. */
export const HeaderDataSchema = z.object({
  /** header public key */
  new_pub_key: z.unknown() /* Any */.optional(),
  /** header diversifier */
  new_diversifier: z.string(),
});
export type HeaderData = z.infer<typeof HeaderDataSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.lightclients.solomachine.v3' as const;

export const schemas = {
  ClientState: ClientStateSchema,
  ConsensusState: ConsensusStateSchema,
  Header: HeaderSchema,
  Misbehaviour: MisbehaviourSchema,
  SignatureAndData: SignatureAndDataSchema,
  TimestampedSignatureData: TimestampedSignatureDataSchema,
  SignBytes: SignBytesSchema,
  HeaderData: HeaderDataSchema,
} as const;
