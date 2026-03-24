// Auto-generated from terp.feeshare.v1 — do not edit.
// Source: terp-rs/src/gen/terp.feeshare.v1.rs
// Package: terp.feeshare.v1
import { z } from 'zod';

/** FeeShare defines an instance that organizes fee distribution conditions for the owner of a given smart contract */
export const FeeShareSchema = z.object({
  /** contract_address is the bech32 address of a registered contract in string form */
  contract_address: z.string(),
  /** deployer_address is the bech32 address of message sender. It must be the same as the contracts admin address. */
  deployer_address: z.string(),
  /** withdrawer_address is the bech32 address of account receiving the transaction fees. */
  withdrawer_address: z.string(),
});
export type FeeShare = z.infer<typeof FeeShareSchema>;

/** GenesisState defines the module's genesis state. */
export const GenesisStateSchema = z.object({
  /** params are the feeshare module parameters */
  params: z.lazy(() => ParamsSchema).optional(),
  /** FeeShare is a slice of active registered contracts for fee distribution */
  fee_share: z.array(z.lazy(() => FeeShareSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** Params defines the feeshare module params */
export const ParamsSchema = z.object({
  /** enable_feeshare defines a parameter to enable the feeshare module */
  enable_fee_share: z.boolean(),
  /** developer_shares defines the proportion of the transaction fees to be distributed to the registered contract owner */
  developer_shares: z.string(),
  /** allowed_denoms defines the list of denoms that are allowed to be paid to the contract withdraw addresses. If said denom is not in the list, the fees will ONLY be sent to the community pool. If this list is empty, all denoms are allowed. */
  allowed_denoms: z.array(z.string()),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgRegisterFeeShare defines a message that registers a FeeShare */
export const MsgRegisterFeeShareSchema = z.object({
  /** contract_address in bech32 format */
  contract_address: z.string(),
  /** deployer_address is the bech32 address of message sender. It must be the same the contract's admin address */
  deployer_address: z.string(),
  /** withdrawer_address is the bech32 address of account receiving the transaction fees */
  withdrawer_address: z.string(),
});
export type MsgRegisterFeeShare = z.infer<typeof MsgRegisterFeeShareSchema>;

/** MsgRegisterFeeShareResponse defines the MsgRegisterFeeShare response type */
export const MsgRegisterFeeShareResponseSchema = z.object({});
export type MsgRegisterFeeShareResponse = z.infer<typeof MsgRegisterFeeShareResponseSchema>;

/** MsgUpdateFeeShare defines a message that updates the withdrawer address for a registered FeeShare */
export const MsgUpdateFeeShareSchema = z.object({
  /** contract_address in bech32 format */
  contract_address: z.string(),
  /** deployer_address is the bech32 address of message sender. It must be the same the contract's admin address */
  deployer_address: z.string(),
  /** withdrawer_address is the bech32 address of account receiving the transaction fees */
  withdrawer_address: z.string(),
});
export type MsgUpdateFeeShare = z.infer<typeof MsgUpdateFeeShareSchema>;

/** MsgUpdateFeeShareResponse defines the MsgUpdateFeeShare response type */
export const MsgUpdateFeeShareResponseSchema = z.object({});
export type MsgUpdateFeeShareResponse = z.infer<typeof MsgUpdateFeeShareResponseSchema>;

/** MsgCancelFeeShare defines a message that cancels a registered FeeShare */
export const MsgCancelFeeShareSchema = z.object({
  /** contract_address in bech32 format */
  contract_address: z.string(),
  /** deployer_address is the bech32 address of message sender. It must be the same the contract's admin address */
  deployer_address: z.string(),
});
export type MsgCancelFeeShare = z.infer<typeof MsgCancelFeeShareSchema>;

/** MsgCancelFeeShareResponse defines the MsgCancelFeeShare response type */
export const MsgCancelFeeShareResponseSchema = z.object({});
export type MsgCancelFeeShareResponse = z.infer<typeof MsgCancelFeeShareResponseSchema>;

/** MsgUpdateParams is the Msg/UpdateParams request type.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsSchema = z.object({
  /** authority is the address that controls the module (defaults to x/gov unless overwritten). */
  authority: z.string(),
  /** params defines the x/feeshare parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryFeeSharesRequest is the request type for the Query/FeeShares RPC method. */
export const QueryFeeSharesRequestSchema = z.object({
  /** pagination defines an optional pagination for the request. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryFeeSharesRequest = z.infer<typeof QueryFeeSharesRequestSchema>;

/** QueryFeeSharesResponse is the response type for the Query/FeeShares RPC method. */
export const QueryFeeSharesResponseSchema = z.object({
  /** FeeShare is a slice of all stored Reveneue */
  feeshare: z.array(z.lazy(() => FeeShareSchema)),
  /** pagination defines the pagination in the response. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryFeeSharesResponse = z.infer<typeof QueryFeeSharesResponseSchema>;

/** QueryFeeShareRequest is the request type for the Query/FeeShare RPC method. */
export const QueryFeeShareRequestSchema = z.object({
  /** contract_address of a registered contract in bech32 format */
  contract_address: z.string(),
});
export type QueryFeeShareRequest = z.infer<typeof QueryFeeShareRequestSchema>;

/** QueryFeeShareResponse is the response type for the Query/FeeShare RPC method. */
export const QueryFeeShareResponseSchema = z.object({
  /** FeeShare is a stored Reveneue for the queried contract */
  feeshare: z.lazy(() => FeeShareSchema).optional(),
});
export type QueryFeeShareResponse = z.infer<typeof QueryFeeShareResponseSchema>;

/** QueryParamsRequest is the request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is the response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params is the returned FeeShare parameter */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;

/** QueryDeployerFeeSharesRequest is the request type for the Query/DeployerFeeShares RPC method. */
export const QueryDeployerFeeSharesRequestSchema = z.object({
  /** deployer_address in bech32 format */
  deployer_address: z.string(),
  /** pagination defines an optional pagination for the request. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryDeployerFeeSharesRequest = z.infer<typeof QueryDeployerFeeSharesRequestSchema>;

/** QueryDeployerFeeSharesResponse is the response type for the Query/DeployerFeeShares RPC method. */
export const QueryDeployerFeeSharesResponseSchema = z.object({
  /** contract_addresses is the slice of registered contract addresses for a deployer */
  contract_addresses: z.array(z.string()),
  /** pagination defines the pagination in the response. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryDeployerFeeSharesResponse = z.infer<typeof QueryDeployerFeeSharesResponseSchema>;

/** QueryWithdrawerFeeSharesRequest is the request type for the Query/WithdrawerFeeShares RPC method. */
export const QueryWithdrawerFeeSharesRequestSchema = z.object({
  /** withdrawer_address in bech32 format */
  withdrawer_address: z.string(),
  /** pagination defines an optional pagination for the request. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryWithdrawerFeeSharesRequest = z.infer<typeof QueryWithdrawerFeeSharesRequestSchema>;

/** QueryWithdrawerFeeSharesResponse is the response type for the Query/WithdrawerFeeShares RPC method. */
export const QueryWithdrawerFeeSharesResponseSchema = z.object({
  /** contract_addresses is the slice of registered contract addresses for a withdrawer */
  contract_addresses: z.array(z.string()),
  /** pagination defines the pagination in the response. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryWithdrawerFeeSharesResponse = z.infer<typeof QueryWithdrawerFeeSharesResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'terp.feeshare.v1' as const;

export const schemas = {
  FeeShare: FeeShareSchema,
  GenesisState: GenesisStateSchema,
  Params: ParamsSchema,
  MsgRegisterFeeShare: MsgRegisterFeeShareSchema,
  MsgRegisterFeeShareResponse: MsgRegisterFeeShareResponseSchema,
  MsgUpdateFeeShare: MsgUpdateFeeShareSchema,
  MsgUpdateFeeShareResponse: MsgUpdateFeeShareResponseSchema,
  MsgCancelFeeShare: MsgCancelFeeShareSchema,
  MsgCancelFeeShareResponse: MsgCancelFeeShareResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryFeeSharesRequest: QueryFeeSharesRequestSchema,
  QueryFeeSharesResponse: QueryFeeSharesResponseSchema,
  QueryFeeShareRequest: QueryFeeShareRequestSchema,
  QueryFeeShareResponse: QueryFeeShareResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
  QueryDeployerFeeSharesRequest: QueryDeployerFeeSharesRequestSchema,
  QueryDeployerFeeSharesResponse: QueryDeployerFeeSharesResponseSchema,
  QueryWithdrawerFeeSharesRequest: QueryWithdrawerFeeSharesRequestSchema,
  QueryWithdrawerFeeSharesResponse: QueryWithdrawerFeeSharesResponseSchema,
} as const;
