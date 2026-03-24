// Auto-generated from terp.clock.v1 — do not edit.
// Source: terp-rs/src/gen/terp.clock.v1.rs
// Package: terp.clock.v1
import { z } from 'zod';

/** GenesisState - initial state of module */
export const GenesisStateSchema = z.object({
  /** Params of this module */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** Params defines the set of module parameters. */
export const ParamsSchema = z.object({
  /** contract_addresses stores the list of executable contracts to be ticked on every block. */
  contract_addresses: z.array(z.string()),
  contract_gas_limit: z.string(),
});
export type Params = z.infer<typeof ParamsSchema>;

/** MsgUpdateParams is the Msg/UpdateParams request type.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsSchema = z.object({
  /** authority is the address of the governance account. */
  authority: z.string(),
  /** params defines the x/clock parameters to update.  NOTE: All parameters must be supplied. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type MsgUpdateParams = z.infer<typeof MsgUpdateParamsSchema>;

/** MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.  Since: cosmos-sdk 0.47 */
export const MsgUpdateParamsResponseSchema = z.object({});
export type MsgUpdateParamsResponse = z.infer<typeof MsgUpdateParamsResponseSchema>;

/** QueryClockContracts is the request type to get all contracts. */
export const QueryClockContractsSchema = z.object({});
export type QueryClockContracts = z.infer<typeof QueryClockContractsSchema>;

/** QueryClockContractsResponse is the response type for the Query/ClockContracts RPC method. */
export const QueryClockContractsResponseSchema = z.object({
  contract_addresses: z.array(z.string()),
});
export type QueryClockContractsResponse = z.infer<typeof QueryClockContractsResponseSchema>;

/** QueryParams is the request type to get all module params. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryClockContractsResponse is the response type for the Query/ClockContracts RPC method. */
export const QueryParamsResponseSchema = z.object({
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'terp.clock.v1' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
  Params: ParamsSchema,
  MsgUpdateParams: MsgUpdateParamsSchema,
  MsgUpdateParamsResponse: MsgUpdateParamsResponseSchema,
  QueryClockContracts: QueryClockContractsSchema,
  QueryClockContractsResponse: QueryClockContractsResponseSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
} as const;
