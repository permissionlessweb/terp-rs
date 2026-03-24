// Auto-generated from terp.drip.v1 — do not edit.
// Source: terp-rs/src/gen/terp.drip.v1.rs
// Package: terp.drip.v1
import { z } from 'zod';

/** GenesisState defines the module's genesis state. */
export const GenesisStateSchema = z.object({
  /** params are the drip module parameters */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** Params defines the drip module params */
export const ParamsSchema = z.object({
  /** enable_drip defines a parameter to enable the drip module */
  enable_drip: z.boolean(),
  /** allowed_addresses defines the list of addresses authorized to use the module */
  allowed_addresses: z.array(z.string()),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgDistributeTokens defines a message that registers a Distribution of tokens. */
export const MsgDistributeTokensSchema = z.object({
  /** sender_address is the bech32 address of message sender. */
  sender_address: z.string(),
  /** amount is the amount being airdropped to stakers */
  amount: z.array(z.unknown() /*  */),
});
export type MsgDistributeTokens = z.infer<typeof MsgDistributeTokensSchema>;

/** MsgDistributeTokensResponse defines the MsgDistributeTokens response type */
export const MsgDistributeTokensResponseSchema = z.object({});
export type MsgDistributeTokensResponse = z.infer<typeof MsgDistributeTokensResponseSchema>;

/** MsgUpdateParams is the Msg/UpdateParams request type.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsSchema = z.object({
  /** authority is the address that controls the module (defaults to x/gov unless overwritten). */
  authority: z.string(),
  /** params defines the x/auth parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryParamsRequest is the request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is the response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params is the returned parameter from the module */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'terp.drip.v1' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
  Params: ParamsSchema,
  MsgDistributeTokens: MsgDistributeTokensSchema,
  MsgDistributeTokensResponse: MsgDistributeTokensResponseSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
} as const;
