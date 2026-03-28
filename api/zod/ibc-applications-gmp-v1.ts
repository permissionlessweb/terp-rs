// Auto-generated from ibc.applications.gmp.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.gmp.v1.rs
// Package: ibc.applications.gmp.v1
import { z } from 'zod';

/** MsgSendCall defines a msg to send a call to a contract/receiver on a ICS27-2 enabled chain. */
export const MsgSendCallSchema = z.object({
  /** the client by which the packet will be sent */
  source_client: z.string(),
  /** the sender address */
  sender: z.string(),
  /** the recipient address on the destination chain */
  receiver: z.string(),
  /** The salt used to generate the caller account address */
  salt: z.string(),
  /** The payload of the call */
  payload: z.string(),
  /** Timeout timestamp in absolute nanoseconds since unix epoch. */
  timeout_timestamp: z.string(),
  /** optional memo */
  memo: z.string(),
  /** optional encoding */
  encoding: z.string(),
});
export type MsgSendCall = z.infer<typeof MsgSendCallSchema>;

/** MsgSendCallResponse defines the Msg/SendCall response type. */
export const MsgSendCallResponseSchema = z.object({
  /** sequence number of the GMP packet sent */
  sequence: z.string(),
});
export type MsgSendCallResponse = z.infer<typeof MsgSendCallResponseSchema>;

/** AccountIdentifier is used to identify a ICS27 account. */
export const AccountIdentifierSchema = z.object({
  /** The (local) client identifier */
  client_id: z.string(),
  /** The sender of the packet */
  sender: z.string(),
  /** The salt of the packet */
  salt: z.string(),
});
export type AccountIdentifier = z.infer<typeof AccountIdentifierSchema>;

/** An ICS27Account is defined as a BaseAccount & the account identifier */
export const Ics27AccountSchema = z.object({
  address: z.string(),
  account_id: z.lazy(() => AccountIdentifierSchema).optional(),
});
export type Ics27Account = z.infer<typeof Ics27AccountSchema>;

/** CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain. */
export const CosmosTxSchema = z.object({
  messages: z.array(z.unknown() /* Any */),
});
export type CosmosTx = z.infer<typeof CosmosTxSchema>;

/** GMPPacketData defines a struct for the packet payload */
export const GmpPacketDataSchema = z.object({
  /** the sender address */
  sender: z.string(),
  /** the recipient address on the destination chain */
  receiver: z.string(),
  /** The salt used to generate the caller account address */
  salt: z.string(),
  /** The payload of the call */
  payload: z.string(),
  /** optional memo */
  memo: z.string(),
});
export type GmpPacketData = z.infer<typeof GmpPacketDataSchema>;

/** Acknowledgement defines a struct for the ICS27-2 acknowledgement */
export const AcknowledgementSchema = z.object({
  /** The result of the call */
  result: z.string(),
});
export type Acknowledgement = z.infer<typeof AcknowledgementSchema>;

/** QueryAccountAddressRequest is the request type for the Query/AccountAddress RPC method. */
export const QueryAccountAddressRequestSchema = z.object({
  /** The (local) client identifier */
  client_id: z.string(),
  /** The sender of the packet */
  sender: z.string(),
  /** The salt of the packet (in hex format) */
  salt: z.string(),
});
export type QueryAccountAddressRequest = z.infer<typeof QueryAccountAddressRequestSchema>;

/** QueryAccountAddressResponse is the response type for the Query/AccountAddress RPC method. */
export const QueryAccountAddressResponseSchema = z.object({
  /** The interchain account address */
  account_address: z.string(),
});
export type QueryAccountAddressResponse = z.infer<typeof QueryAccountAddressResponseSchema>;

/** QueryAccountIdentifierRequest is the request type for querying the account identifier by account address. */
export const QueryAccountIdentifierRequestSchema = z.object({
  account_address: z.string(),
});
export type QueryAccountIdentifierRequest = z.infer<typeof QueryAccountIdentifierRequestSchema>;

/** QueryAccountIdentifierResponse is the response type for querying the account identifier by account address. */
export const QueryAccountIdentifierResponseSchema = z.object({
  account_id: z.lazy(() => AccountIdentifierSchema).optional(),
});
export type QueryAccountIdentifierResponse = z.infer<typeof QueryAccountIdentifierResponseSchema>;

/** GenesisState defines the 27-gmp genesis state */
export const GenesisStateSchema = z.object({
  /** The list of registered ICS27 accounts */
  ics27_accounts: z.array(z.lazy(() => RegisteredIcs27AccountSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** RegisteredICS27Account contains an account identifier and associated interchain account address */
export const RegisteredIcs27AccountSchema = z.object({
  /** / The address of the ics27 account */
  account_address: z.string(),
  /** / The account identifier */
  account_id: z.lazy(() => AccountIdentifierSchema).optional(),
});
export type RegisteredIcs27Account = z.infer<typeof RegisteredIcs27AccountSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.gmp.v1' as const;

export const schemas = {
  MsgSendCall: MsgSendCallSchema,
  MsgSendCallResponse: MsgSendCallResponseSchema,
  AccountIdentifier: AccountIdentifierSchema,
  Ics27Account: Ics27AccountSchema,
  CosmosTx: CosmosTxSchema,
  GmpPacketData: GmpPacketDataSchema,
  Acknowledgement: AcknowledgementSchema,
  QueryAccountAddressRequest: QueryAccountAddressRequestSchema,
  QueryAccountAddressResponse: QueryAccountAddressResponseSchema,
  QueryAccountIdentifierRequest: QueryAccountIdentifierRequestSchema,
  QueryAccountIdentifierResponse: QueryAccountIdentifierResponseSchema,
  GenesisState: GenesisStateSchema,
  RegisteredIcs27Account: RegisteredIcs27AccountSchema,
} as const;
