# Auto-generated from ibc.applications.packet_forward_middleware.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.packet_forward_middleware.v1.rs
# Package: ibc.applications.packet_forward_middleware.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.packet_forward_middleware.v1"

@dataclass
class GenesisState:
    """GenesisState defines the packetforward genesis state"""
    # key - information about forwarded packet: src_channel (parsedReceiver.Channel), src_port (parsedReceiver.Port), sequence value - information about original packet for refunding if necessary: retries, srcPacketSender, srcPacket.DestinationChannel, srcPacket.DestinationPort
    in_flight_packets: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.applications.packet_forward_middleware.v1.GenesisState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GenesisState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class InFlightPacket:
    """InFlightPacket contains information about original packet for writing the acknowledgement and refunding if necessary."""
    original_sender_address: str = ""
    refund_channel_id: str = ""
    refund_port_id: str = ""
    packet_src_channel_id: str = ""
    packet_src_port_id: str = ""
    packet_timeout_timestamp: str = "0"
    packet_timeout_height: str = ""
    packet_data: str = ""
    refund_sequence: str = "0"
    retries_remaining: int = 0
    timeout: str = "0"
    nonrefundable: bool = False
    TYPE_URL: str = field(default="/ibc.applications.packet_forward_middleware.v1.InFlightPacket", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InFlightPacket":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

