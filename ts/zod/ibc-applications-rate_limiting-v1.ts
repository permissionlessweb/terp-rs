// Auto-generated from ibc.applications.rate_limiting.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.rate_limiting.v1.rs
// Package: ibc.applications.rate_limiting.v1
import { z } from 'zod';

/** Gov tx to add a new rate limit */
export const MsgAddRateLimitSchema = z.object({
  /** signer defines the x/gov module account address or other authority signing the message */
  signer: z.string(),
  /** Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom */
  denom: z.string(),
  /** ChannelId for the rate limit, on the side of the rate limited chain */
  channel_or_client_id: z.string(),
  /** MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_send: z.string(),
  /** MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_recv: z.string(),
  /** DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day) */
  duration_hours: z.string(),
});
export type MsgAddRateLimit = z.infer<typeof MsgAddRateLimitSchema>;

/** MsgAddRateLimitResponse is the return type for AddRateLimit function. */
export const MsgAddRateLimitResponseSchema = z.object({});
export type MsgAddRateLimitResponse = z.infer<typeof MsgAddRateLimitResponseSchema>;

/** Gov tx to update an existing rate limit */
export const MsgUpdateRateLimitSchema = z.object({
  /** signer defines the x/gov module account address or other authority signing the message */
  signer: z.string(),
  /** Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom */
  denom: z.string(),
  /** ChannelId for the rate limit, on the side of the rate limited chain */
  channel_or_client_id: z.string(),
  /** MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_send: z.string(),
  /** MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_recv: z.string(),
  /** DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day) */
  duration_hours: z.string(),
});
export type MsgUpdateRateLimit = z.infer<typeof MsgUpdateRateLimitSchema>;

/** MsgUpdateRateLimitResponse is the return type for UpdateRateLimit. */
export const MsgUpdateRateLimitResponseSchema = z.object({});
export type MsgUpdateRateLimitResponse = z.infer<typeof MsgUpdateRateLimitResponseSchema>;

/** Gov tx to remove a rate limit */
export const MsgRemoveRateLimitSchema = z.object({
  /** signer defines the x/gov module account address or other authority signing the message */
  signer: z.string(),
  /** Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom */
  denom: z.string(),
  /** ChannelId for the rate limit, on the side of the rate limited chain */
  channel_or_client_id: z.string(),
});
export type MsgRemoveRateLimit = z.infer<typeof MsgRemoveRateLimitSchema>;

/** MsgRemoveRateLimitResponse is the response type for RemoveRateLimit */
export const MsgRemoveRateLimitResponseSchema = z.object({});
export type MsgRemoveRateLimitResponse = z.infer<typeof MsgRemoveRateLimitResponseSchema>;

/** Gov tx to reset the flow on a rate limit */
export const MsgResetRateLimitSchema = z.object({
  /** signer defines the x/gov module account address or other authority signing the message */
  signer: z.string(),
  /** Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom */
  denom: z.string(),
  /** ChannelId for the rate limit, on the side of the rate limited chain */
  channel_or_client_id: z.string(),
});
export type MsgResetRateLimit = z.infer<typeof MsgResetRateLimitSchema>;

/** MsgResetRateLimitResponse is the response type for ResetRateLimit. */
export const MsgResetRateLimitResponseSchema = z.object({});
export type MsgResetRateLimitResponse = z.infer<typeof MsgResetRateLimitResponseSchema>;

/** Path holds the denom and channelID that define the rate limited route */
export const PathSchema = z.object({
  denom: z.string(),
  channel_or_client_id: z.string(),
});
export type Path = z.infer<typeof PathSchema>;

/** Quota defines the rate limit thresholds for transfer packets */
export const QuotaSchema = z.object({
  /** MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_send: z.string(),
  /** MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%) */
  max_percent_recv: z.string(),
  /** DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day) */
  duration_hours: z.string(),
});
export type Quota = z.infer<typeof QuotaSchema>;

/** Flow tracks all the inflows and outflows of a channel. */
export const FlowSchema = z.object({
  /** Inflow defines the total amount of inbound transfers for the given rate limit in the current window */
  inflow: z.string(),
  /** Outflow defines the total amount of outbound transfers for the given rate limit in the current window */
  outflow: z.string(),
  /** ChannelValue stores the total supply of the denom at the start of the rate limit. This is used as the denominator when checking the rate limit threshold The ChannelValue is fixed for the duration of the rate limit window */
  channel_value: z.string(),
});
export type Flow = z.infer<typeof FlowSchema>;

/** RateLimit stores all the context about a given rate limit, including the relevant denom and channel, rate limit thresholds, and current progress towards the limits */
export const RateLimitSchema = z.object({
  path: z.lazy(() => PathSchema).optional(),
  quota: z.lazy(() => QuotaSchema).optional(),
  flow: z.lazy(() => FlowSchema).optional(),
});
export type RateLimit = z.infer<typeof RateLimitSchema>;

/** WhitelistedAddressPair represents a sender-receiver combo that is not subject to rate limit restrictions */
export const WhitelistedAddressPairSchema = z.object({
  sender: z.string(),
  receiver: z.string(),
});
export type WhitelistedAddressPair = z.infer<typeof WhitelistedAddressPairSchema>;

/** HourEpoch is the epoch type. */
export const HourEpochSchema = z.object({
  epoch_number: z.string(),
  duration: z.unknown() /* Duration */.optional(),
  epoch_start_time: z.unknown() /* Timestamp */.optional(),
  epoch_start_height: z.string(),
});
export type HourEpoch = z.infer<typeof HourEpochSchema>;

/** Queries all rate limits */
export const QueryAllRateLimitsRequestSchema = z.object({});
export type QueryAllRateLimitsRequest = z.infer<typeof QueryAllRateLimitsRequestSchema>;

/** QueryAllRateLimitsResponse returns all the rate limits stored on the chain. */
export const QueryAllRateLimitsResponseSchema = z.object({
  rate_limits: z.array(z.lazy(() => RateLimitSchema)),
});
export type QueryAllRateLimitsResponse = z.infer<typeof QueryAllRateLimitsResponseSchema>;

/** Queries a specific rate limit by channel ID and denom */
export const QueryRateLimitRequestSchema = z.object({
  denom: z.string(),
  channel_or_client_id: z.string(),
});
export type QueryRateLimitRequest = z.infer<typeof QueryRateLimitRequestSchema>;

/** QueryRateLimitResponse returns a rate limit by denom and channel_or_client_id combination. */
export const QueryRateLimitResponseSchema = z.object({
  rate_limit: z.lazy(() => RateLimitSchema).optional(),
});
export type QueryRateLimitResponse = z.infer<typeof QueryRateLimitResponseSchema>;

/** Queries all the rate limits for a given chain */
export const QueryRateLimitsByChainIdRequestSchema = z.object({
  chain_id: z.string(),
});
export type QueryRateLimitsByChainIdRequest = z.infer<typeof QueryRateLimitsByChainIdRequestSchema>;

/** QueryRateLimitsByChainIDResponse returns all rate-limits by a chain. */
export const QueryRateLimitsByChainIdResponseSchema = z.object({
  rate_limits: z.array(z.lazy(() => RateLimitSchema)),
});
export type QueryRateLimitsByChainIdResponse = z.infer<typeof QueryRateLimitsByChainIdResponseSchema>;

/** Queries all the rate limits for a given channel or client ID */
export const QueryRateLimitsByChannelOrClientIdRequestSchema = z.object({
  channel_or_client_id: z.string(),
});
export type QueryRateLimitsByChannelOrClientIdRequest = z.infer<typeof QueryRateLimitsByChannelOrClientIdRequestSchema>;

/** QueryRateLimitsByChannelOrClientIDResponse returns all rate-limits by a channel or client id. */
export const QueryRateLimitsByChannelOrClientIdResponseSchema = z.object({
  rate_limits: z.array(z.lazy(() => RateLimitSchema)),
});
export type QueryRateLimitsByChannelOrClientIdResponse = z.infer<typeof QueryRateLimitsByChannelOrClientIdResponseSchema>;

/** Queries all blacklisted denoms */
export const QueryAllBlacklistedDenomsRequestSchema = z.object({});
export type QueryAllBlacklistedDenomsRequest = z.infer<typeof QueryAllBlacklistedDenomsRequestSchema>;

/** QueryAllBlacklistedDenomsResponse returns all the blacklisted denosm. */
export const QueryAllBlacklistedDenomsResponseSchema = z.object({
  denoms: z.array(z.string()),
});
export type QueryAllBlacklistedDenomsResponse = z.infer<typeof QueryAllBlacklistedDenomsResponseSchema>;

/** Queries all whitelisted address pairs */
export const QueryAllWhitelistedAddressesRequestSchema = z.object({});
export type QueryAllWhitelistedAddressesRequest = z.infer<typeof QueryAllWhitelistedAddressesRequestSchema>;

/** QueryAllWhitelistedAddressesResponse returns all whitelisted pairs. */
export const QueryAllWhitelistedAddressesResponseSchema = z.object({
  address_pairs: z.array(z.lazy(() => WhitelistedAddressPairSchema)),
});
export type QueryAllWhitelistedAddressesResponse = z.infer<typeof QueryAllWhitelistedAddressesResponseSchema>;

/** GenesisState defines the ratelimit module's genesis state. */
export const GenesisStateSchema = z.object({
  rate_limits: z.array(z.lazy(() => RateLimitSchema)),
  whitelisted_address_pairs: z.array(z.lazy(() => WhitelistedAddressPairSchema)),
  blacklisted_denoms: z.array(z.string()),
  pending_send_packet_sequence_numbers: z.array(z.string()),
  hour_epoch: z.lazy(() => HourEpochSchema).optional(),
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.rate_limiting.v1' as const;

export const schemas = {
  MsgAddRateLimit: MsgAddRateLimitSchema,
  MsgAddRateLimitResponse: MsgAddRateLimitResponseSchema,
  MsgUpdateRateLimit: MsgUpdateRateLimitSchema,
  MsgUpdateRateLimitResponse: MsgUpdateRateLimitResponseSchema,
  MsgRemoveRateLimit: MsgRemoveRateLimitSchema,
  MsgRemoveRateLimitResponse: MsgRemoveRateLimitResponseSchema,
  MsgResetRateLimit: MsgResetRateLimitSchema,
  MsgResetRateLimitResponse: MsgResetRateLimitResponseSchema,
  Path: PathSchema,
  Quota: QuotaSchema,
  Flow: FlowSchema,
  RateLimit: RateLimitSchema,
  WhitelistedAddressPair: WhitelistedAddressPairSchema,
  HourEpoch: HourEpochSchema,
  QueryAllRateLimitsRequest: QueryAllRateLimitsRequestSchema,
  QueryAllRateLimitsResponse: QueryAllRateLimitsResponseSchema,
  QueryRateLimitRequest: QueryRateLimitRequestSchema,
  QueryRateLimitResponse: QueryRateLimitResponseSchema,
  QueryRateLimitsByChainIdRequest: QueryRateLimitsByChainIdRequestSchema,
  QueryRateLimitsByChainIdResponse: QueryRateLimitsByChainIdResponseSchema,
  QueryRateLimitsByChannelOrClientIdRequest: QueryRateLimitsByChannelOrClientIdRequestSchema,
  QueryRateLimitsByChannelOrClientIdResponse: QueryRateLimitsByChannelOrClientIdResponseSchema,
  QueryAllBlacklistedDenomsRequest: QueryAllBlacklistedDenomsRequestSchema,
  QueryAllBlacklistedDenomsResponse: QueryAllBlacklistedDenomsResponseSchema,
  QueryAllWhitelistedAddressesRequest: QueryAllWhitelistedAddressesRequestSchema,
  QueryAllWhitelistedAddressesResponse: QueryAllWhitelistedAddressesResponseSchema,
  GenesisState: GenesisStateSchema,
} as const;
