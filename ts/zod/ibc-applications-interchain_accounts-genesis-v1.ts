// Auto-generated from ibc.applications.interchain_accounts.genesis.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.interchain_accounts.genesis.v1.rs
// Package: ibc.applications.interchain_accounts.genesis.v1
import { z } from 'zod';

/** GenesisState defines the interchain accounts genesis state */
export const GenesisStateSchema = z.object({
  controller_genesis_state: z.lazy(() => ControllerGenesisStateSchema).optional(),
  host_genesis_state: z.lazy(() => HostGenesisStateSchema).optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** ControllerGenesisState defines the interchain accounts controller genesis state */
export const ControllerGenesisStateSchema = z.object({
  active_channels: z.array(z.lazy(() => ActiveChannelSchema)),
  interchain_accounts: z.array(z.lazy(() => RegisteredInterchainAccountSchema)),
  ports: z.array(z.string()),
  params: z.unknown() /* Params */.optional(),
});
export type ControllerGenesisState = z.infer<typeof ControllerGenesisStateSchema>;

/** HostGenesisState defines the interchain accounts host genesis state */
export const HostGenesisStateSchema = z.object({
  active_channels: z.array(z.lazy(() => ActiveChannelSchema)),
  interchain_accounts: z.array(z.lazy(() => RegisteredInterchainAccountSchema)),
  port: z.string(),
  params: z.unknown() /* Params */.optional(),
});
export type HostGenesisState = z.infer<typeof HostGenesisStateSchema>;

/** ActiveChannel contains a connection ID, port ID and associated active channel ID, as well as a boolean flag to indicate if the channel is middleware enabled */
export const ActiveChannelSchema = z.object({
  connection_id: z.string(),
  port_id: z.string(),
  channel_id: z.string(),
  is_middleware_enabled: z.boolean(),
});
export type ActiveChannel = z.infer<typeof ActiveChannelSchema>;

/** RegisteredInterchainAccount contains a connection ID, port ID and associated interchain account address */
export const RegisteredInterchainAccountSchema = z.object({
  connection_id: z.string(),
  port_id: z.string(),
  account_address: z.string(),
});
export type RegisteredInterchainAccount = z.infer<typeof RegisteredInterchainAccountSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.interchain_accounts.genesis.v1' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
  ControllerGenesisState: ControllerGenesisStateSchema,
  HostGenesisState: HostGenesisStateSchema,
  ActiveChannel: ActiveChannelSchema,
  RegisteredInterchainAccount: RegisteredInterchainAccountSchema,
} as const;
