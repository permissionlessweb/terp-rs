// Auto-generated from ibc.core.client.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.core.client.v1.rs
// Package: ibc.core.client.v1
import { z } from 'zod';

/** IdentifiedClientState defines a client state with an additional client identifier field. */
export const IdentifiedClientStateSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** client state */
  client_state: z.unknown() /* Any */.optional(),
});
export type IdentifiedClientState = z.infer<typeof IdentifiedClientStateSchema>;

/** ConsensusStateWithHeight defines a consensus state with an additional height field. */
export const ConsensusStateWithHeightSchema = z.object({
  /** consensus state height */
  height: z.lazy(() => HeightSchema).optional(),
  /** consensus state */
  consensus_state: z.unknown() /* Any */.optional(),
});
export type ConsensusStateWithHeight = z.infer<typeof ConsensusStateWithHeightSchema>;

/** ClientConsensusStates defines all the stored consensus states for a given client. */
export const ClientConsensusStatesSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** consensus states and their heights associated with the client */
  consensus_states: z.array(z.lazy(() => ConsensusStateWithHeightSchema)),
});
export type ClientConsensusStates = z.infer<typeof ClientConsensusStatesSchema>;

/** Height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients  Normally the RevisionHeight is incremented at each height while keeping RevisionNumber the same. However some consensus algorithms may choose to reset the height in certain conditions e.g. hard forks, state-machine breaking changes In these cases, the RevisionNumber is incremented so that height continues to be monitonically increasing even as the RevisionHeight gets reset  Please note that json tags for generated Go code are overridden to explicitly exclude the omitempty jsontag. This enforces the Go json marshaller to always emit zero values for both revision_number and revision_height. */
export const HeightSchema = z.object({
  /** the revision that the client is currently on */
  revision_number: z.string(),
  /** the height within the given revision */
  revision_height: z.string(),
});
export type Height = z.infer<typeof HeightSchema>;

/** Params defines the set of IBC light client parameters. */
export const ParamsSchema = z.object({
  /** allowed_clients defines the list of allowed client state types which can be created and interacted with. If a client type is removed from the allowed clients list, usage of this client will be disabled until it is added again to the list. */
  allowed_clients: z.array(z.string()),
});
export type Params = z.infer<typeof ParamsSchema>;

/** GenesisState defines the ibc client submodule's genesis state. */
export const GenesisStateSchema = z.object({
  /** client states with their corresponding identifiers */
  clients: z.array(z.lazy(() => IdentifiedClientStateSchema)),
  /** consensus states from each client */
  clients_consensus: z.array(z.lazy(() => ClientConsensusStatesSchema)),
  /** metadata from each client */
  clients_metadata: z.array(z.lazy(() => IdentifiedGenesisMetadataSchema)),
  params: z.lazy(() => ParamsSchema).optional(),
  /** Deprecated: create_localhost has been deprecated. The localhost client is automatically created at genesis. */
  create_localhost: z.boolean(),
  /** the sequence for the next generated client identifier */
  next_client_sequence: z.string(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** GenesisMetadata defines the genesis type for metadata that will be used to export all client store keys that are not client or consensus states. */
export const GenesisMetadataSchema = z.object({
  /** store key of metadata without clientID-prefix */
  key: z.string(),
  /** metadata value */
  value: z.string(),
});
export type GenesisMetadata = z.infer<typeof GenesisMetadataSchema>;

/** IdentifiedGenesisMetadata has the client metadata with the corresponding client id. */
export const IdentifiedGenesisMetadataSchema = z.object({
  client_id: z.string(),
  client_metadata: z.array(z.lazy(() => GenesisMetadataSchema)),
});
export type IdentifiedGenesisMetadata = z.infer<typeof IdentifiedGenesisMetadataSchema>;

/** MsgCreateClient defines a message to create an IBC client */
export const MsgCreateClientSchema = z.object({
  /** light client state */
  client_state: z.unknown() /* Any */.optional(),
  /** consensus state associated with the client that corresponds to a given height. */
  consensus_state: z.unknown() /* Any */.optional(),
  /** signer address */
  signer: z.string(),
});
export type MsgCreateClient = z.infer<typeof MsgCreateClientSchema>;

/** MsgCreateClientResponse defines the Msg/CreateClient response type. */
export const MsgCreateClientResponseSchema = z.object({
  client_id: z.string(),
});
export type MsgCreateClientResponse = z.infer<typeof MsgCreateClientResponseSchema>;

/** MsgUpdateClient defines an sdk.Msg to update a IBC client state using the given client message. */
export const MsgUpdateClientSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** client message to update the light client */
  client_message: z.unknown() /* Any */.optional(),
  /** signer address */
  signer: z.string(),
});
export type MsgUpdateClient = z.infer<typeof MsgUpdateClientSchema>;

/** MsgUpdateClientResponse defines the Msg/UpdateClient response type. */
export const MsgUpdateClientResponseSchema = z.object({});
export type MsgUpdateClientResponse = z.infer<typeof MsgUpdateClientResponseSchema>;

/** MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client state */
export const MsgUpgradeClientSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
  /** upgraded client state */
  client_state: z.unknown() /* Any */.optional(),
  /** upgraded consensus state, only contains enough information to serve as a basis of trust in update logic */
  consensus_state: z.unknown() /* Any */.optional(),
  /** proof that old chain committed to new client */
  proof_upgrade_client: z.string(),
  /** proof that old chain committed to new consensus state */
  proof_upgrade_consensus_state: z.string(),
  /** signer address */
  signer: z.string(),
});
export type MsgUpgradeClient = z.infer<typeof MsgUpgradeClientSchema>;

/** MsgUpgradeClientResponse defines the Msg/UpgradeClient response type. */
export const MsgUpgradeClientResponseSchema = z.object({});
export type MsgUpgradeClientResponse = z.infer<typeof MsgUpgradeClientResponseSchema>;

/** MsgRecoverClient defines the message used to recover a frozen or expired client. */
export const MsgRecoverClientSchema = z.object({
  /** the client identifier for the client to be updated if the proposal passes */
  subject_client_id: z.string(),
  /** the substitute client identifier for the client which will replace the subject client */
  substitute_client_id: z.string(),
  /** signer address */
  signer: z.string(),
});
export type MsgRecoverClient = z.infer<typeof MsgRecoverClientSchema>;

/** MsgRecoverClientResponse defines the Msg/RecoverClient response type. */
export const MsgRecoverClientResponseSchema = z.object({});
export type MsgRecoverClientResponse = z.infer<typeof MsgRecoverClientResponseSchema>;

/** MsgIBCSoftwareUpgrade defines the message used to schedule an upgrade of an IBC client using a v1 governance proposal */
export const MsgIbcSoftwareUpgradeSchema = z.object({
  plan: z.unknown() /*  */.optional(),
  /** An UpgradedClientState must be provided to perform an IBC breaking upgrade. This will make the chain commit to the correct upgraded (self) client state before the upgrade occurs, so that connecting chains can verify that the new upgraded client is valid by verifying a proof on the previous version of the chain. This will allow IBC connections to persist smoothly across planned chain upgrades. Correspondingly, the UpgradedClientState field has been deprecated in the Cosmos SDK to allow for this logic to exist solely in the 02-client module. */
  upgraded_client_state: z.unknown() /* Any */.optional(),
  /** signer address */
  signer: z.string(),
});
export type MsgIbcSoftwareUpgrade = z.infer<typeof MsgIbcSoftwareUpgradeSchema>;

/** MsgIBCSoftwareUpgradeResponse defines the Msg/IBCSoftwareUpgrade response type. */
export const MsgIbcSoftwareUpgradeResponseSchema = z.object({});
export type MsgIbcSoftwareUpgradeResponse = z.infer<typeof MsgIbcSoftwareUpgradeResponseSchema>;

/** MsgUpdateParams defines the sdk.Msg type to update the client parameters. */
export const MsgUpdateParamsSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** params defines the client parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the MsgUpdateParams response type. */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** MsgDeleteClientCreator defines a message to delete the client creator of a client */
export const MsgDeleteClientCreatorSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** signer address */
  signer: z.string(),
});
export type MsgDeleteClientCreator = z.infer<typeof MsgDeleteClientCreatorSchema>;

/** MsgDeleteClientCreatorResponse defines the Msg/DeleteClientCreator response type. */
export const MsgDeleteClientCreatorResponseSchema = z.object({});
export type MsgDeleteClientCreatorResponse = z.infer<typeof MsgDeleteClientCreatorResponseSchema>;

/** QueryClientStateRequest is the request type for the Query/ClientState RPC method */
export const QueryClientStateRequestSchema = z.object({
  /** client state unique identifier */
  client_id: z.string(),
});
export type QueryClientStateRequest = z.infer<typeof QueryClientStateRequestSchema>;

/** QueryClientStateResponse is the response type for the Query/ClientState RPC method. Besides the client state, it includes a proof and the height from which the proof was retrieved. */
export const QueryClientStateResponseSchema = z.object({
  /** client state associated with the request identifier */
  client_state: z.unknown() /* Any */.optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.lazy(() => HeightSchema).optional(),
});
export type QueryClientStateResponse = z.infer<typeof QueryClientStateResponseSchema>;

/** QueryClientStatesRequest is the request type for the Query/ClientStates RPC method */
export const QueryClientStatesRequestSchema = z.object({
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryClientStatesRequest = z.infer<typeof QueryClientStatesRequestSchema>;

/** QueryClientStatesResponse is the response type for the Query/ClientStates RPC method. */
export const QueryClientStatesResponseSchema = z.object({
  /** list of stored ClientStates of the chain. */
  client_states: z.array(z.lazy(() => IdentifiedClientStateSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryClientStatesResponse = z.infer<typeof QueryClientStatesResponseSchema>;

/** QueryConsensusStateRequest is the request type for the Query/ConsensusState RPC method. Besides the consensus state, it includes a proof and the height from which the proof was retrieved. */
export const QueryConsensusStateRequestSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** consensus state revision number */
  revision_number: z.string(),
  /** consensus state revision height */
  revision_height: z.string(),
  /** latest_height overrides the height field and queries the latest stored ConsensusState */
  latest_height: z.boolean(),
});
export type QueryConsensusStateRequest = z.infer<typeof QueryConsensusStateRequestSchema>;

/** QueryConsensusStateResponse is the response type for the Query/ConsensusState RPC method */
export const QueryConsensusStateResponseSchema = z.object({
  /** consensus state associated with the client identifier at the given height */
  consensus_state: z.unknown() /* Any */.optional(),
  /** merkle proof of existence */
  proof: z.string(),
  /** height at which the proof was retrieved */
  proof_height: z.lazy(() => HeightSchema).optional(),
});
export type QueryConsensusStateResponse = z.infer<typeof QueryConsensusStateResponseSchema>;

/** QueryConsensusStatesRequest is the request type for the Query/ConsensusStates RPC method. */
export const QueryConsensusStatesRequestSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConsensusStatesRequest = z.infer<typeof QueryConsensusStatesRequestSchema>;

/** QueryConsensusStatesResponse is the response type for the Query/ConsensusStates RPC method */
export const QueryConsensusStatesResponseSchema = z.object({
  /** consensus states associated with the identifier */
  consensus_states: z.array(z.lazy(() => ConsensusStateWithHeightSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConsensusStatesResponse = z.infer<typeof QueryConsensusStatesResponseSchema>;

/** QueryConsensusStateHeightsRequest is the request type for Query/ConsensusStateHeights RPC method. */
export const QueryConsensusStateHeightsRequestSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** pagination request */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConsensusStateHeightsRequest = z.infer<typeof QueryConsensusStateHeightsRequestSchema>;

/** QueryConsensusStateHeightsResponse is the response type for the Query/ConsensusStateHeights RPC method */
export const QueryConsensusStateHeightsResponseSchema = z.object({
  /** consensus state heights */
  consensus_state_heights: z.array(z.lazy(() => HeightSchema)),
  /** pagination response */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryConsensusStateHeightsResponse = z.infer<typeof QueryConsensusStateHeightsResponseSchema>;

/** QueryClientStatusRequest is the request type for the Query/ClientStatus RPC method */
export const QueryClientStatusRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
});
export type QueryClientStatusRequest = z.infer<typeof QueryClientStatusRequestSchema>;

/** QueryClientStatusResponse is the response type for the Query/ClientStatus RPC method. It returns the current status of the IBC client. */
export const QueryClientStatusResponseSchema = z.object({
  status: z.string(),
});
export type QueryClientStatusResponse = z.infer<typeof QueryClientStatusResponseSchema>;

/** QueryClientParamsRequest is the request type for the Query/ClientParams RPC method. */
export const QueryClientParamsRequestSchema = z.object({});
export type QueryClientParamsRequest = z.infer<typeof QueryClientParamsRequestSchema>;

/** QueryClientParamsResponse is the response type for the Query/ClientParams RPC method. */
export const QueryClientParamsResponseSchema = z.object({
  /** params defines the parameters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryClientParamsResponse = z.infer<typeof QueryClientParamsResponseSchema>;

/** QueryClientCreatorRequest is the request type for the Query/ClientCreator RPC method. */
export const QueryClientCreatorRequestSchema = z.object({
  /** client unique identifier */
  client_id: z.string(),
});
export type QueryClientCreatorRequest = z.infer<typeof QueryClientCreatorRequestSchema>;

/** QueryClientCreatorResponse is the response type for the Query/ClientCreator RPC method. */
export const QueryClientCreatorResponseSchema = z.object({
  /** creator of the client */
  creator: z.string(),
});
export type QueryClientCreatorResponse = z.infer<typeof QueryClientCreatorResponseSchema>;

/** QueryUpgradedClientStateRequest is the request type for the Query/UpgradedClientState RPC method */
export const QueryUpgradedClientStateRequestSchema = z.object({});
export type QueryUpgradedClientStateRequest = z.infer<typeof QueryUpgradedClientStateRequestSchema>;

/** QueryUpgradedClientStateResponse is the response type for the Query/UpgradedClientState RPC method. */
export const QueryUpgradedClientStateResponseSchema = z.object({
  /** client state associated with the request identifier */
  upgraded_client_state: z.unknown() /* Any */.optional(),
});
export type QueryUpgradedClientStateResponse = z.infer<typeof QueryUpgradedClientStateResponseSchema>;

/** QueryUpgradedConsensusStateRequest is the request type for the Query/UpgradedConsensusState RPC method */
export const QueryUpgradedConsensusStateRequestSchema = z.object({});
export type QueryUpgradedConsensusStateRequest = z.infer<typeof QueryUpgradedConsensusStateRequestSchema>;

/** QueryUpgradedConsensusStateResponse is the response type for the Query/UpgradedConsensusState RPC method. */
export const QueryUpgradedConsensusStateResponseSchema = z.object({
  /** Consensus state associated with the request identifier */
  upgraded_consensus_state: z.unknown() /* Any */.optional(),
});
export type QueryUpgradedConsensusStateResponse = z.infer<typeof QueryUpgradedConsensusStateResponseSchema>;

/** QueryVerifyMembershipRequest is the request type for the Query/VerifyMembership RPC method */
export const QueryVerifyMembershipRequestSchema = z.object({
  /** client unique identifier. */
  client_id: z.string(),
  /** the proof to be verified by the client. */
  proof: z.string(),
  /** the height of the commitment root at which the proof is verified. */
  proof_height: z.lazy(() => HeightSchema).optional(),
  /** the value which is proven. */
  value: z.string(),
  /** optional time delay */
  time_delay: z.string(),
  /** optional block delay */
  block_delay: z.string(),
  /** the commitment key path. */
  merkle_path: z.unknown() /* MerklePath */.optional(),
});
export type QueryVerifyMembershipRequest = z.infer<typeof QueryVerifyMembershipRequestSchema>;

/** QueryVerifyMembershipResponse is the response type for the Query/VerifyMembership RPC method */
export const QueryVerifyMembershipResponseSchema = z.object({
  /** boolean indicating success or failure of proof verification. */
  success: z.boolean(),
});
export type QueryVerifyMembershipResponse = z.infer<typeof QueryVerifyMembershipResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.client.v1' as const;

export const schemas = {
  IdentifiedClientState: IdentifiedClientStateSchema,
  ConsensusStateWithHeight: ConsensusStateWithHeightSchema,
  ClientConsensusStates: ClientConsensusStatesSchema,
  Height: HeightSchema,
  Params: ParamsSchema,
  GenesisState: GenesisStateSchema,
  GenesisMetadata: GenesisMetadataSchema,
  IdentifiedGenesisMetadata: IdentifiedGenesisMetadataSchema,
  MsgCreateClient: MsgCreateClientSchema,
  MsgCreateClientResponse: MsgCreateClientResponseSchema,
  MsgUpdateClient: MsgUpdateClientSchema,
  MsgUpdateClientResponse: MsgUpdateClientResponseSchema,
  MsgUpgradeClient: MsgUpgradeClientSchema,
  MsgUpgradeClientResponse: MsgUpgradeClientResponseSchema,
  MsgRecoverClient: MsgRecoverClientSchema,
  MsgRecoverClientResponse: MsgRecoverClientResponseSchema,
  MsgIbcSoftwareUpgrade: MsgIbcSoftwareUpgradeSchema,
  MsgIbcSoftwareUpgradeResponse: MsgIbcSoftwareUpgradeResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  MsgDeleteClientCreator: MsgDeleteClientCreatorSchema,
  MsgDeleteClientCreatorResponse: MsgDeleteClientCreatorResponseSchema,
  QueryClientStateRequest: QueryClientStateRequestSchema,
  QueryClientStateResponse: QueryClientStateResponseSchema,
  QueryClientStatesRequest: QueryClientStatesRequestSchema,
  QueryClientStatesResponse: QueryClientStatesResponseSchema,
  QueryConsensusStateRequest: QueryConsensusStateRequestSchema,
  QueryConsensusStateResponse: QueryConsensusStateResponseSchema,
  QueryConsensusStatesRequest: QueryConsensusStatesRequestSchema,
  QueryConsensusStatesResponse: QueryConsensusStatesResponseSchema,
  QueryConsensusStateHeightsRequest: QueryConsensusStateHeightsRequestSchema,
  QueryConsensusStateHeightsResponse: QueryConsensusStateHeightsResponseSchema,
  QueryClientStatusRequest: QueryClientStatusRequestSchema,
  QueryClientStatusResponse: QueryClientStatusResponseSchema,
  QueryClientParamsRequest: QueryClientParamsRequestSchema,
  QueryClientParamsResponse: QueryClientParamsResponseSchema,
  QueryClientCreatorRequest: QueryClientCreatorRequestSchema,
  QueryClientCreatorResponse: QueryClientCreatorResponseSchema,
  QueryUpgradedClientStateRequest: QueryUpgradedClientStateRequestSchema,
  QueryUpgradedClientStateResponse: QueryUpgradedClientStateResponseSchema,
  QueryUpgradedConsensusStateRequest: QueryUpgradedConsensusStateRequestSchema,
  QueryUpgradedConsensusStateResponse: QueryUpgradedConsensusStateResponseSchema,
  QueryVerifyMembershipRequest: QueryVerifyMembershipRequestSchema,
  QueryVerifyMembershipResponse: QueryVerifyMembershipResponseSchema,
} as const;
