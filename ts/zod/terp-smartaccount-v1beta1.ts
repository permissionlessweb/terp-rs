// Auto-generated from terp.smartaccount.v1beta1 — do not edit.
// Source: terp-rs/src/gen/terp.smartaccount.v1beta1.rs
// Package: terp.smartaccount.v1beta1
import { z } from 'zod';

/** MsgAddAuthenticatorRequest defines the Msg/AddAuthenticator request type. */
export const MsgAddAuthenticatorSchema = z.object({
  sender: z.string(),
  authenticator_type: z.string(),
  data: z.string(),
});
export type MsgAddAuthenticator = z.infer<typeof MsgAddAuthenticatorSchema>;

/** MsgAddAuthenticatorResponse defines the Msg/AddAuthenticator response type. */
export const MsgAddAuthenticatorResponseSchema = z.object({
  success: z.boolean(),
});
export type MsgAddAuthenticatorResponse = z.infer<typeof MsgAddAuthenticatorResponseSchema>;

/** MsgRemoveAuthenticatorRequest defines the Msg/RemoveAuthenticator request type. */
export const MsgRemoveAuthenticatorSchema = z.object({
  sender: z.string(),
  id: z.string(),
});
export type MsgRemoveAuthenticator = z.infer<typeof MsgRemoveAuthenticatorSchema>;

/** MsgRemoveAuthenticatorResponse defines the Msg/RemoveAuthenticator response type. */
export const MsgRemoveAuthenticatorResponseSchema = z.object({
  success: z.boolean(),
});
export type MsgRemoveAuthenticatorResponse = z.infer<typeof MsgRemoveAuthenticatorResponseSchema>;

export const MsgSetActiveStateSchema = z.object({
  sender: z.string(),
  active: z.boolean(),
});
export type MsgSetActiveState = z.infer<typeof MsgSetActiveStateSchema>;

export const MsgSetActiveStateResponseSchema = z.object({});
export type MsgSetActiveStateResponse = z.infer<typeof MsgSetActiveStateResponseSchema>;

/** AgAuthData is a Serialized array of signing.SignatureV2. We Marshal & Unmarshal via `UnmarshalSignatureJSON` & `MarshalSignatureJSON` */
export const AgAuthDataSchema = z.object({
  data: z.string(),
});
export type AgAuthData = z.infer<typeof AgAuthDataSchema>;

/** TxExtension allows for additional authenticator-specific data in transactions. */
export const TxExtensionSchema = z.object({
  /** selected_authenticators holds the authenticator_id for the chosen authenticator per message. */
  selected_authenticators: z.array(z.string()),
  /** optional, used to provide aggregate key signature data to module for authentication. */
  agg_auth: z.lazy(() => AgAuthDataSchema).optional(),
});
export type TxExtension = z.infer<typeof TxExtensionSchema>;

/** BlsConfig */
export const BlsConfigSchema = z.object({
  /** list of pubkeys that are points in aggregate key set */
  pubkeys: z.array(z.string()),
  /** minimum threshold of points in order for tx to be valid */
  threshold: z.string(),
});
export type BlsConfig = z.infer<typeof BlsConfigSchema>;

/** AccountAuthenticator represents a foundational model for all authenticators. It provides extensibility by allowing concrete types to interpret and validate transactions based on the encapsulated data. */
export const AccountAuthenticatorSchema = z.object({
  /** ID uniquely identifies the authenticator instance. */
  id: z.string(),
  /** Type specifies the category of the AccountAuthenticator. This type information is essential for differentiating authenticators and ensuring precise data retrieval from the storage layer. Config is a versatile field used in conjunction with the specific type of account authenticator to facilitate complex authentication processes. The interpretation of this field is overloaded, enabling multiple authenticators to utilize it for their respective purposes. */
  config: z.string(),
});
export type AccountAuthenticator = z.infer<typeof AccountAuthenticatorSchema>;

/** Params defines the parameters for the module. */
export const ParamsSchema = z.object({
  /** MaximumUnauthenticatedGas defines the maximum amount of gas that can be used to authenticate a transaction in ante handler without having fee payer authenticated. */
  maximum_unauthenticated_gas: z.string(),
  /** IsSmartAccountActive defines the state of the authenticator. If set to false, the authenticator module will not be used and the classic cosmos sdk authentication will be used instead. */
  is_smart_account_active: z.boolean(),
  /** CircuitBreakerControllers defines list of addresses that are allowed to set is_smart_account_active without going through governance. */
  circuit_breaker_controllers: z.array(z.string()),
});
export type Params = z.infer<typeof ParamsSchema>;

/** QueryParamsRequest is request type for the Query/Params RPC method. */
export const QueryParamsRequestSchema = z.object({});
export type QueryParamsRequest = z.infer<typeof QueryParamsRequestSchema>;

/** QueryParamsResponse is response type for the Query/Params RPC method. */
export const QueryParamsResponseSchema = z.object({
  /** params holds all the parameters of this module. */
  params: z.lazy(() => ParamsSchema).optional(),
});
export type QueryParamsResponse = z.infer<typeof QueryParamsResponseSchema>;

/** MsgGetAuthenticatorsRequest defines the Msg/GetAuthenticators request type. */
export const GetAuthenticatorsRequestSchema = z.object({
  account: z.string(),
});
export type GetAuthenticatorsRequest = z.infer<typeof GetAuthenticatorsRequestSchema>;

/** MsgGetAuthenticatorsResponse defines the Msg/GetAuthenticators response type. */
export const GetAuthenticatorsResponseSchema = z.object({
  account_authenticators: z.array(z.lazy(() => AccountAuthenticatorSchema)),
});
export type GetAuthenticatorsResponse = z.infer<typeof GetAuthenticatorsResponseSchema>;

/** MsgGetAuthenticatorRequest defines the Msg/GetAuthenticator request type. */
export const GetAuthenticatorRequestSchema = z.object({
  account: z.string(),
  authenticator_id: z.string(),
});
export type GetAuthenticatorRequest = z.infer<typeof GetAuthenticatorRequestSchema>;

/** MsgGetAuthenticatorResponse defines the Msg/GetAuthenticator response type. */
export const GetAuthenticatorResponseSchema = z.object({
  account_authenticator: z.lazy(() => AccountAuthenticatorSchema).optional(),
});
export type GetAuthenticatorResponse = z.infer<typeof GetAuthenticatorResponseSchema>;

/** AuthenticatorData represents a genesis exported account with Authenticators. The address is used as the key, and the account authenticators are stored in the authenticators field. */
export const AuthenticatorDataSchema = z.object({
  /** address is an account address, one address can have many authenticators */
  address: z.string(),
  /** authenticators are the account's authenticators, these can be multiple types including SignatureVerification, AllOfs, CosmWasmAuthenticators, etc */
  authenticators: z.array(z.lazy(() => AccountAuthenticatorSchema)),
});
export type AuthenticatorData = z.infer<typeof AuthenticatorDataSchema>;

/** GenesisState defines the authenticator module's genesis state. */
export const GenesisStateSchema = z.object({
  /** params define the parameters for the authenticator module. */
  params: z.lazy(() => ParamsSchema).optional(),
  /** next_authenticator_id is the next available authenticator ID. */
  next_authenticator_id: z.string(),
  /** authenticator_data contains the data for multiple accounts, each with their authenticators. */
  authenticator_data: z.array(z.lazy(() => AuthenticatorDataSchema)),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'terp.smartaccount.v1beta1' as const;

export const schemas = {
  MsgAddAuthenticator: MsgAddAuthenticatorSchema,
  MsgAddAuthenticatorResponse: MsgAddAuthenticatorResponseSchema,
  MsgRemoveAuthenticator: MsgRemoveAuthenticatorSchema,
  MsgRemoveAuthenticatorResponse: MsgRemoveAuthenticatorResponseSchema,
  MsgSetActiveState: MsgSetActiveStateSchema,
  MsgSetActiveStateResponse: MsgSetActiveStateResponseSchema,
  AgAuthData: AgAuthDataSchema,
  TxExtension: TxExtensionSchema,
  BlsConfig: BlsConfigSchema,
  AccountAuthenticator: AccountAuthenticatorSchema,
  Params: ParamsSchema,
  QueryParamsRequest: QueryParamsRequestSchema,
  QueryParamsResponse: QueryParamsResponseSchema,
  GetAuthenticatorsRequest: GetAuthenticatorsRequestSchema,
  GetAuthenticatorsResponse: GetAuthenticatorsResponseSchema,
  GetAuthenticatorRequest: GetAuthenticatorRequestSchema,
  GetAuthenticatorResponse: GetAuthenticatorResponseSchema,
  AuthenticatorData: AuthenticatorDataSchema,
  GenesisState: GenesisStateSchema,
} as const;
