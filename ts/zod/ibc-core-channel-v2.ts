// Auto-generated from ibc.core.channel.v2 — do not edit.
// Source: terp-rs/src/gen/ibc.core.channel.v2.rs
// Package: ibc.core.channel.v2
import { z } from 'zod';

/** GenesisState defines the ibc channel/v2 submodule's genesis state. */
export const GenesisStateSchema = z.object({
  acknowledgements: z.array(z.lazy(() => PacketStateSchema)),
  commitments: z.array(z.lazy(() => PacketStateSchema)),
  receipts: z.array(z.lazy(() => PacketStateSchema)),
  async_packets: z.array(z.lazy(() => PacketStateSchema)),
  send_sequences: z.array(z.lazy(() => PacketSequenceSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** PacketState defines the generic type necessary to retrieve and store packet commitments, acknowledgements, and receipts. Caller is responsible for knowing the context necessary to interpret this state as a commitment, acknowledgement, or a receipt. */
export const PacketStateSchema = z.object({
  /** client unique identifier. */
  client_id: z.string(),
  /** packet sequence. */
  sequence: z.string(),
  /** embedded data that represents packet state. */
  data: z.string(),
});
export type PacketState = z.infer<typeof PacketStateSchema>;

/** PacketSequence defines the genesis type necessary to retrieve and store next send sequences. */
export const PacketSequenceSchema = z.object({
  /** client unique identifier. */
  client_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type PacketSequence = z.infer<typeof PacketSequenceSchema>;

/** Packet defines a type that carries data across different chains through IBC */
export const PacketSchema = z.object({
  /** number corresponds to the order of sends and receives, where a Packet with an earlier sequence number must be sent and received before a Packet with a later sequence number. */
  sequence: z.string(),
  /** identifies the sending client on the sending chain. */
  source_client: z.string(),
  /** identifies the receiving client on the receiving chain. */
  destination_client: z.string(),
  /** timeout timestamp in seconds after which the packet times out. */
  timeout_timestamp: z.string(),
  /** a list of payloads, each one for a specific application. */
  payloads: z.array(z.lazy(() => PayloadSchema)),
});
export type Packet = z.infer<typeof PacketSchema>;

/** Payload contains the source and destination ports and payload for the application (version, encoding, raw bytes) */
export const PayloadSchema = z.object({
  /** specifies the source port of the packet. */
  source_port: z.string(),
  /** specifies the destination port of the packet. */
  destination_port: z.string(),
  /** version of the specified application. */
  version: z.string(),
  /** the encoding used for the provided value. */
  encoding: z.string(),
  /** the raw bytes for the payload. */
  value: z.string(),
});
export type Payload = z.infer<typeof PayloadSchema>;

/** Acknowledgement contains a list of all ack results associated with a single packet. In the case of a successful receive, the acknowledgement will contain an app acknowledgement for each application that received a payload in the same order that the payloads were sent in the packet. If the receive is not successful, the acknowledgement will contain a single app acknowledgment which will be a constant error acknowledgment as defined by the IBC v2 protocol. */
export const AcknowledgementSchema = z.object({
  app_acknowledgements: z.array(z.string()),
});
export type Acknowledgement = z.infer<typeof AcknowledgementSchema>;

/** RecvPacketResult speecifies the status of a packet as well as the acknowledgement bytes. */
export const RecvPacketResultSchema = z.object({
  /** status of the packet */
  status: z.number().int(),
  /** acknowledgement of the packet */
  acknowledgement: z.string(),
});
export type RecvPacketResult = z.infer<typeof RecvPacketResultSchema>;

/** MsgSendPacket sends an outgoing IBC packet. */
export const MsgSendPacketSchema = z.object({
  source_client: z.string(),
  timeout_timestamp: z.string(),
  payloads: z.array(z.lazy(() => PayloadSchema)),
  signer: z.string(),
});
export type MsgSendPacket = z.infer<typeof MsgSendPacketSchema>;

/** MsgSendPacketResponse defines the Msg/SendPacket response type. */
export const MsgSendPacketResponseSchema = z.object({
  sequence: z.string(),
});
export type MsgSendPacketResponse = z.infer<typeof MsgSendPacketResponseSchema>;

/** MsgRecvPacket receives an incoming IBC packet. */
export const MsgRecvPacketSchema = z.object({
  packet: z.lazy(() => PacketSchema).optional(),
  proof_commitment: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgRecvPacket = z.infer<typeof MsgRecvPacketSchema>;

/** MsgRecvPacketResponse defines the Msg/RecvPacket response type. */
export const MsgRecvPacketResponseSchema = z.object({
  result: z.number().int(),
});
export type MsgRecvPacketResponse = z.infer<typeof MsgRecvPacketResponseSchema>;

/** MsgTimeout receives timed-out packet */
export const MsgTimeoutSchema = z.object({
  packet: z.lazy(() => PacketSchema).optional(),
  proof_unreceived: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgTimeout = z.infer<typeof MsgTimeoutSchema>;

/** MsgTimeoutResponse defines the Msg/Timeout response type. */
export const MsgTimeoutResponseSchema = z.object({
  result: z.number().int(),
});
export type MsgTimeoutResponse = z.infer<typeof MsgTimeoutResponseSchema>;

/** MsgAcknowledgement receives incoming IBC acknowledgement. */
export const MsgAcknowledgementSchema = z.object({
  packet: z.lazy(() => PacketSchema).optional(),
  acknowledgement: z.lazy(() => AcknowledgementSchema).optional(),
  proof_acked: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgAcknowledgement = z.infer<typeof MsgAcknowledgementSchema>;

/** MsgAcknowledgementResponse defines the Msg/Acknowledgement response type. */
export const MsgAcknowledgementResponseSchema = z.object({
  result: z.number().int(),
});
export type MsgAcknowledgementResponse = z.infer<typeof MsgAcknowledgementResponseSchema>;

/** QueryNextSequenceSendRequest is the request type for the Query/QueryNextSequenceSend RPC method */
export const QueryNextSequenceSendRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
});
export type QueryNextSequenceSendRequest = z.infer<typeof QueryNextSequenceSendRequestSchema>;

/** QueryNextSequenceSendResponse is the response type for the Query/QueryNextSequenceSend RPC method */
export const QueryNextSequenceSendResponseSchema = z.object({
  /** next sequence send number */
  next_sequence_send: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryNextSequenceSendResponse = z.infer<typeof QueryNextSequenceSendResponseSchema>;

/** QueryPacketCommitmentRequest is the request type for the Query/PacketCommitment RPC method. */
export const QueryPacketCommitmentRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketCommitmentRequest = z.infer<typeof QueryPacketCommitmentRequestSchema>;

/** QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method. */
export const QueryPacketCommitmentResponseSchema = z.object({
  /** packet associated with the request fields */
  commitment: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketCommitmentResponse = z.infer<typeof QueryPacketCommitmentResponseSchema>;

/** QueryPacketCommitmentsRequest is the request type for the Query/PacketCommitments RPC method. */
export const QueryPacketCommitmentsRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryPacketCommitmentsRequest = z.infer<typeof QueryPacketCommitmentsRequestSchema>;

/** QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method. */
export const QueryPacketCommitmentsResponseSchema = z.object({
  /** collection of packet commitments for the requested channel identifier. */
  commitments: z.array(z.lazy(() => PacketStateSchema)),
  /** pagination response. */
  pagination: z.unknown() /*  */.optional(),
  /** query block height. */
  height: z.unknown() /* Height */.optional(),
});
export type QueryPacketCommitmentsResponse = z.infer<typeof QueryPacketCommitmentsResponseSchema>;

/** QueryPacketAcknowledgementRequest is the request type for the Query/PacketAcknowledgement RPC method. */
export const QueryPacketAcknowledgementRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketAcknowledgementRequest = z.infer<typeof QueryPacketAcknowledgementRequestSchema>;

/** QueryPacketAcknowledgementResponse is the response type for the Query/PacketAcknowledgement RPC method. */
export const QueryPacketAcknowledgementResponseSchema = z.object({
  /** acknowledgement associated with the request fields */
  acknowledgement: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketAcknowledgementResponse = z.infer<typeof QueryPacketAcknowledgementResponseSchema>;

/** QueryPacketAcknowledgementsRequest is the request type for the Query/QueryPacketCommitments RPC method */
export const QueryPacketAcknowledgementsRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
  /** list of packet sequences */
  packet_commitment_sequences: z.array(z.string()),
});
export type QueryPacketAcknowledgementsRequest = z.infer<typeof QueryPacketAcknowledgementsRequestSchema>;

/** QueryPacketAcknowledgemetsResponse is the request type for the Query/QueryPacketAcknowledgements RPC method */
export const QueryPacketAcknowledgementsResponseSchema = z.object({
  acknowledgements: z.array(z.lazy(() => PacketStateSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryPacketAcknowledgementsResponse = z.infer<typeof QueryPacketAcknowledgementsResponseSchema>;

/** QueryPacketReceiptRequest is the request type for the Query/PacketReceipt RPC method. */
export const QueryPacketReceiptRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketReceiptRequest = z.infer<typeof QueryPacketReceiptRequestSchema>;

/** QueryPacketReceiptResponse is the response type for the Query/PacketReceipt RPC method. */
export const QueryPacketReceiptResponseSchema = z.object({
  /** success flag for if receipt exists */
  received: z.boolean(),
  /** merkle proof of existence or absence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketReceiptResponse = z.infer<typeof QueryPacketReceiptResponseSchema>;

/** QueryUnreceivedPacketsRequest is the request type for the Query/UnreceivedPackets RPC method */
export const QueryUnreceivedPacketsRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** list of packet sequences */
  sequences: z.array(z.string()),
});
export type QueryUnreceivedPacketsRequest = z.infer<typeof QueryUnreceivedPacketsRequestSchema>;

/** QueryUnreceivedPacketsResponse is the response type for the Query/UnreceivedPacketCommitments RPC method */
export const QueryUnreceivedPacketsResponseSchema = z.object({
  /** list of unreceived packet sequences */
  sequences: z.array(z.string()),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryUnreceivedPacketsResponse = z.infer<typeof QueryUnreceivedPacketsResponseSchema>;

/** QueryUnreceivedAcks is the request type for the Query/UnreceivedAcks RPC method */
export const QueryUnreceivedAcksRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** list of acknowledgement sequences */
  packet_ack_sequences: z.array(z.string()),
});
export type QueryUnreceivedAcksRequest = z.infer<typeof QueryUnreceivedAcksRequestSchema>;

/** QueryUnreceivedAcksResponse is the response type for the Query/UnreceivedAcks RPC method */
export const QueryUnreceivedAcksResponseSchema = z.object({
  /** list of unreceived acknowledgement sequences */
  sequences: z.array(z.string()),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryUnreceivedAcksResponse = z.infer<typeof QueryUnreceivedAcksResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.channel.v2' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
  PacketState: PacketStateSchema,
  PacketSequence: PacketSequenceSchema,
  Packet: PacketSchema,
  Payload: PayloadSchema,
  Acknowledgement: AcknowledgementSchema,
  RecvPacketResult: RecvPacketResultSchema,
  MsgSendPacket: MsgSendPacketSchema,
  MsgSendPacketResponse: MsgSendPacketResponseSchema,
  MsgRecvPacket: MsgRecvPacketSchema,
  MsgRecvPacketResponse: MsgRecvPacketResponseSchema,
  MsgTimeout: MsgTimeoutSchema,
  MsgTimeoutResponse: MsgTimeoutResponseSchema,
  MsgAcknowledgement: MsgAcknowledgementSchema,
  MsgAcknowledgementResponse: MsgAcknowledgementResponseSchema,
  QueryNextSequenceSendRequest: QueryNextSequenceSendRequestSchema,
  QueryNextSequenceSendResponse: QueryNextSequenceSendResponseSchema,
  QueryPacketCommitmentRequest: QueryPacketCommitmentRequestSchema,
  QueryPacketCommitmentResponse: QueryPacketCommitmentResponseSchema,
  QueryPacketCommitmentsRequest: QueryPacketCommitmentsRequestSchema,
  QueryPacketCommitmentsResponse: QueryPacketCommitmentsResponseSchema,
  QueryPacketAcknowledgementRequest: QueryPacketAcknowledgementRequestSchema,
  QueryPacketAcknowledgementResponse: QueryPacketAcknowledgementResponseSchema,
  QueryPacketAcknowledgementsRequest: QueryPacketAcknowledgementsRequestSchema,
  QueryPacketAcknowledgementsResponse: QueryPacketAcknowledgementsResponseSchema,
  QueryPacketReceiptRequest: QueryPacketReceiptRequestSchema,
  QueryPacketReceiptResponse: QueryPacketReceiptResponseSchema,
  QueryUnreceivedPacketsRequest: QueryUnreceivedPacketsRequestSchema,
  QueryUnreceivedPacketsResponse: QueryUnreceivedPacketsResponseSchema,
  QueryUnreceivedAcksRequest: QueryUnreceivedAcksRequestSchema,
  QueryUnreceivedAcksResponse: QueryUnreceivedAcksResponseSchema,
} as const;
