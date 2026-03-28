// Auto-generated from ibc.core.types.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.core.types.v1.rs
// Package: ibc.core.types.v1
import { z } from 'zod';

/** GenesisState defines the ibc module's genesis state. */
export const GenesisStateSchema = z.object({
  /** ICS002 - Clients genesis state */
  client_genesis: z.lazy(() => GenesisStateSchema).optional(),
  /** ICS003 - Connections genesis state */
  connection_genesis: z.unknown() /*  */.optional(),
  /** ICS004 - Channel genesis state */
  channel_genesis: z.lazy(() => GenesisStateSchema).optional(),
  /** ICS002 - Clients/v2 genesis state */
  client_v2_genesis: z.unknown() /*  */.optional(),
  /** ICS004 - Channel/v2 genesis state */
  channel_v2_genesis: z.unknown() /*  */.optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.core.types.v1' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
} as const;
