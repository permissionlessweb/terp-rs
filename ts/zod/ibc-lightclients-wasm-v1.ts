// Auto-generated from ibc.lightclients.wasm.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.lightclients.wasm.v1.rs
// Package: ibc.lightclients.wasm.v1
import { z } from 'zod';

/** MsgStoreCode defines the request type for the StoreCode rpc. */
export const MsgStoreCodeSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** wasm byte code of light client contract. It can be raw or gzip compressed */
  wasm_byte_code: z.string(),
});
export type MsgStoreCode = z.infer<typeof MsgStoreCodeSchema>;

/** MsgStoreCodeResponse defines the response type for the StoreCode rpc */
export const MsgStoreCodeResponseSchema = z.object({
  /** checksum is the sha256 hash of the stored code */
  checksum: z.string(),
});
export type MsgStoreCodeResponse = z.infer<typeof MsgStoreCodeResponseSchema>;

/** MsgRemoveChecksum defines the request type for the MsgRemoveChecksum rpc. */
export const MsgRemoveChecksumSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** checksum is the sha256 hash to be removed from the store */
  checksum: z.string(),
});
export type MsgRemoveChecksum = z.infer<typeof MsgRemoveChecksumSchema>;

/** MsgStoreChecksumResponse defines the response type for the StoreCode rpc */
export const MsgRemoveChecksumResponseSchema = z.object({});
export type MsgRemoveChecksumResponse = z.infer<typeof MsgRemoveChecksumResponseSchema>;

/** MsgMigrateContract defines the request type for the MigrateContract rpc. */
export const MsgMigrateContractSchema = z.object({
  /** signer address */
  signer: z.string(),
  /** the client id of the contract */
  client_id: z.string(),
  /** checksum is the sha256 hash of the new wasm byte code for the contract */
  checksum: z.string(),
  /** the json encoded message to be passed to the contract on migration */
  msg: z.string(),
});
export type MsgMigrateContract = z.infer<typeof MsgMigrateContractSchema>;

/** MsgMigrateContractResponse defines the response type for the MigrateContract rpc */
export const MsgMigrateContractResponseSchema = z.object({});
export type MsgMigrateContractResponse = z.infer<typeof MsgMigrateContractResponseSchema>;

/** QueryChecksumsRequest is the request type for the Query/Checksums RPC method. */
export const QueryChecksumsRequestSchema = z.object({
  /** pagination defines an optional pagination for the request. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryChecksumsRequest = z.infer<typeof QueryChecksumsRequestSchema>;

/** QueryChecksumsResponse is the response type for the Query/Checksums RPC method. */
export const QueryChecksumsResponseSchema = z.object({
  /** checksums is a list of the hex encoded checksums of all wasm codes stored. */
  checksums: z.array(z.string()),
  /** pagination defines the pagination in the response. */
  pagination: z.unknown() /*  */.optional(),
});
export type QueryChecksumsResponse = z.infer<typeof QueryChecksumsResponseSchema>;

/** QueryCodeRequest is the request type for the Query/Code RPC method. */
export const QueryCodeRequestSchema = z.object({
  /** checksum is a hex encoded string of the code stored. */
  checksum: z.string(),
});
export type QueryCodeRequest = z.infer<typeof QueryCodeRequestSchema>;

/** QueryCodeResponse is the response type for the Query/Code RPC method. */
export const QueryCodeResponseSchema = z.object({
  data: z.string(),
});
export type QueryCodeResponse = z.infer<typeof QueryCodeResponseSchema>;

/** GenesisState defines 08-wasm's keeper genesis state */
export const GenesisStateSchema = z.object({
  /** uploaded light client wasm contracts */
  contracts: z.array(z.lazy(() => ContractSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** Contract stores contract code */
export const ContractSchema = z.object({
  /** contract byte code */
  code_bytes: z.string(),
});
export type Contract = z.infer<typeof ContractSchema>;

/** Wasm light client's Client state */
export const ClientStateSchema = z.object({
  /** bytes encoding the client state of the underlying light client implemented as a Wasm contract. */
  data: z.string(),
  checksum: z.string(),
  latest_height: z.unknown() /*  */.optional(),
});
export type ClientState = z.infer<typeof ClientStateSchema>;

/** Wasm light client's ConsensusState */
export const ConsensusStateSchema = z.object({
  /** bytes encoding the consensus state of the underlying light client implemented as a Wasm contract. */
  data: z.string(),
});
export type ConsensusState = z.infer<typeof ConsensusStateSchema>;

/** Wasm light client message (either header(s) or misbehaviour) */
export const ClientMessageSchema = z.object({
  data: z.string(),
});
export type ClientMessage = z.infer<typeof ClientMessageSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.lightclients.wasm.v1' as const;

export const schemas = {
  MsgStoreCode: MsgStoreCodeSchema,
  MsgStoreCodeResponse: MsgStoreCodeResponseSchema,
  MsgRemoveChecksum: MsgRemoveChecksumSchema,
  MsgRemoveChecksumResponse: MsgRemoveChecksumResponseSchema,
  MsgMigrateContract: MsgMigrateContractSchema,
  MsgMigrateContractResponse: MsgMigrateContractResponseSchema,
  QueryChecksumsRequest: QueryChecksumsRequestSchema,
  QueryChecksumsResponse: QueryChecksumsResponseSchema,
  QueryCodeRequest: QueryCodeRequestSchema,
  QueryCodeResponse: QueryCodeResponseSchema,
  GenesisState: GenesisStateSchema,
  Contract: ContractSchema,
  ClientState: ClientStateSchema,
  ConsensusState: ConsensusStateSchema,
  ClientMessage: ClientMessageSchema,
} as const;
