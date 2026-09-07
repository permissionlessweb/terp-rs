// Auto-generated from ibc.core.client.v2 — do not edit.
// Source: terp-rs/src/gen/ibc.core.client.v2.rs
// Package: ibc.core.client.v2
import { z } from 'zod';

/** CounterpartyInfo defines the key that the counterparty will use to message our client */
export const CounterpartyInfoSchema = z.object({
  /** merkle prefix key is the prefix that ics provable keys are stored under */
  merkle_prefix: z.array(z.string()),
  /** client identifier is the identifier used to send packet messages to our client */
  client_id: z.string(),
});
export type CounterpartyInfo = z.infer<typeof CounterpartyInfoSchema>;

/** GenesisCounterpartyInfo defines the state associating a client with a counterparty. */
export const GenesisCounterpartyInfoSchema = z.object({
  /** ClientId is the ID of the given client. */
  client_id: z.string(),
  /** CounterpartyInfo is the counterparty info of the given client. */
  counterparty_info: z.lazy(() => CounterpartyInfoSchema).optional(),
});
export type GenesisCounterpartyInfo = z.infer<typeof GenesisCounterpartyInfoSchema>;

/** GenesisState defines the ibc client v2 submodule's genesis state. */
export const GenesisStateSchema = z.object({
  /** counterparty info for each client */
  counterparty_infos: z.array(z.lazy(() => GenesisCounterpartyInfoSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** Config is a **per-client** configuration struct that sets which relayers are allowed to relay v2 IBC messages for a given client. If it is set, then only relayers in the allow list can send v2 messages If it is not set, then the client allows permissionless relaying of v2 messages */
export const ConfigSchema = z.object({
  /** allowed_relayers defines the set of allowed relayers for IBC V2 protocol for the given client */
  allowed_relayers: z.array(z.string()),
});
export type Config = z.infer<typeof ConfigSchema>;

/** MsgRegisterCounterparty defines a message to register a counterparty on a client */
export const MsgRegisterCounterpartySchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** counterparty merkle prefix */
  counterparty_merkle_prefix: z.array(z.string()),
  /** counterparty client identifier */
  counterparty_client_id: z.string(),
  /** signer address */
  signer: z.string(),
});
export type MsgRegisterCounterparty = z.infer<typeof MsgRegisterCounterpartySchema>;

/** MsgRegisterCounterpartyResponse defines the Msg/RegisterCounterparty response type. */
export const MsgRegisterCounterpartyResponseSchema = z.object({});
export type MsgRegisterCounterpartyResponse = z.infer<typeof MsgRegisterCounterpartyResponseSchema>;

/** MsgUpdateClientConfig defines the sdk.Msg type to update the configuration for a given client */
export const MsgUpdateClientConfigSchema = z.object({
  /** client identifier */
  client_id: z.string(),
  /** allowed relayers  NOTE: All fields in the config must be supplied. */
  config: z.lazy(() => ConfigSchema).optional(),
  /** signer address */
  signer: z.string(),
});
export type MsgUpdateClientConfig = z.infer<typeof MsgUpdateClientConfigSchema>;

/** MsgUpdateClientConfigResponse defines the MsgUpdateClientConfig response type. */
export const MsgUpdateClientConfigResponseSchema = z.object({});
export type MsgUpdateClientConfigResponse = z.infer<typeof MsgUpdateClientConfigResponseSchema>;

/** QueryCounterpartyInfoRequest is the request type for the Query/CounterpartyInfo RPC method */
export const QueryCounterpartyInfoRequestSchema = z.object({
  /** client state unique identifier */
  client_id: z.string(),
});
export type QueryCounterpartyInfoRequest = z.infer<typeof QueryCounterpartyInfoRequestSchema>;

/** QueryCounterpartyInfoResponse is the response type for the Query/CounterpartyInfo RPC method. */
export const QueryCounterpartyInfoResponseSchema = z.object({
  counterparty_info: z.lazy(() => CounterpartyInfoSchema).optional(),
});
export type QueryCounterpartyInfoResponse = z.infer<typeof QueryCounterpartyInfoResponseSchema>;

/** QueryConfigRequest is the request type for the Query/Config RPC method */
export const QueryConfigRequestSchema = z.object({
  /** client state unique identifier */
  client_id: z.string(),
});
export type QueryConfigRequest = z.infer<typeof QueryConfigRequestSchema>;

/** QueryConfigResponse is the response type for the Query/Config RPC method */
export const QueryConfigResponseSchema = z.object({
  config: z.lazy(() => ConfigSchema).optional(),
});
export type QueryConfigResponse = z.infer<typeof QueryConfigResponseSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.client.v2' as const;

export const schemas = {
  CounterpartyInfo: CounterpartyInfoSchema,
  GenesisCounterpartyInfo: GenesisCounterpartyInfoSchema,
  GenesisState: GenesisStateSchema,
  Config: ConfigSchema,
  MsgRegisterCounterparty: MsgRegisterCounterpartySchema,
  MsgRegisterCounterpartyResponse: MsgRegisterCounterpartyResponseSchema,
  MsgUpdateClientConfig: MsgUpdateClientConfigSchema,
  MsgUpdateClientConfigResponse: MsgUpdateClientConfigResponseSchema,
  QueryCounterpartyInfoRequest: QueryCounterpartyInfoRequestSchema,
  QueryCounterpartyInfoResponse: QueryCounterpartyInfoResponseSchema,
  QueryConfigRequest: QueryConfigRequestSchema,
  QueryConfigResponse: QueryConfigResponseSchema,
} as const;
