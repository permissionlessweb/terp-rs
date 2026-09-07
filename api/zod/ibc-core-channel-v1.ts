// Auto-generated from ibc.core.channel.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.core.channel.v1.rs
// Package: ibc.core.channel.v1
import { z } from 'zod';

/** Channel defines pipeline for exactly-once packet delivery between specific modules on separate blockchains, which has at least one end capable of sending packets and one end capable of receiving packets. */
export const ChannelSchema = z.object({
  /** current state of the channel end */
  state: z.number().int(),
  /** whether the channel is ordered or unordered */
  ordering: z.number().int(),
  /** counterparty channel end */
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  /** list of connection identifiers, in order, along which packets sent on this channel will travel */
  connection_hops: z.array(z.string()),
  /** opaque channel version, which is agreed upon during the handshake */
  version: z.string(),
});
export type Channel = z.infer<typeof ChannelSchema>;

/** IdentifiedChannel defines a channel with additional port and channel identifier fields. */
export const IdentifiedChannelSchema = z.object({
  /** current state of the channel end */
  state: z.number().int(),
  /** whether the channel is ordered or unordered */
  ordering: z.number().int(),
  /** counterparty channel end */
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  /** list of connection identifiers, in order, along which packets sent on this channel will travel */
  connection_hops: z.array(z.string()),
  /** opaque channel version, which is agreed upon during the handshake */
  version: z.string(),
  /** port identifier */
  port_id: z.string(),
  /** channel identifier */
  channel_id: z.string(),
});
export type IdentifiedChannel = z.infer<typeof IdentifiedChannelSchema>;

/** Counterparty defines a channel end counterparty */
export const CounterpartySchema = z.object({
  /** port on the counterparty chain which owns the other end of the channel. */
  port_id: z.string(),
  /** channel end on the counterparty chain */
  channel_id: z.string(),
});
export type Counterparty = z.infer<typeof CounterpartySchema>;

/** Packet defines a type that carries data across different chains through IBC */
export const PacketSchema = z.object({
  /** number corresponds to the order of sends and receives, where a Packet with an earlier sequence number must be sent and received before a Packet with a later sequence number. */
  sequence: z.string(),
  /** identifies the port on the sending chain. */
  source_port: z.string(),
  /** identifies the channel end on the sending chain. */
  source_channel: z.string(),
  /** identifies the port on the receiving chain. */
  destination_port: z.string(),
  /** identifies the channel end on the receiving chain. */
  destination_channel: z.string(),
  /** actual opaque bytes transferred directly to the application module */
  data: z.string(),
  /** block height after which the packet times out */
  timeout_height: z.unknown() /* Height */.optional(),
  /** block timestamp (in nanoseconds) after which the packet times out */
  timeout_timestamp: z.string(),
});
export type Packet = z.infer<typeof PacketSchema>;

/** PacketState defines the generic type necessary to retrieve and store packet commitments, acknowledgements, and receipts. Caller is responsible for knowing the context necessary to interpret this state as a commitment, acknowledgement, or a receipt. */
export const PacketStateSchema = z.object({
  /** channel port identifier. */
  port_id: z.string(),
  /** channel unique identifier. */
  channel_id: z.string(),
  /** packet sequence. */
  sequence: z.string(),
  /** embedded data that represents packet state. */
  data: z.string(),
});
export type PacketState = z.infer<typeof PacketStateSchema>;

/** PacketId is an identifier for a unique Packet Source chains refer to packets by source port/channel Destination chains refer to packets by destination port/channel */
export const PacketIdSchema = z.object({
  /** channel port identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type PacketId = z.infer<typeof PacketIdSchema>;

/** Acknowledgement is the recommended acknowledgement format to be used by app-specific protocols. NOTE: The field numbers 21 and 22 were explicitly chosen to avoid accidental conflicts with other protobuf message formats used for acknowledgements. The first byte of any message with this format will be the non-ASCII values `0xaa` (result) or `0xb2` (error). Implemented as defined by ICS: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope> */
export const AcknowledgementSchema = z.object({
  /** response contains either a result or an error and must be non-empty */
  response: z.unknown(),
});
export type Acknowledgement = z.infer<typeof AcknowledgementSchema>;

/** Timeout defines an execution deadline structure for 04-channel handlers. This includes packet lifecycle handlers. A valid Timeout contains either one or both of a timestamp and block height (sequence). */
export const TimeoutSchema = z.object({
  /** block height after which the packet times out */
  height: z.unknown() /* Height */.optional(),
  /** block timestamp (in nanoseconds) after which the packet times out */
  timestamp: z.string(),
});
export type Timeout = z.infer<typeof TimeoutSchema>;

/** GenesisState defines the ibc channel submodule's genesis state. */
export const GenesisStateSchema = z.object({
  channels: z.array(z.lazy(() => IdentifiedChannelSchema)),
  acknowledgements: z.array(z.lazy(() => PacketStateSchema)),
  commitments: z.array(z.lazy(() => PacketStateSchema)),
  receipts: z.array(z.lazy(() => PacketStateSchema)),
  send_sequences: z.array(z.lazy(() => PacketSequenceSchema)),
  recv_sequences: z.array(z.lazy(() => PacketSequenceSchema)),
  ack_sequences: z.array(z.lazy(() => PacketSequenceSchema)),
  /** the sequence for the next generated channel identifier */
  next_channel_sequence: z.string(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** PacketSequence defines the genesis type necessary to retrieve and store next send and receive sequences. */
export const PacketSequenceSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
  sequence: z.string(),
});
export type PacketSequence = z.infer<typeof PacketSequenceSchema>;

/** MsgChannelOpenInit defines an sdk.Msg to initialize a channel handshake. It is called by a relayer on Chain A. */
export const MsgChannelOpenInitSchema = z.object({
  port_id: z.string(),
  channel: z.lazy(() => ChannelSchema).optional(),
  signer: z.string(),
});
export type MsgChannelOpenInit = z.infer<typeof MsgChannelOpenInitSchema>;

/** MsgChannelOpenInitResponse defines the Msg/ChannelOpenInit response type. */
export const MsgChannelOpenInitResponseSchema = z.object({
  channel_id: z.string(),
  version: z.string(),
});
export type MsgChannelOpenInitResponse = z.infer<typeof MsgChannelOpenInitResponseSchema>;

/** MsgChannelOpenInit defines a msg sent by a Relayer to try to open a channel on Chain B. The version field within the Channel field has been deprecated. Its value will be ignored by core IBC. */
export const MsgChannelOpenTrySchema = z.object({
  port_id: z.string(),
  /** Deprecated: this field is unused. Crossing hello's are no longer supported in core IBC. */
  previous_channel_id: z.string(),
  /** NOTE: the version field within the channel has been deprecated. Its value will be ignored by core IBC. */
  channel: z.lazy(() => ChannelSchema).optional(),
  counterparty_version: z.string(),
  proof_init: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgChannelOpenTry = z.infer<typeof MsgChannelOpenTrySchema>;

/** MsgChannelOpenTryResponse defines the Msg/ChannelOpenTry response type. */
export const MsgChannelOpenTryResponseSchema = z.object({
  version: z.string(),
  channel_id: z.string(),
});
export type MsgChannelOpenTryResponse = z.infer<typeof MsgChannelOpenTryResponseSchema>;

/** MsgChannelOpenAck defines a msg sent by a Relayer to Chain A to acknowledge the change of channel state to TRYOPEN on Chain B. */
export const MsgChannelOpenAckSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
  counterparty_channel_id: z.string(),
  counterparty_version: z.string(),
  proof_try: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgChannelOpenAck = z.infer<typeof MsgChannelOpenAckSchema>;

/** MsgChannelOpenAckResponse defines the Msg/ChannelOpenAck response type. */
export const MsgChannelOpenAckResponseSchema = z.object({});
export type MsgChannelOpenAckResponse = z.infer<typeof MsgChannelOpenAckResponseSchema>;

/** MsgChannelOpenConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of channel state to OPEN on Chain A. */
export const MsgChannelOpenConfirmSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
  proof_ack: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgChannelOpenConfirm = z.infer<typeof MsgChannelOpenConfirmSchema>;

/** MsgChannelOpenConfirmResponse defines the Msg/ChannelOpenConfirm response type. */
export const MsgChannelOpenConfirmResponseSchema = z.object({});
export type MsgChannelOpenConfirmResponse = z.infer<typeof MsgChannelOpenConfirmResponseSchema>;

/** MsgChannelCloseInit defines a msg sent by a Relayer to Chain A to close a channel with Chain B. */
export const MsgChannelCloseInitSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
  signer: z.string(),
});
export type MsgChannelCloseInit = z.infer<typeof MsgChannelCloseInitSchema>;

/** MsgChannelCloseInitResponse defines the Msg/ChannelCloseInit response type. */
export const MsgChannelCloseInitResponseSchema = z.object({});
export type MsgChannelCloseInitResponse = z.infer<typeof MsgChannelCloseInitResponseSchema>;

/** MsgChannelCloseConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of channel state to CLOSED on Chain A. */
export const MsgChannelCloseConfirmSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
  proof_init: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgChannelCloseConfirm = z.infer<typeof MsgChannelCloseConfirmSchema>;

/** MsgChannelCloseConfirmResponse defines the Msg/ChannelCloseConfirm response type. */
export const MsgChannelCloseConfirmResponseSchema = z.object({});
export type MsgChannelCloseConfirmResponse = z.infer<typeof MsgChannelCloseConfirmResponseSchema>;

/** MsgRecvPacket receives incoming IBC packet */
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
  next_sequence_recv: z.string(),
  signer: z.string(),
});
export type MsgTimeout = z.infer<typeof MsgTimeoutSchema>;

/** MsgTimeoutResponse defines the Msg/Timeout response type. */
export const MsgTimeoutResponseSchema = z.object({
  result: z.number().int(),
});
export type MsgTimeoutResponse = z.infer<typeof MsgTimeoutResponseSchema>;

/** MsgTimeoutOnClose timed-out packet upon counterparty channel closure. */
export const MsgTimeoutOnCloseSchema = z.object({
  packet: z.lazy(() => PacketSchema).optional(),
  proof_unreceived: z.string(),
  proof_close: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  next_sequence_recv: z.string(),
  signer: z.string(),
});
export type MsgTimeoutOnClose = z.infer<typeof MsgTimeoutOnCloseSchema>;

/** MsgTimeoutOnCloseResponse defines the Msg/TimeoutOnClose response type. */
export const MsgTimeoutOnCloseResponseSchema = z.object({
  result: z.number().int(),
});
export type MsgTimeoutOnCloseResponse = z.infer<typeof MsgTimeoutOnCloseResponseSchema>;

/** MsgAcknowledgement receives incoming IBC acknowledgement */
export const MsgAcknowledgementSchema = z.object({
  packet: z.lazy(() => PacketSchema).optional(),
  acknowledgement: z.string(),
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

/** QueryChannelRequest is the request type for the Query/Channel RPC method */
export const QueryChannelRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
});
export type QueryChannelRequest = z.infer<typeof QueryChannelRequestSchema>;

/** QueryChannelResponse is the response type for the Query/Channel RPC method. Besides the Channel end, it includes a proof and the height from which the proof was retrieved. */
export const QueryChannelResponseSchema = z.object({
  /** channel associated with the request identifiers */
  channel: z.lazy(() => ChannelSchema).optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryChannelResponse = z.infer<typeof QueryChannelResponseSchema>;

/** QueryChannelsRequest is the request type for the Query/Channels RPC method */
export const QueryChannelsRequestSchema = z.object({
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryChannelsRequest = z.infer<typeof QueryChannelsRequestSchema>;

/** QueryChannelsResponse is the response type for the Query/Channels RPC method. */
export const QueryChannelsResponseSchema = z.object({
  /** list of stored channels of the chain. */
  channels: z.array(z.lazy(() => IdentifiedChannelSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryChannelsResponse = z.infer<typeof QueryChannelsResponseSchema>;

/** QueryConnectionChannelsRequest is the request type for the Query/QueryConnectionChannels RPC method */
export const QueryConnectionChannelsRequestSchema = z.object({
  /** connection unique identifier */
  connection: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConnectionChannelsRequest = z.infer<typeof QueryConnectionChannelsRequestSchema>;

/** QueryConnectionChannelsResponse is the Response type for the Query/QueryConnectionChannels RPC method */
export const QueryConnectionChannelsResponseSchema = z.object({
  /** list of channels associated with a connection. */
  channels: z.array(z.lazy(() => IdentifiedChannelSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryConnectionChannelsResponse = z.infer<typeof QueryConnectionChannelsResponseSchema>;

/** QueryChannelClientStateRequest is the request type for the Query/ClientState RPC method */
export const QueryChannelClientStateRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
});
export type QueryChannelClientStateRequest = z.infer<typeof QueryChannelClientStateRequestSchema>;

/** QueryChannelClientStateResponse is the Response type for the Query/QueryChannelClientState RPC method */
export const QueryChannelClientStateResponseSchema = z.object({
  /** client state associated with the channel */
  identified_client_state: z.unknown() /*  */.optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryChannelClientStateResponse = z.infer<typeof QueryChannelClientStateResponseSchema>;

/** QueryChannelConsensusStateRequest is the request type for the Query/ConsensusState RPC method */
export const QueryChannelConsensusStateRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** revision number of the consensus state */
  revision_number: z.string(),
  /** revision height of the consensus state */
  revision_height: z.string(),
});
export type QueryChannelConsensusStateRequest = z.infer<typeof QueryChannelConsensusStateRequestSchema>;

/** QueryChannelClientStateResponse is the Response type for the Query/QueryChannelClientState RPC method */
export const QueryChannelConsensusStateResponseSchema = z.object({
  /** consensus state associated with the channel */
  consensus_state: z.unknown() /* Any */.optional(),
  /** client ID associated with the consensus state */
  client_id: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryChannelConsensusStateResponse = z.infer<typeof QueryChannelConsensusStateResponseSchema>;

/** QueryPacketCommitmentRequest is the request type for the Query/PacketCommitment RPC method */
export const QueryPacketCommitmentRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketCommitmentRequest = z.infer<typeof QueryPacketCommitmentRequestSchema>;

/** QueryPacketCommitmentResponse defines the client query response for a packet which also includes a proof and the height from which the proof was retrieved */
export const QueryPacketCommitmentResponseSchema = z.object({
  /** packet associated with the request fields */
  commitment: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketCommitmentResponse = z.infer<typeof QueryPacketCommitmentResponseSchema>;

/** QueryPacketCommitmentsRequest is the request type for the Query/QueryPacketCommitments RPC method */
export const QueryPacketCommitmentsRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryPacketCommitmentsRequest = z.infer<typeof QueryPacketCommitmentsRequestSchema>;

/** QueryPacketCommitmentsResponse is the request type for the Query/QueryPacketCommitments RPC method */
export const QueryPacketCommitmentsResponseSchema = z.object({
  commitments: z.array(z.lazy(() => PacketStateSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryPacketCommitmentsResponse = z.infer<typeof QueryPacketCommitmentsResponseSchema>;

/** QueryPacketReceiptRequest is the request type for the Query/PacketReceipt RPC method */
export const QueryPacketReceiptRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketReceiptRequest = z.infer<typeof QueryPacketReceiptRequestSchema>;

/** QueryPacketReceiptResponse defines the client query response for a packet receipt which also includes a proof, and the height from which the proof was retrieved */
export const QueryPacketReceiptResponseSchema = z.object({
  /** success flag for if receipt exists */
  received: z.boolean(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketReceiptResponse = z.infer<typeof QueryPacketReceiptResponseSchema>;

/** QueryPacketAcknowledgementRequest is the request type for the Query/PacketAcknowledgement RPC method */
export const QueryPacketAcknowledgementRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** packet sequence */
  sequence: z.string(),
});
export type QueryPacketAcknowledgementRequest = z.infer<typeof QueryPacketAcknowledgementRequestSchema>;

/** QueryPacketAcknowledgementResponse defines the client query response for a packet which also includes a proof and the height from which the proof was retrieved */
export const QueryPacketAcknowledgementResponseSchema = z.object({
  /** packet associated with the request fields */
  acknowledgement: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryPacketAcknowledgementResponse = z.infer<typeof QueryPacketAcknowledgementResponseSchema>;

/** QueryPacketAcknowledgementsRequest is the request type for the Query/QueryPacketCommitments RPC method */
export const QueryPacketAcknowledgementsRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
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

/** QueryUnreceivedPacketsRequest is the request type for the Query/UnreceivedPackets RPC method */
export const QueryUnreceivedPacketsRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
  /** list of packet sequences */
  packet_commitment_sequences: z.array(z.string()),
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
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
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

/** QueryNextSequenceReceiveRequest is the request type for the Query/QueryNextSequenceReceiveRequest RPC method */
export const QueryNextSequenceReceiveRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
});
export type QueryNextSequenceReceiveRequest = z.infer<typeof QueryNextSequenceReceiveRequestSchema>;

/** QuerySequenceResponse is the response type for the Query/QueryNextSequenceReceiveResponse RPC method */
export const QueryNextSequenceReceiveResponseSchema = z.object({
  /** next sequence receive number */
  next_sequence_receive: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryNextSequenceReceiveResponse = z.infer<typeof QueryNextSequenceReceiveResponseSchema>;

/** QueryNextSequenceSendRequest is the request type for the Query/QueryNextSequenceSend RPC method */
export const QueryNextSequenceSendRequestSchema = z.object({
  /** port unique identifier */
  port_id: z.string(),
  /** channel unique identifier */
  channel_id: z.string(),
});
export type QueryNextSequenceSendRequest = z.infer<typeof QueryNextSequenceSendRequestSchema>;

/** QueryNextSequenceSendResponse is the request type for the Query/QueryNextSequenceSend RPC method */
export const QueryNextSequenceSendResponseSchema = z.object({
  /** next sequence send number */
  next_sequence_send: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryNextSequenceSendResponse = z.infer<typeof QueryNextSequenceSendResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.channel.v1' as const;

export const schemas = {
  Channel: ChannelSchema,
  IdentifiedChannel: IdentifiedChannelSchema,
  Counterparty: CounterpartySchema,
  Packet: PacketSchema,
  PacketState: PacketStateSchema,
  PacketId: PacketIdSchema,
  Acknowledgement: AcknowledgementSchema,
  Timeout: TimeoutSchema,
  GenesisState: GenesisStateSchema,
  PacketSequence: PacketSequenceSchema,
  MsgChannelOpenInit: MsgChannelOpenInitSchema,
  MsgChannelOpenInitResponse: MsgChannelOpenInitResponseSchema,
  MsgChannelOpenTry: MsgChannelOpenTrySchema,
  MsgChannelOpenTryResponse: MsgChannelOpenTryResponseSchema,
  MsgChannelOpenAck: MsgChannelOpenAckSchema,
  MsgChannelOpenAckResponse: MsgChannelOpenAckResponseSchema,
  MsgChannelOpenConfirm: MsgChannelOpenConfirmSchema,
  MsgChannelOpenConfirmResponse: MsgChannelOpenConfirmResponseSchema,
  MsgChannelCloseInit: MsgChannelCloseInitSchema,
  MsgChannelCloseInitResponse: MsgChannelCloseInitResponseSchema,
  MsgChannelCloseConfirm: MsgChannelCloseConfirmSchema,
  MsgChannelCloseConfirmResponse: MsgChannelCloseConfirmResponseSchema,
  MsgRecvPacket: MsgRecvPacketSchema,
  MsgRecvPacketResponse: MsgRecvPacketResponseSchema,
  MsgTimeout: MsgTimeoutSchema,
  MsgTimeoutResponse: MsgTimeoutResponseSchema,
  MsgTimeoutOnClose: MsgTimeoutOnCloseSchema,
  MsgTimeoutOnCloseResponse: MsgTimeoutOnCloseResponseSchema,
  MsgAcknowledgement: MsgAcknowledgementSchema,
  MsgAcknowledgementResponse: MsgAcknowledgementResponseSchema,
  QueryChannelRequest: QueryChannelRequestSchema,
  QueryChannelResponse: QueryChannelResponseSchema,
  QueryChannelsRequest: QueryChannelsRequestSchema,
  QueryChannelsResponse: QueryChannelsResponseSchema,
  QueryConnectionChannelsRequest: QueryConnectionChannelsRequestSchema,
  QueryConnectionChannelsResponse: QueryConnectionChannelsResponseSchema,
  QueryChannelClientStateRequest: QueryChannelClientStateRequestSchema,
  QueryChannelClientStateResponse: QueryChannelClientStateResponseSchema,
  QueryChannelConsensusStateRequest: QueryChannelConsensusStateRequestSchema,
  QueryChannelConsensusStateResponse: QueryChannelConsensusStateResponseSchema,
  QueryPacketCommitmentRequest: QueryPacketCommitmentRequestSchema,
  QueryPacketCommitmentResponse: QueryPacketCommitmentResponseSchema,
  QueryPacketCommitmentsRequest: QueryPacketCommitmentsRequestSchema,
  QueryPacketCommitmentsResponse: QueryPacketCommitmentsResponseSchema,
  QueryPacketReceiptRequest: QueryPacketReceiptRequestSchema,
  QueryPacketReceiptResponse: QueryPacketReceiptResponseSchema,
  QueryPacketAcknowledgementRequest: QueryPacketAcknowledgementRequestSchema,
  QueryPacketAcknowledgementResponse: QueryPacketAcknowledgementResponseSchema,
  QueryPacketAcknowledgementsRequest: QueryPacketAcknowledgementsRequestSchema,
  QueryPacketAcknowledgementsResponse: QueryPacketAcknowledgementsResponseSchema,
  QueryUnreceivedPacketsRequest: QueryUnreceivedPacketsRequestSchema,
  QueryUnreceivedPacketsResponse: QueryUnreceivedPacketsResponseSchema,
  QueryUnreceivedAcksRequest: QueryUnreceivedAcksRequestSchema,
  QueryUnreceivedAcksResponse: QueryUnreceivedAcksResponseSchema,
  QueryNextSequenceReceiveRequest: QueryNextSequenceReceiveRequestSchema,
  QueryNextSequenceReceiveResponse: QueryNextSequenceReceiveResponseSchema,
  QueryNextSequenceSendRequest: QueryNextSequenceSendRequestSchema,
  QueryNextSequenceSendResponse: QueryNextSequenceSendResponseSchema,
} as const;
