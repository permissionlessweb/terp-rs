// Auto-generated from ibc.lightclients.solomachine.v2 — do not edit.
// Source: terp-rs/src/gen/ibc.lightclients.solomachine.v2.rs
// Package: ibc.lightclients.solomachine.v2
import { z } from 'zod';

/** ClientState defines a solo machine client that tracks the current consensus state and if the client is frozen. */
export const ClientStateSchema = z.object({
  /** latest sequence of the client state */
  sequence: z.string(),
  /** frozen sequence of the solo machine */
  is_frozen: z.boolean(),
  consensus_state: z.lazy(() => ConsensusStateSchema).optional(),
  /** when set to true, will allow governance to update a solo machine client. The client will be unfrozen if it is frozen. */
  allow_update_after_proposal: z.boolean(),
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
  /** sequence to update solo machine public key at */
  sequence: z.string(),
  timestamp: z.string(),
  signature: z.string(),
  new_public_key: z.unknown() /* Any */.optional(),
  new_diversifier: z.string(),
});
export type Header = z.infer<typeof HeaderSchema>;

/** Misbehaviour defines misbehaviour for a solo machine which consists of a sequence and two signatures over different messages at that sequence. */
export const MisbehaviourSchema = z.object({
  client_id: z.string(),
  sequence: z.string(),
  signature_one: z.lazy(() => SignatureAndDataSchema).optional(),
  signature_two: z.lazy(() => SignatureAndDataSchema).optional(),
});
export type Misbehaviour = z.infer<typeof MisbehaviourSchema>;

/** SignatureAndData contains a signature and the data signed over to create that signature. */
export const SignatureAndDataSchema = z.object({
  signature: z.string(),
  data_type: z.number().int(),
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
  sequence: z.string(),
  timestamp: z.string(),
  diversifier: z.string(),
  /** type of the data used */
  data_type: z.number().int(),
  /** marshaled data */
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

/** ClientStateData returns the SignBytes data for client state verification. */
export const ClientStateDataSchema = z.object({
  path: z.string(),
  client_state: z.unknown() /* Any */.optional(),
});
export type ClientStateData = z.infer<typeof ClientStateDataSchema>;

/** ConsensusStateData returns the SignBytes data for consensus state verification. */
export const ConsensusStateDataSchema = z.object({
  path: z.string(),
  consensus_state: z.unknown() /* Any */.optional(),
});
export type ConsensusStateData = z.infer<typeof ConsensusStateDataSchema>;

/** ConnectionStateData returns the SignBytes data for connection state verification. */
export const ConnectionStateDataSchema = z.object({
  path: z.string(),
  connection: z.unknown() /*  */.optional(),
});
export type ConnectionStateData = z.infer<typeof ConnectionStateDataSchema>;

/** ChannelStateData returns the SignBytes data for channel state verification. */
export const ChannelStateDataSchema = z.object({
  path: z.string(),
  channel: z.unknown() /* Channel */.optional(),
});
export type ChannelStateData = z.infer<typeof ChannelStateDataSchema>;

/** PacketCommitmentData returns the SignBytes data for packet commitment verification. */
export const PacketCommitmentDataSchema = z.object({
  path: z.string(),
  commitment: z.string(),
});
export type PacketCommitmentData = z.infer<typeof PacketCommitmentDataSchema>;

/** PacketAcknowledgementData returns the SignBytes data for acknowledgement verification. */
export const PacketAcknowledgementDataSchema = z.object({
  path: z.string(),
  acknowledgement: z.string(),
});
export type PacketAcknowledgementData = z.infer<typeof PacketAcknowledgementDataSchema>;

/** PacketReceiptAbsenceData returns the SignBytes data for packet receipt absence verification. */
export const PacketReceiptAbsenceDataSchema = z.object({
  path: z.string(),
});
export type PacketReceiptAbsenceData = z.infer<typeof PacketReceiptAbsenceDataSchema>;

/** NextSequenceRecvData returns the SignBytes data for verification of the next sequence to be received. */
export const NextSequenceRecvDataSchema = z.object({
  path: z.string(),
  next_seq_recv: z.string(),
});
export type NextSequenceRecvData = z.infer<typeof NextSequenceRecvDataSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.lightclients.solomachine.v2' as const;

export const schemas = {
  ClientState: ClientStateSchema,
  ConsensusState: ConsensusStateSchema,
  Header: HeaderSchema,
  Misbehaviour: MisbehaviourSchema,
  SignatureAndData: SignatureAndDataSchema,
  TimestampedSignatureData: TimestampedSignatureDataSchema,
  SignBytes: SignBytesSchema,
  HeaderData: HeaderDataSchema,
  ClientStateData: ClientStateDataSchema,
  ConsensusStateData: ConsensusStateDataSchema,
  ConnectionStateData: ConnectionStateDataSchema,
  ChannelStateData: ChannelStateDataSchema,
  PacketCommitmentData: PacketCommitmentDataSchema,
  PacketAcknowledgementData: PacketAcknowledgementDataSchema,
  PacketReceiptAbsenceData: PacketReceiptAbsenceDataSchema,
  NextSequenceRecvData: NextSequenceRecvDataSchema,
} as const;
