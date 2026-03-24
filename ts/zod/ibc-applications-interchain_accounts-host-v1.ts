// Auto-generated from ibc.applications.interchain_accounts.host.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.interchain_accounts.host.v1.rs
// Package: ibc.applications.interchain_accounts.host.v1
import { z } from 'zod';

/** Params defines the set of on-chain interchain accounts parameters. The following parameters may be used to disable the host submodule. */
export const ParamsSchema = z.object({
  /** host_enabled enables or disables the host submodule. */
  host_enabled: z.boolean(),
  /** allow_messages defines a list of sdk message typeURLs allowed to be executed on a host chain. */
  allow_messages: z.array(z.string()),
});
export type Params = z.infer<typeof ParamsSchema>;

/** QueryRequest defines the parameters for a particular query request by an interchain account. */
export const QueryRequestSchema = z.object({
  /** path defines the path of the query request as defined by ADR-021. <https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-021-protobuf-query-encoding.md#custom-query-registration-and-routing> */
  path: z.string(),
  /** data defines the payload of the query request as defined by ADR-021. <https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-021-protobuf-query-encoding.md#custom-query-registration-and-routing> */
  data: z.string(),
});
export type QueryRequest = z.infer<typeof QueryRequestSchema>;

/** MsgUpdateParams defines the payload for Msg/UpdateParams */
export const MsgUpdateParamsSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** params defines the 27-interchain-accounts/host parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response for Msg/UpdateParams */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** MsgModuleQuerySafe defines the payload for Msg/ModuleQuerySafe */
export const MsgModuleQuerySafeSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** requests defines the module safe queries to execute. */
  requests: z.array(z.lazy(() => QueryRequestSchema)),
});
export type MsgModuleQuerySafe = z.infer<typeof MsgModuleQuerySafeSchema>;

/** MsgModuleQuerySafeResponse defines the response for Msg/ModuleQuerySafe */
export const MsgModuleQuerySafeResponseSchema = z.object({
  /** height at which the responses were queried */
  height: z.string(),
  /** protobuf encoded responses for each query */
  responses: z.array(z.string()),
});
export type MsgModuleQuerySafeResponse = z.infer<typeof MsgModuleQuerySafeResponseSchema>;

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
export const PACKAGE = 'ibc.applications.interchain_accounts.host.v1' as const;

export const schemas = {
  Params: ParamsSchema,
  QueryRequest: QueryRequestSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  MsgModuleQuerySafe: MsgModuleQuerySafeSchema,
  MsgModuleQuerySafeResponse: MsgModuleQuerySafeResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
} as const;
