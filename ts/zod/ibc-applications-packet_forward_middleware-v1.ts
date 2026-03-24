// Auto-generated from ibc.applications.packet_forward_middleware.v1 — do not edit.
// Source: terp-rs/src/gen/ibc.applications.packet_forward_middleware.v1.rs
// Package: ibc.applications.packet_forward_middleware.v1
import { z } from 'zod';

/** GenesisState defines the packetforward genesis state */
export const GenesisStateSchema = z.object({
  /** key - information about forwarded packet: src_channel (parsedReceiver.Channel), src_port (parsedReceiver.Port), sequence value - information about original packet for refunding if necessary: retries, srcPacketSender, srcPacket.DestinationChannel, srcPacket.DestinationPort */
  in_flight_packets: z.unknown() /*  */,
});
export type GenesisState = z.infer<typeof GenesisStateSchema>;

/** InFlightPacket contains information about original packet for writing the acknowledgement and refunding if necessary. */
export const InFlightPacketSchema = z.object({
  original_sender_address: z.string(),
  refund_channel_id: z.string(),
  refund_port_id: z.string(),
  packet_src_channel_id: z.string(),
  packet_src_port_id: z.string(),
  packet_timeout_timestamp: z.string(),
  packet_timeout_height: z.string(),
  packet_data: z.string(),
  refund_sequence: z.string(),
  retries_remaining: z.number().int(),
  timeout: z.string(),
  nonrefundable: z.boolean(),
});
export type InFlightPacket = z.infer<typeof InFlightPacketSchema>;


// ── Package metadata ──────────────────────────────────────────
export const PACKAGE = 'ibc.applications.packet_forward_middleware.v1' as const;

export const schemas = {
  GenesisState: GenesisStateSchema,
  InFlightPacket: InFlightPacketSchema,
} as const;
