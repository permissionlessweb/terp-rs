// Auto-generated from ibc.applications.interchain_accounts.controller.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.interchain_accounts.controller.v1.rs
// Package: ibc.applications.interchain_accounts.controller.v1
import { z } from 'zod';

/** Params defines the set of on-chain interchain accounts parameters. The following parameters may be used to disable the controller submodule. */
export const ParamsSchema = z.object({
  /** controller_enabled enables or disables the controller submodule. */
  controller_enabled: z.boolean(),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgRegisterInterchainAccount defines the payload for Msg/RegisterAccount */
export const MsgRegisterInterchainAccountSchema = z.object({
  owner: z.string(),
  connection_id: z.string(),
  version: z.string(),
});
export type MsgRegisterInterchainAccount = z.infer<typeof MsgRegisterInterchainAccountSchema>;

/** MsgRegisterInterchainAccountResponse defines the response for Msg/RegisterAccount */
export const MsgRegisterInterchainAccountResponseSchema = z.object({
  channel_id: z.string(),
  port_id: z.string(),
});
export type MsgRegisterInterchainAccountResponse = z.infer<typeof MsgRegisterInterchainAccountResponseSchema>;

/** MsgSendTx defines the payload for Msg/SendTx */
export const MsgSendTxSchema = z.object({
  owner: z.string(),
  connection_id: z.string(),
  packet_data: z.unknown() /*  */.optional(),
  /** Relative timeout timestamp provided will be added to the current block time during transaction execution. The timeout timestamp must be non-zero. */
  relative_timeout: z.string(),
});
export type MsgSendTx = z.infer<typeof MsgSendTxSchema>;

/** MsgSendTxResponse defines the response for MsgSendTx */
export const MsgSendTxResponseSchema = z.object({
  sequence: z.string(),
});
export type MsgSendTxResponse = z.infer<typeof MsgSendTxResponseSchema>;

/** MsgUpdateParams defines the payload for Msg/UpdateParams */
export const MsgUpdateParamsSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** params defines the 27-interchain-accounts/controller parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response for Msg/UpdateParams */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryInterchainAccountRequest is the request type for the Query/InterchainAccount RPC method. */
export const QueryInterchainAccountRequestSchema = z.object({
  owner: z.string(),
  connection_id: z.string(),
});
export type QueryInterchainAccountRequest = z.infer<typeof QueryInterchainAccountRequestSchema>;

/** QueryInterchainAccountResponse the response type for the Query/InterchainAccount RPC method. */
export const QueryInterchainAccountResponseSchema = z.object({
  address: z.string(),
});
export type QueryInterchainAccountResponse = z.infer<typeof QueryInterchainAccountResponseSchema>;

/** QueryParamsRequest is the request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is the response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params defines the parameters of the module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.interchain_accounts.controller.v1' as const;

export const schemas = {
  Params: ParamsSchema,
  MsgRegisterInterchainAccount: MsgRegisterInterchainAccountSchema,
  MsgRegisterInterchainAccountResponse: MsgRegisterInterchainAccountResponseSchema,
  MsgSendTx: MsgSendTxSchema,
  MsgSendTxResponse: MsgSendTxResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryInterchainAccountRequest: QueryInterchainAccountRequestSchema,
  QueryInterchainAccountResponse: QueryInterchainAccountResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
} as const;
