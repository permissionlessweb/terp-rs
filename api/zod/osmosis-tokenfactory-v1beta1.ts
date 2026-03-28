// Auto-generated from osmosis.tokenfactory.v1beta1 — do not edit.
// Source: terp-rs/src/gen/osmosis.tokenfactory.v1beta1.rs
// Package: osmosis.tokenfactory.v1beta1
import { z } from 'zod';

/** DenomAuthorityMetadata specifies metadata for addresses that have specific capabilities over a token factory denom. Right now there is only one Admin permission, but is planned to be extended to the future. */
export const DenomAuthorityMetadataSchema = z.object({
  /** Can be empty for no admin, or a valid osmosis address */
  admin: z.string(),
});
export type DenomAuthorityMetadata = z.infer<typeof DenomAuthorityMetadataSchema>;

/** Params defines the parameters for the tokenfactory module. */
export const ParamsSchema = z.object({
  denom_creation_fee: z.array(z.unknown() /*  */),
  /** if denom_creation_fee is an empty array, then this field is used to add more gas consumption to the base cost. <https://github.com/CosmWasm/token-factory/issues/11> */
  denom_creation_gas_consume: z.string(),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgCreateDenom defines the message structure for the CreateDenom gRPC service method. It allows an account to create a new denom. It requires a sender address and a sub denomination. The (sender_address, sub_denomination) tuple must be unique and cannot be re-used.  The resulting denom created is defined as \<factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is originally set to be the creator, but this can be changed later. The token denom does not indicate the current admin. */
export const MsgCreateDenomSchema = z.object({
  sender: z.string(),
  /** subdenom can be up to 44 "alphanumeric" characters long. */
  subdenom: z.string(),
});
export type MsgCreateDenom = z.infer<typeof MsgCreateDenomSchema>;

/** MsgCreateDenomResponse is the return value of MsgCreateDenom It returns the full string of the newly created denom */
export const MsgCreateDenomResponseSchema = z.object({
  new_token_denom: z.string(),
});
export type MsgCreateDenomResponse = z.infer<typeof MsgCreateDenomResponseSchema>;

/** MsgMint is the sdk.Msg type for allowing an admin account to mint more of a token.  For now, we only support minting to the sender account */
export const MsgMintSchema = z.object({
  sender: z.string(),
  amount: z.unknown() /* Coin */.optional(),
  mint_to_address: z.string(),
});
export type MsgMint = z.infer<typeof MsgMintSchema>;

export const MsgMintResponseSchema = z.object({});
export type MsgMintResponse = z.infer<typeof MsgMintResponseSchema>;

/** MsgBurn is the sdk.Msg type for allowing an admin account to burn a token.  For now, we only support burning from the sender account. */
export const MsgBurnSchema = z.object({
  sender: z.string(),
  amount: z.unknown() /* Coin */.optional(),
  burn_from_address: z.string(),
});
export type MsgBurn = z.infer<typeof MsgBurnSchema>;

export const MsgBurnResponseSchema = z.object({});
export type MsgBurnResponse = z.infer<typeof MsgBurnResponseSchema>;

/** MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign adminship of a denom to a new account */
export const MsgChangeAdminSchema = z.object({
  sender: z.string(),
  denom: z.string(),
  new_admin: z.string(),
});
export type MsgChangeAdmin = z.infer<typeof MsgChangeAdminSchema>;

/** MsgChangeAdminResponse defines the response structure for an executed MsgChangeAdmin message. */
export const MsgChangeAdminResponseSchema = z.object({});
export type MsgChangeAdminResponse = z.infer<typeof MsgChangeAdminResponseSchema>;

/** MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set the denom's bank metadata */
export const MsgSetDenomMetadataSchema = z.object({
  sender: z.string(),
  metadata: z.unknown() /*  */.optional(),
});
export type MsgSetDenomMetadata = z.infer<typeof MsgSetDenomMetadataSchema>;

/** MsgSetDenomMetadataResponse defines the response structure for an executed MsgSetDenomMetadata message. */
export const MsgSetDenomMetadataResponseSchema = z.object({});
export type MsgSetDenomMetadataResponse = z.infer<typeof MsgSetDenomMetadataResponseSchema>;

export const MsgForceTransferSchema = z.object({
  sender: z.string(),
  amount: z.unknown() /* Coin */.optional(),
  transfer_from_address: z.string(),
  transfer_to_address: z.string(),
});
export type MsgForceTransfer = z.infer<typeof MsgForceTransferSchema>;

export const MsgForceTransferResponseSchema = z.object({});
export type MsgForceTransferResponse = z.infer<typeof MsgForceTransferResponseSchema>;

/** MsgUpdateParams is the Msg/UpdateParams request type.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsSchema = z.object({
  /** sender is the address of the governance account. */
  authority: z.string(),
  /** params defines the x/mint parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryParamsRequest is the request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is the response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params defines the parameters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;

/** QueryDenomAuthorityMetadataRequest defines the request structure for the DenomAuthorityMetadata gRPC query. */
export const QueryDenomAuthorityMetadataRequestSchema = z.object({
  denom: z.string(),
});
export type QueryDenomAuthorityMetadataRequest = z.infer<typeof QueryDenomAuthorityMetadataRequestSchema>;

/** QueryDenomAuthorityMetadataResponse defines the response structure for the DenomAuthorityMetadata gRPC query. */
export const QueryDenomAuthorityMetadataResponseSchema = z.object({
  authority_metadata: z.lazy(() => DenomAuthorityMetadataSchema).optional(),
});
export type QueryDenomAuthorityMetadataResponse = z.infer<typeof QueryDenomAuthorityMetadataResponseSchema>;

/** QueryDenomsFromCreatorRequest defines the request structure for the DenomsFromCreator gRPC query. */
export const QueryDenomsFromCreatorRequestSchema = z.object({
  creator: z.string(),
});
export type QueryDenomsFromCreatorRequest = z.infer<typeof QueryDenomsFromCreatorRequestSchema>;

/** QueryDenomsFromCreatorRequest defines the response structure for the DenomsFromCreator gRPC query. */
export const QueryDenomsFromCreatorResponseSchema = z.object({
  denoms: z.array(z.string()),
});
export type QueryDenomsFromCreatorResponse = z.infer<typeof QueryDenomsFromCreatorResponseSchema>;

/** GenesisState defines the tokenfactory module's genesis state. */
export const GenesisStateSchema = z.object({
  /** params defines the paramaters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
  factory_denoms: z.array(z.lazy(() => GenesisDenomSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** GenesisDenom defines a tokenfactory denom that is defined within genesis state. The structure contains DenomAuthorityMetadata which defines the denom's admin. */
export const GenesisDenomSchema = z.object({
  denom: z.string(),
  authority_metadata: z.lazy(() => DenomAuthorityMetadataSchema).optional(),
});
export type GenesisDenom = z.infer<typeof GenesisDenomSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'osmosis.tokenfactory.v1beta1' as const;

export const schemas = {
  DenomAuthorityMetadata: DenomAuthorityMetadataSchema,
  Params: ParamsSchema,
  MsgCreateDenom: MsgCreateDenomSchema,
  MsgCreateDenomResponse: MsgCreateDenomResponseSchema,
  MsgMint: MsgMintSchema,
  MsgMintResponse: MsgMintResponseSchema,
  MsgBurn: MsgBurnSchema,
  MsgBurnResponse: MsgBurnResponseSchema,
  MsgChangeAdmin: MsgChangeAdminSchema,
  MsgChangeAdminResponse: MsgChangeAdminResponseSchema,
  MsgSetDenomMetadata: MsgSetDenomMetadataSchema,
  MsgSetDenomMetadataResponse: MsgSetDenomMetadataResponseSchema,
  MsgForceTransfer: MsgForceTransferSchema,
  MsgForceTransferResponse: MsgForceTransferResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
  QueryDenomAuthorityMetadataRequest: QueryDenomAuthorityMetadataRequestSchema,
  QueryDenomAuthorityMetadataResponse: QueryDenomAuthorityMetadataResponseSchema,
  QueryDenomsFromCreatorRequest: QueryDenomsFromCreatorRequestSchema,
  QueryDenomsFromCreatorResponse: QueryDenomsFromCreatorResponseSchema,
  GenesisState: GenesisStateSchema,
  GenesisDenom: GenesisDenomSchema,
} as const;
