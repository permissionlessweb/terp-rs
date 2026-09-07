// Auto-generated from ibc.applications.interchain_accounts.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.interchain_accounts.v1.rs
// Package: ibc.applications.interchain_accounts.v1
import { z } from 'zod';

/** An InterchainAccount is defined as a BaseAccount & the address of the account owner on the controller chain */
export const InterchainAccountSchema = z.object({
  base_account: z.unknown() /*  */.optional(),
  account_owner: z.string(),
});
export type InterchainAccount = z.infer<typeof InterchainAccountSchema>;

/** InterchainAccountPacketData is comprised of a raw transaction, type of transaction and optional memo field. */
export const InterchainAccountPacketDataSchema = z.object({
  data: z.string(),
  memo: z.string(),
});
export type InterchainAccountPacketData = z.infer<typeof InterchainAccountPacketDataSchema>;

/** CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain. */
export const CosmosTxSchema = z.object({
  messages: z.array(z.unknown() /* Any */),
});
export type CosmosTx = z.infer<typeof CosmosTxSchema>;

/** Metadata defines a set of protocol specific data encoded into the ICS27 channel version bytestring See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning> */
export const MetadataSchema = z.object({
  /** version defines the ICS27 protocol version */
  version: z.string(),
  /** controller_connection_id is the connection identifier associated with the controller chain */
  controller_connection_id: z.string(),
  /** host_connection_id is the connection identifier associated with the host chain */
  host_connection_id: z.string(),
  /** address defines the interchain account address to be fulfilled upon the OnChanOpenTry handshake step NOTE: the address field is empty on the OnChanOpenInit handshake step */
  address: z.string(),
  /** encoding defines the supported codec format */
  encoding: z.string(),
  /** tx_type defines the type of transactions the interchain account can execute */
  tx_type: z.string(),
});
export type Metadata = z.infer<typeof MetadataSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.interchain_accounts.v1' as const;

export const schemas = {
  InterchainAccount: InterchainAccountSchema,
  InterchainAccountPacketData: InterchainAccountPacketDataSchema,
  CosmosTx: CosmosTxSchema,
  Metadata: MetadataSchema,
} as const;
