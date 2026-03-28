// Auto-generated from ibc.applications.transfer.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.transfer.v1.rs
// Package: ibc.applications.transfer.v1
import { z } from 'zod';

/** Params defines the set of IBC transfer parameters. NOTE: To prevent a single token from being transferred, set the TransfersEnabled parameter to true and then set the bank module's SendEnabled parameter for the denomination to false. */
export const ParamsSchema = z.object({
  /** send_enabled enables or disables all cross-chain token transfers from this chain. */
  send_enabled: z.boolean(),
  /** receive_enabled enables or disables all cross-chain token transfers to this chain. */
  receive_enabled: z.boolean(),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between ICS20 enabled chains. See ICS Spec here: <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures> */
export const MsgTransferSchema = z.object({
  /** the port on which the packet will be sent */
  source_port: z.string(),
  /** the channel by which the packet will be sent */
  source_channel: z.string(),
  /** token to be transferred */
  token: z.unknown() /*  */.optional(),
  /** the sender address */
  sender: z.string(),
  /** the recipient address on the destination chain */
  receiver: z.string(),
  /** Timeout height relative to the current block height. If you are sending with IBC v1 protocol, either timeout_height or timeout_timestamp must be set. If you are sending with IBC v2 protocol, timeout_timestamp must be set, and timeout_height must be omitted. */
  timeout_height: z.unknown() /*  */.optional(),
  /** Timeout timestamp in absolute nanoseconds since unix epoch. If you are sending with IBC v1 protocol, either timeout_height or timeout_timestamp must be set. If you are sending with IBC v2 protocol, timeout_timestamp must be set. */
  timeout_timestamp: z.string(),
  /** optional memo */
  memo: z.string(),
  /** optional encoding */
  encoding: z.string(),
  /** boolean flag to indicate if the transfer message is sent with the IBC v2 protocol but uses v1 channel identifiers. In this case, the v1 channel identifiers function as aliases to the underlying client ids. This only needs to be set if the channel IDs are V1 channel identifiers. */
  use_aliasing: z.boolean(),
});
export type MsgTransfer = z.infer<typeof MsgTransferSchema>;

/** MsgTransferResponse defines the Msg/Transfer response type. */
export const MsgTransferResponseSchema = z.object({
  /** sequence number of the transfer packet sent */
  sequence: z.string(),
});
export type MsgTransferResponse = z.infer<typeof MsgTransferResponseSchema>;

/** MsgUpdateParams is the Msg/UpdateParams request type. */
export const MsgUpdateParamsSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** params defines the transfer parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message. */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** DenomTrace contains the base denomination for ICS20 fungible tokens and the source tracing information path. */
export const DenomTraceSchema = z.object({
  /** path defines the chain of port/channel identifiers used for tracing the source of the fungible token. */
  path: z.string(),
  /** base denomination of the relayed fungible token. */
  base_denom: z.string(),
});
export type DenomTrace = z.infer<typeof DenomTraceSchema>;

/** Token defines a struct which represents a token to be transferred. */
export const TokenSchema = z.object({
  /** the token denomination */
  denom: z.lazy(() => DenomSchema).optional(),
  /** the token amount to be transferred */
  amount: z.string(),
});
export type Token = z.infer<typeof TokenSchema>;

/** Denom holds the base denom of a Token and a trace of the chains it was sent through. */
export const DenomSchema = z.object({
  /** the base token denomination */
  base: z.string(),
  /** the trace of the token */
  trace: z.array(z.lazy(() => HopSchema)),
});
export type Denom = z.infer<typeof DenomSchema>;

/** Hop defines a port ID, channel ID pair specifying a unique "hop" in a trace */
export const HopSchema = z.object({
  port_id: z.string(),
  channel_id: z.string(),
});
export type Hop = z.infer<typeof HopSchema>;

/** FungibleTokenPacketData defines a struct for the packet payload See FungibleTokenPacketData spec: <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures> */
export const FungibleTokenPacketDataSchema = z.object({
  /** the token denomination to be transferred */
  denom: z.string(),
  /** the token amount to be transferred */
  amount: z.string(),
  /** the sender address */
  sender: z.string(),
  /** the recipient address on the destination chain */
  receiver: z.string(),
  /** optional memo */
  memo: z.string(),
});
export type FungibleTokenPacketData = z.infer<typeof FungibleTokenPacketDataSchema>;

/** QueryParamsRequest is the request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is the response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params defines the parameters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;

/** QueryDenomRequest is the request type for the Query/Denom RPC method */
export const QueryDenomRequestSchema = z.object({
  /** hash (in hex format) or denom (full denom with ibc prefix) of the on chain denomination. */
  hash: z.string(),
});
export type QueryDenomRequest = z.infer<typeof QueryDenomRequestSchema>;

/** QueryDenomResponse is the response type for the Query/Denom RPC method. */
export const QueryDenomResponseSchema = z.object({
  /** denom returns the requested denomination. */
  denom: z.lazy(() => DenomSchema).optional(),
});
export type QueryDenomResponse = z.infer<typeof QueryDenomResponseSchema>;

/** QueryDenomsRequest is the request type for the Query/Denoms RPC method */
export const QueryDenomsRequestSchema = z.object({
  /** pagination defines an optional pagination for the request. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryDenomsRequest = z.infer<typeof QueryDenomsRequestSchema>;

/** QueryDenomsResponse is the response type for the Query/Denoms RPC method. */
export const QueryDenomsResponseSchema = z.object({
  /** denoms returns all denominations. */
  denoms: z.array(z.lazy(() => DenomSchema)),
  /** pagination defines the pagination in the response. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryDenomsResponse = z.infer<typeof QueryDenomsResponseSchema>;

/** QueryDenomHashRequest is the request type for the Query/DenomHash RPC method */
export const QueryDenomHashRequestSchema = z.object({
  /** The denomination trace (\[port_id\]/\[channel_id\])+/\[denom\] */
  trace: z.string(),
});
export type QueryDenomHashRequest = z.infer<typeof QueryDenomHashRequestSchema>;

/** QueryDenomHashResponse is the response type for the Query/DenomHash RPC method. */
export const QueryDenomHashResponseSchema = z.object({
  /** hash (in hex format) of the denomination trace information. */
  hash: z.string(),
});
export type QueryDenomHashResponse = z.infer<typeof QueryDenomHashResponseSchema>;

/** QueryEscrowAddressRequest is the request type for the EscrowAddress RPC method. */
export const QueryEscrowAddressRequestSchema = z.object({
  /** unique port identifier */
  port_id: z.string(),
  /** unique channel identifier */
  channel_id: z.string(),
});
export type QueryEscrowAddressRequest = z.infer<typeof QueryEscrowAddressRequestSchema>;

/** QueryEscrowAddressResponse is the response type of the EscrowAddress RPC method. */
export const QueryEscrowAddressResponseSchema = z.object({
  /** the escrow account address */
  escrow_address: z.string(),
});
export type QueryEscrowAddressResponse = z.infer<typeof QueryEscrowAddressResponseSchema>;

/** QueryTotalEscrowForDenomRequest is the request type for TotalEscrowForDenom RPC method. */
export const QueryTotalEscrowForDenomRequestSchema = z.object({
  denom: z.string(),
});
export type QueryTotalEscrowForDenomRequest = z.infer<typeof QueryTotalEscrowForDenomRequestSchema>;

/** QueryTotalEscrowForDenomResponse is the response type for TotalEscrowForDenom RPC method. */
export const QueryTotalEscrowForDenomResponseSchema = z.object({
  amount: z.unknown() /*  */.optional(),
});
export type QueryTotalEscrowForDenomResponse = z.infer<typeof QueryTotalEscrowForDenomResponseSchema>;

/** Allocation defines the spend limit for a particular port and channel */
export const AllocationSchema = z.object({
  /** the port on which the packet will be sent */
  source_port: z.string(),
  /** the channel by which the packet will be sent */
  source_channel: z.string(),
  /** spend limitation on the channel */
  spend_limit: z.array(z.unknown() /*  */),
  /** allow list of receivers, an empty allow list permits any receiver address */
  allow_list: z.array(z.string()),
  /** allow list of memo strings, an empty list prohibits all memo strings; a list only with "\*" permits any memo string */
  allowed_packet_data: z.array(z.string()),
});
export type Allocation = z.infer<typeof AllocationSchema>;

/** TransferAuthorization allows the grantee to spend up to spend_limit coins from the granter's account for ibc transfer on a specific channel */
export const TransferAuthorizationSchema = z.object({
  /** port and channel amounts */
  allocations: z.array(z.lazy(() => AllocationSchema)),
});
export type TransferAuthorization = z.infer<typeof TransferAuthorizationSchema>;

/** GenesisState defines the ibc-transfer genesis state */
export const GenesisStateSchema = z.object({
  port_id: z.string(),
  denoms: z.array(z.lazy(() => DenomSchema)),
  params: z.lazy(() => ParamsSchema).optional(),
  /** total_escrowed contains the total amount of tokens escrowed by the transfer module */
  total_escrowed: z.array(z.unknown() /*  */),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.transfer.v1' as const;

export const schemas = {
  Params: ParamsSchema,
  MsgTransfer: MsgTransferSchema,
  MsgTransferResponse: MsgTransferResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  DenomTrace: DenomTraceSchema,
  Token: TokenSchema,
  Denom: DenomSchema,
  Hop: HopSchema,
  FungibleTokenPacketData: FungibleTokenPacketDataSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
  QueryDenomRequest: QueryDenomRequestSchema,
  QueryDenomResponse: QueryDenomResponseSchema,
  QueryDenomsRequest: QueryDenomsRequestSchema,
  QueryDenomsResponse: QueryDenomsResponseSchema,
  QueryDenomHashRequest: QueryDenomHashRequestSchema,
  QueryDenomHashResponse: QueryDenomHashResponseSchema,
  QueryEscrowAddressRequest: QueryEscrowAddressRequestSchema,
  QueryEscrowAddressResponse: QueryEscrowAddressResponseSchema,
  QueryTotalEscrowForDenomRequest: QueryTotalEscrowForDenomRequestSchema,
  QueryTotalEscrowForDenomResponse: QueryTotalEscrowForDenomResponseSchema,
  Allocation: AllocationSchema,
  TransferAuthorization: TransferAuthorizationSchema,
  GenesisState: GenesisStateSchema,
} as const;
