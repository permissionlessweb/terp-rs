// Auto-generated from ibc.core.connection.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.core.connection.v1.rs
// Package: ibc.core.connection.v1
import { z } from 'zod';

/** ConnectionEnd defines a stateful object on a chain connected to another separate one. NOTE: there must only be 2 defined ConnectionEnds to establish a connection between two chains. */
export const ConnectionEndSchema = z.object({
  /** client associated with this connection. */
  client_id: z.string(),
  /** IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection. */
  versions: z.array(z.lazy(() => VersionSchema)),
  /** current state of the connection end. */
  state: z.number().int(),
  /** counterparty chain associated with this connection. */
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  /** delay period that must pass before a consensus state can be used for packet-verification NOTE: delay period logic is only implemented by some clients. */
  delay_period: z.string(),
});
export type ConnectionEnd = z.infer<typeof ConnectionEndSchema>;

/** IdentifiedConnection defines a connection with additional connection identifier field. */
export const IdentifiedConnectionSchema = z.object({
  /** connection identifier. */
  id: z.string(),
  /** client associated with this connection. */
  client_id: z.string(),
  /** IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection */
  versions: z.array(z.lazy(() => VersionSchema)),
  /** current state of the connection end. */
  state: z.number().int(),
  /** counterparty chain associated with this connection. */
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  /** delay period associated with this connection. */
  delay_period: z.string(),
});
export type IdentifiedConnection = z.infer<typeof IdentifiedConnectionSchema>;

/** Counterparty defines the counterparty chain associated with a connection end. */
export const CounterpartySchema = z.object({
  /** identifies the client on the counterparty chain associated with a given connection. */
  client_id: z.string(),
  /** identifies the connection end on the counterparty chain associated with a given connection. */
  connection_id: z.string(),
  /** commitment merkle prefix of the counterparty chain. */
  prefix: z.unknown() /* MerklePrefix */.optional(),
});
export type Counterparty = z.infer<typeof CounterpartySchema>;

/** ClientPaths define all the connection paths for a client state. */
export const ClientPathsSchema = z.object({
  /** list of connection paths */
  paths: z.array(z.string()),
});
export type ClientPaths = z.infer<typeof ClientPathsSchema>;

/** ConnectionPaths define all the connection paths for a given client state. */
export const ConnectionPathsSchema = z.object({
  /** client state unique identifier */
  client_id: z.string(),
  /** list of connection paths */
  paths: z.array(z.string()),
});
export type ConnectionPaths = z.infer<typeof ConnectionPathsSchema>;

/** Version defines the versioning scheme used to negotiate the IBC version in the connection handshake. */
export const VersionSchema = z.object({
  /** unique version identifier */
  identifier: z.string(),
  /** list of features compatible with the specified identifier */
  features: z.array(z.string()),
});
export type Version = z.infer<typeof VersionSchema>;

/** Params defines the set of Connection parameters. */
export const ParamsSchema = z.object({
  /** maximum expected time per block (in nanoseconds), used to enforce block delay. This parameter should reflect the largest amount of time that the chain might reasonably take to produce the next block under normal operating conditions. A safe choice is 3-5x the expected time per block. */
  max_expected_time_per_block: z.string(),
});
export type Params = z.infer<typeof ParamsSchema>;

/** GenesisState defines the ibc connection submodule's genesis state. */
export const GenesisStateSchema = z.object({
  connections: z.array(z.lazy(() => IdentifiedConnectionSchema)),
  client_connection_paths: z.array(z.lazy(() => ConnectionPathsSchema)),
  /** the sequence for the next generated connection identifier */
  next_connection_sequence: z.string(),
  params: z.lazy(() => ParamsSchema).optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** MsgConnectionOpenInit defines the msg sent by an account on Chain A to initialize a connection with Chain B. */
export const MsgConnectionOpenInitSchema = z.object({
  client_id: z.string(),
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  version: z.lazy(() => VersionSchema).optional(),
  delay_period: z.string(),
  signer: z.string(),
});
export type MsgConnectionOpenInit = z.infer<typeof MsgConnectionOpenInitSchema>;

/** MsgConnectionOpenInitResponse defines the Msg/ConnectionOpenInit response type. */
export const MsgConnectionOpenInitResponseSchema = z.object({});
export type MsgConnectionOpenInitResponse = z.infer<typeof MsgConnectionOpenInitResponseSchema>;

/** MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a connection on Chain B. */
export const MsgConnectionOpenTrySchema = z.object({
  client_id: z.string(),
  /** Deprecated: this field is unused. Crossing hellos are no longer supported in core IBC. */
  previous_connection_id: z.string(),
  /** Deprecated: this field is unused. */
  client_state: z.unknown() /* Any */.optional(),
  counterparty: z.lazy(() => CounterpartySchema).optional(),
  delay_period: z.string(),
  counterparty_versions: z.array(z.lazy(() => VersionSchema)),
  proof_height: z.unknown() /* Height */.optional(),
  /** proof of the initialization the connection on Chain A: `UNINITIALIZED ->  INIT` */
  proof_init: z.string(),
  /** Deprecated: this field is unused. */
  proof_client: z.string(),
  /** Deprecated: this field is unused. */
  proof_consensus: z.string(),
  /** Deprecated: this field is unused. */
  consensus_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
  /** Deprecated: this field is unused. */
  host_consensus_state_proof: z.string(),
});
export type MsgConnectionOpenTry = z.infer<typeof MsgConnectionOpenTrySchema>;

/** MsgConnectionOpenTryResponse defines the Msg/ConnectionOpenTry response type. */
export const MsgConnectionOpenTryResponseSchema = z.object({});
export type MsgConnectionOpenTryResponse = z.infer<typeof MsgConnectionOpenTryResponseSchema>;

/** MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to acknowledge the change of connection state to TRYOPEN on Chain B. */
export const MsgConnectionOpenAckSchema = z.object({
  connection_id: z.string(),
  counterparty_connection_id: z.string(),
  version: z.lazy(() => VersionSchema).optional(),
  /** Deprecated: this field is unused. */
  client_state: z.unknown() /* Any */.optional(),
  proof_height: z.unknown() /* Height */.optional(),
  /** proof of the initialization the connection on Chain B: `UNINITIALIZED ->  TRYOPEN` */
  proof_try: z.string(),
  /** Deprecated: this field is unused. */
  proof_client: z.string(),
  /** Deprecated: this field is unused. */
  proof_consensus: z.string(),
  /** Deprecated: this field is unused. */
  consensus_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
  /** Deprecated: this field is unused. */
  host_consensus_state_proof: z.string(),
});
export type MsgConnectionOpenAck = z.infer<typeof MsgConnectionOpenAckSchema>;

/** MsgConnectionOpenAckResponse defines the Msg/ConnectionOpenAck response type. */
export const MsgConnectionOpenAckResponseSchema = z.object({});
export type MsgConnectionOpenAckResponse = z.infer<typeof MsgConnectionOpenAckResponseSchema>;

/** MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of connection state to OPEN on Chain A. */
export const MsgConnectionOpenConfirmSchema = z.object({
  connection_id: z.string(),
  /** proof for the change of the connection state on Chain A: `INIT -> OPEN` */
  proof_ack: z.string(),
  proof_height: z.unknown() /* Height */.optional(),
  signer: z.string(),
});
export type MsgConnectionOpenConfirm = z.infer<typeof MsgConnectionOpenConfirmSchema>;

/** MsgConnectionOpenConfirmResponse defines the Msg/ConnectionOpenConfirm response type. */
export const MsgConnectionOpenConfirmResponseSchema = z.object({});
export type MsgConnectionOpenConfirmResponse = z.infer<typeof MsgConnectionOpenConfirmResponseSchema>;

/** MsgUpdateParams defines the sdk.Msg type to update the connection parameters. */
export const MsgUpdateParamsSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** params defines the connection parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the MsgUpdateParams response type. */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryConnectionRequest is the request type for the Query/Connection RPC method */
export const QueryConnectionRequestSchema = z.object({
  /** connection unique identifier */
  connection_id: z.string(),
});
export type QueryConnectionRequest = z.infer<typeof QueryConnectionRequestSchema>;

/** QueryConnectionResponse is the response type for the Query/Connection RPC method. Besides the connection end, it includes a proof and the height from which the proof was retrieved. */
export const QueryConnectionResponseSchema = z.object({
  /** connection associated with the request identifier */
  connection: z.lazy(() => ConnectionEndSchema).optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryConnectionResponse = z.infer<typeof QueryConnectionResponseSchema>;

/** QueryConnectionsRequest is the request type for the Query/Connections RPC method */
export const QueryConnectionsRequestSchema = z.object({
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConnectionsRequest = z.infer<typeof QueryConnectionsRequestSchema>;

/** QueryConnectionsResponse is the response type for the Query/Connections RPC method. */
export const QueryConnectionsResponseSchema = z.object({
  /** list of stored connections of the chain. */
  connections: z.array(z.lazy(() => IdentifiedConnectionSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
  /** query block height */
  height: z.unknown() /* Height */.optional(),
});
export type QueryConnectionsResponse = z.infer<typeof QueryConnectionsResponseSchema>;

/** QueryClientConnectionsRequest is the request type for the Query/ClientConnections RPC method */
export const QueryClientConnectionsRequestSchema = z.object({
  /** client identifier associated with a connection */
  client_id: z.string(),
});
export type QueryClientConnectionsRequest = z.infer<typeof QueryClientConnectionsRequestSchema>;

/** QueryClientConnectionsResponse is the response type for the Query/ClientConnections RPC method */
export const QueryClientConnectionsResponseSchema = z.object({
  /** slice of all the connection paths associated with a client. */
  connection_paths: z.array(z.string()),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was generated */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryClientConnectionsResponse = z.infer<typeof QueryClientConnectionsResponseSchema>;

/** QueryConnectionClientStateRequest is the request type for the Query/ConnectionClientState RPC method */
export const QueryConnectionClientStateRequestSchema = z.object({
  /** connection identifier */
  connection_id: z.string(),
});
export type QueryConnectionClientStateRequest = z.infer<typeof QueryConnectionClientStateRequestSchema>;

/** QueryConnectionClientStateResponse is the response type for the Query/ConnectionClientState RPC method */
export const QueryConnectionClientStateResponseSchema = z.object({
  /** client state associated with the channel */
  identified_client_state: z.unknown() /*  */.optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryConnectionClientStateResponse = z.infer<typeof QueryConnectionClientStateResponseSchema>;

/** QueryConnectionConsensusStateRequest is the request type for the Query/ConnectionConsensusState RPC method */
export const QueryConnectionConsensusStateRequestSchema = z.object({
  /** connection identifier */
  connection_id: z.string(),
  revision_number: z.string(),
  revision_height: z.string(),
});
export type QueryConnectionConsensusStateRequest = z.infer<typeof QueryConnectionConsensusStateRequestSchema>;

/** QueryConnectionConsensusStateResponse is the response type for the Query/ConnectionConsensusState RPC method */
export const QueryConnectionConsensusStateResponseSchema = z.object({
  /** consensus state associated with the channel */
  consensus_state: z.unknown() /* Any */.optional(),
  /** client ID associated with the consensus state */
  client_id: z.string(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.unknown() /* Height */.optional(),
});
export type QueryConnectionConsensusStateResponse = z.infer<typeof QueryConnectionConsensusStateResponseSchema>;

/** QueryConnectionParamsRequest is the request type for the Query/ConnectionParams RPC method. */
export const QueryConnectionParamsRequestSchema = z.object({});
export type QueryConnectionParamsRequest = z.infer<typeof QueryConnectionParamsRequestSchema>;

/** QueryConnectionParamsResponse is the response type for the Query/ConnectionParams RPC method. */
export const QueryConnectionParamsResponseSchema = z.object({
  /** params defines the parameters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryConnectionParamsResponse = z.infer<typeof QueryConnectionParamsResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.connection.v1' as const;

export const schemas = {
  ConnectionEnd: ConnectionEndSchema,
  IdentifiedConnection: IdentifiedConnectionSchema,
  Counterparty: CounterpartySchema,
  ClientPaths: ClientPathsSchema,
  ConnectionPaths: ConnectionPathsSchema,
  Version: VersionSchema,
  Params: ParamsSchema,
  GenesisState: GenesisStateSchema,
  MsgConnectionOpenInit: MsgConnectionOpenInitSchema,
  MsgConnectionOpenInitResponse: MsgConnectionOpenInitResponseSchema,
  MsgConnectionOpenTry: MsgConnectionOpenTrySchema,
  MsgConnectionOpenTryResponse: MsgConnectionOpenTryResponseSchema,
  MsgConnectionOpenAck: MsgConnectionOpenAckSchema,
  MsgConnectionOpenAckResponse: MsgConnectionOpenAckResponseSchema,
  MsgConnectionOpenConfirm: MsgConnectionOpenConfirmSchema,
  MsgConnectionOpenConfirmResponse: MsgConnectionOpenConfirmResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryConnectionRequest: QueryConnectionRequestSchema,
  QueryConnectionResponse: QueryConnectionResponseSchema,
  QueryConnectionsRequest: QueryConnectionsRequestSchema,
  QueryConnectionsResponse: QueryConnectionsResponseSchema,
  QueryClientConnectionsRequest: QueryClientConnectionsRequestSchema,
  QueryClientConnectionsResponse: QueryClientConnectionsResponseSchema,
  QueryConnectionClientStateRequest: QueryConnectionClientStateRequestSchema,
  QueryConnectionClientStateResponse: QueryConnectionClientStateResponseSchema,
  QueryConnectionConsensusStateRequest: QueryConnectionConsensusStateRequestSchema,
  QueryConnectionConsensusStateResponse: QueryConnectionConsensusStateResponseSchema,
  QueryConnectionParamsRequest: QueryConnectionParamsRequestSchema,
  QueryConnectionParamsResponse: QueryConnectionParamsResponseSchema,
} as const;
