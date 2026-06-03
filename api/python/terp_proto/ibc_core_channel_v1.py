# Auto-generated from ibc.core.channel.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.channel.v1.rs
# Package: ibc.core.channel.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.channel.v1"

@dataclass
class Channel:
    """Channel defines pipeline for exactly-once packet delivery between specific modules on separate blockchains, which has at least one end capable of sending packets and one end capable of receiving packets."""
    # current state of the channel end
    state: int = 0
    # whether the channel is ordered or unordered
    ordering: int = 0
    # counterparty channel end
    counterparty: Optional[Counterparty] = None
    # list of connection identifiers, in order, along which packets sent on this channel will travel
    connection_hops: List[str] = field(default_factory=list)
    # opaque channel version, which is agreed upon during the handshake
    version: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.Channel", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Channel":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class IdentifiedChannel:
    """IdentifiedChannel defines a channel with additional port and channel identifier fields."""
    # current state of the channel end
    state: int = 0
    # whether the channel is ordered or unordered
    ordering: int = 0
    # counterparty channel end
    counterparty: Optional[Counterparty] = None
    # list of connection identifiers, in order, along which packets sent on this channel will travel
    connection_hops: List[str] = field(default_factory=list)
    # opaque channel version, which is agreed upon during the handshake
    version: str = ""
    # port identifier
    port_id: str = ""
    # channel identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.IdentifiedChannel", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "IdentifiedChannel":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Counterparty:
    """Counterparty defines a channel end counterparty"""
    # port on the counterparty chain which owns the other end of the channel.
    port_id: str = ""
    # channel end on the counterparty chain
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.Counterparty", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Counterparty":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Packet:
    """Packet defines a type that carries data across different chains through IBC"""
    # number corresponds to the order of sends and receives, where a Packet with an earlier sequence number must be sent and received before a Packet with a later sequence number.
    sequence: str = "0"
    # identifies the port on the sending chain.
    source_port: str = ""
    # identifies the channel end on the sending chain.
    source_channel: str = ""
    # identifies the port on the receiving chain.
    destination_port: str = ""
    # identifies the channel end on the receiving chain.
    destination_channel: str = ""
    # actual opaque bytes transferred directly to the application module
    data: str = ""
    # block height after which the packet times out
    timeout_height: Optional[Any  # Height] = None
    # block timestamp (in nanoseconds) after which the packet times out
    timeout_timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.Packet", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Packet":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PacketState:
    """PacketState defines the generic type necessary to retrieve and store packet commitments, acknowledgements, and receipts. Caller is responsible for knowing the context necessary to interpret this state as a commitment, acknowledgement, or a receipt."""
    # channel port identifier.
    port_id: str = ""
    # channel unique identifier.
    channel_id: str = ""
    # packet sequence.
    sequence: str = "0"
    # embedded data that represents packet state.
    data: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.PacketState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PacketId:
    """PacketId is an identifier for a unique Packet Source chains refer to packets by source port/channel Destination chains refer to packets by destination port/channel"""
    # channel port identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.PacketId", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketId":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Acknowledgement:
    """Acknowledgement is the recommended acknowledgement format to be used by app-specific protocols. NOTE: The field numbers 21 and 22 were explicitly chosen to avoid accidental conflicts with other protobuf message formats used for acknowledgements. The first byte of any message with this format will be the non-ASCII values `0xaa` (result) or `0xb2` (error). Implemented as defined by ICS: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope>"""
    # response contains either a result or an error and must be non-empty
    response: Any = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.Acknowledgement", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Acknowledgement":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Timeout:
    """Timeout defines an execution deadline structure for 04-channel handlers. This includes packet lifecycle handlers. A valid Timeout contains either one or both of a timestamp and block height (sequence)."""
    # block height after which the packet times out
    height: Optional[Any  # Height] = None
    # block timestamp (in nanoseconds) after which the packet times out
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.Timeout", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Timeout":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the ibc channel submodule's genesis state."""
    channels: List[IdentifiedChannel] = field(default_factory=list)
    acknowledgements: List[PacketState] = field(default_factory=list)
    commitments: List[PacketState] = field(default_factory=list)
    receipts: List[PacketState] = field(default_factory=list)
    send_sequences: List[PacketSequence] = field(default_factory=list)
    recv_sequences: List[PacketSequence] = field(default_factory=list)
    ack_sequences: List[PacketSequence] = field(default_factory=list)
    # the sequence for the next generated channel identifier
    next_channel_sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.GenesisState", init=False, repr=False)

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
class PacketSequence:
    """PacketSequence defines the genesis type necessary to retrieve and store next send and receive sequences."""
    port_id: str = ""
    channel_id: str = ""
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.PacketSequence", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketSequence":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenInit:
    """MsgChannelOpenInit defines an sdk.Msg to initialize a channel handshake. It is called by a relayer on Chain A."""
    port_id: str = ""
    channel: Optional[Channel] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenInit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenInit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenInitResponse:
    """MsgChannelOpenInitResponse defines the Msg/ChannelOpenInit response type."""
    channel_id: str = ""
    version: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenInitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenInitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenTry:
    """MsgChannelOpenInit defines a msg sent by a Relayer to try to open a channel on Chain B. The version field within the Channel field has been deprecated. Its value will be ignored by core IBC."""
    port_id: str = ""
    # Deprecated: this field is unused. Crossing hello's are no longer supported in core IBC.
    previous_channel_id: str = ""
    # NOTE: the version field within the channel has been deprecated. Its value will be ignored by core IBC.
    channel: Optional[Channel] = None
    counterparty_version: str = ""
    proof_init: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenTry", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenTry":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenTryResponse:
    """MsgChannelOpenTryResponse defines the Msg/ChannelOpenTry response type."""
    version: str = ""
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenTryResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenTryResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenAck:
    """MsgChannelOpenAck defines a msg sent by a Relayer to Chain A to acknowledge the change of channel state to TRYOPEN on Chain B."""
    port_id: str = ""
    channel_id: str = ""
    counterparty_channel_id: str = ""
    counterparty_version: str = ""
    proof_try: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenAck", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenAck":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenAckResponse:
    """MsgChannelOpenAckResponse defines the Msg/ChannelOpenAck response type."""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenAckResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenAckResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenConfirm:
    """MsgChannelOpenConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of channel state to OPEN on Chain A."""
    port_id: str = ""
    channel_id: str = ""
    proof_ack: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenConfirm", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenConfirm":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelOpenConfirmResponse:
    """MsgChannelOpenConfirmResponse defines the Msg/ChannelOpenConfirm response type."""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelOpenConfirmResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelOpenConfirmResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelCloseInit:
    """MsgChannelCloseInit defines a msg sent by a Relayer to Chain A to close a channel with Chain B."""
    port_id: str = ""
    channel_id: str = ""
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelCloseInit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelCloseInit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelCloseInitResponse:
    """MsgChannelCloseInitResponse defines the Msg/ChannelCloseInit response type."""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelCloseInitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelCloseInitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelCloseConfirm:
    """MsgChannelCloseConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of channel state to CLOSED on Chain A."""
    port_id: str = ""
    channel_id: str = ""
    proof_init: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelCloseConfirm", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelCloseConfirm":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChannelCloseConfirmResponse:
    """MsgChannelCloseConfirmResponse defines the Msg/ChannelCloseConfirm response type."""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgChannelCloseConfirmResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChannelCloseConfirmResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRecvPacket:
    """MsgRecvPacket receives incoming IBC packet"""
    packet: Optional[Packet] = None
    proof_commitment: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgRecvPacket", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRecvPacket":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRecvPacketResponse:
    """MsgRecvPacketResponse defines the Msg/RecvPacket response type."""
    result: int = 0
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgRecvPacketResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRecvPacketResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgTimeout:
    """MsgTimeout receives timed-out packet"""
    packet: Optional[Packet] = None
    proof_unreceived: str = ""
    proof_height: Optional[Any  # Height] = None
    next_sequence_recv: str = "0"
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgTimeout", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTimeout":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgTimeoutResponse:
    """MsgTimeoutResponse defines the Msg/Timeout response type."""
    result: int = 0
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgTimeoutResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTimeoutResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgTimeoutOnClose:
    """MsgTimeoutOnClose timed-out packet upon counterparty channel closure."""
    packet: Optional[Packet] = None
    proof_unreceived: str = ""
    proof_close: str = ""
    proof_height: Optional[Any  # Height] = None
    next_sequence_recv: str = "0"
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgTimeoutOnClose", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTimeoutOnClose":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgTimeoutOnCloseResponse:
    """MsgTimeoutOnCloseResponse defines the Msg/TimeoutOnClose response type."""
    result: int = 0
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgTimeoutOnCloseResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTimeoutOnCloseResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgAcknowledgement:
    """MsgAcknowledgement receives incoming IBC acknowledgement"""
    packet: Optional[Packet] = None
    acknowledgement: str = ""
    proof_acked: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgAcknowledgement", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAcknowledgement":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgAcknowledgementResponse:
    """MsgAcknowledgementResponse defines the Msg/Acknowledgement response type."""
    result: int = 0
    TYPE_URL: str = field(default="/ibc.core.channel.v1.MsgAcknowledgementResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAcknowledgementResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelRequest:
    """QueryChannelRequest is the request type for the Query/Channel RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelResponse:
    """QueryChannelResponse is the response type for the Query/Channel RPC method. Besides the Channel end, it includes a proof and the height from which the proof was retrieved."""
    # channel associated with the request identifiers
    channel: Optional[Channel] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelsRequest:
    """QueryChannelsRequest is the request type for the Query/Channels RPC method"""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelsResponse:
    """QueryChannelsResponse is the response type for the Query/Channels RPC method."""
    # list of stored channels of the chain.
    channels: List[IdentifiedChannel] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionChannelsRequest:
    """QueryConnectionChannelsRequest is the request type for the Query/QueryConnectionChannels RPC method"""
    # connection unique identifier
    connection: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryConnectionChannelsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionChannelsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionChannelsResponse:
    """QueryConnectionChannelsResponse is the Response type for the Query/QueryConnectionChannels RPC method"""
    # list of channels associated with a connection.
    channels: List[IdentifiedChannel] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryConnectionChannelsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionChannelsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelClientStateRequest:
    """QueryChannelClientStateRequest is the request type for the Query/ClientState RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelClientStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelClientStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelClientStateResponse:
    """QueryChannelClientStateResponse is the Response type for the Query/QueryChannelClientState RPC method"""
    # client state associated with the channel
    identified_client_state: Optional[Any  # Any] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelClientStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelClientStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelConsensusStateRequest:
    """QueryChannelConsensusStateRequest is the request type for the Query/ConsensusState RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # revision number of the consensus state
    revision_number: str = "0"
    # revision height of the consensus state
    revision_height: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelConsensusStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelConsensusStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryChannelConsensusStateResponse:
    """QueryChannelClientStateResponse is the Response type for the Query/QueryChannelClientState RPC method"""
    # consensus state associated with the channel
    consensus_state: Optional[Any  # Any] = None
    # client ID associated with the consensus state
    client_id: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryChannelConsensusStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryChannelConsensusStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketCommitmentRequest:
    """QueryPacketCommitmentRequest is the request type for the Query/PacketCommitment RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketCommitmentRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketCommitmentRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketCommitmentResponse:
    """QueryPacketCommitmentResponse defines the client query response for a packet which also includes a proof and the height from which the proof was retrieved"""
    # packet associated with the request fields
    commitment: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketCommitmentResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketCommitmentResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketCommitmentsRequest:
    """QueryPacketCommitmentsRequest is the request type for the Query/QueryPacketCommitments RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketCommitmentsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketCommitmentsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketCommitmentsResponse:
    """QueryPacketCommitmentsResponse is the request type for the Query/QueryPacketCommitments RPC method"""
    commitments: List[PacketState] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketCommitmentsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketCommitmentsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketReceiptRequest:
    """QueryPacketReceiptRequest is the request type for the Query/PacketReceipt RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketReceiptRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketReceiptRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketReceiptResponse:
    """QueryPacketReceiptResponse defines the client query response for a packet receipt which also includes a proof, and the height from which the proof was retrieved"""
    # success flag for if receipt exists
    received: bool = False
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketReceiptResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketReceiptResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketAcknowledgementRequest:
    """QueryPacketAcknowledgementRequest is the request type for the Query/PacketAcknowledgement RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketAcknowledgementRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketAcknowledgementRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketAcknowledgementResponse:
    """QueryPacketAcknowledgementResponse defines the client query response for a packet which also includes a proof and the height from which the proof was retrieved"""
    # packet associated with the request fields
    acknowledgement: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketAcknowledgementResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketAcknowledgementResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketAcknowledgementsRequest:
    """QueryPacketAcknowledgementsRequest is the request type for the Query/QueryPacketCommitments RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    # list of packet sequences
    packet_commitment_sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketAcknowledgementsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketAcknowledgementsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryPacketAcknowledgementsResponse:
    """QueryPacketAcknowledgemetsResponse is the request type for the Query/QueryPacketAcknowledgements RPC method"""
    acknowledgements: List[PacketState] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryPacketAcknowledgementsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryPacketAcknowledgementsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUnreceivedPacketsRequest:
    """QueryUnreceivedPacketsRequest is the request type for the Query/UnreceivedPackets RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # list of packet sequences
    packet_commitment_sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryUnreceivedPacketsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUnreceivedPacketsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUnreceivedPacketsResponse:
    """QueryUnreceivedPacketsResponse is the response type for the Query/UnreceivedPacketCommitments RPC method"""
    # list of unreceived packet sequences
    sequences: List[str] = field(default_factory=list)
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryUnreceivedPacketsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUnreceivedPacketsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUnreceivedAcksRequest:
    """QueryUnreceivedAcks is the request type for the Query/UnreceivedAcks RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    # list of acknowledgement sequences
    packet_ack_sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryUnreceivedAcksRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUnreceivedAcksRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUnreceivedAcksResponse:
    """QueryUnreceivedAcksResponse is the response type for the Query/UnreceivedAcks RPC method"""
    # list of unreceived acknowledgement sequences
    sequences: List[str] = field(default_factory=list)
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryUnreceivedAcksResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUnreceivedAcksResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryNextSequenceReceiveRequest:
    """QueryNextSequenceReceiveRequest is the request type for the Query/QueryNextSequenceReceiveRequest RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryNextSequenceReceiveRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryNextSequenceReceiveRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryNextSequenceReceiveResponse:
    """QuerySequenceResponse is the response type for the Query/QueryNextSequenceReceiveResponse RPC method"""
    # next sequence receive number
    next_sequence_receive: str = "0"
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryNextSequenceReceiveResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryNextSequenceReceiveResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryNextSequenceSendRequest:
    """QueryNextSequenceSendRequest is the request type for the Query/QueryNextSequenceSend RPC method"""
    # port unique identifier
    port_id: str = ""
    # channel unique identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryNextSequenceSendRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryNextSequenceSendRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryNextSequenceSendResponse:
    """QueryNextSequenceSendResponse is the request type for the Query/QueryNextSequenceSend RPC method"""
    # next sequence send number
    next_sequence_send: str = "0"
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v1.QueryNextSequenceSendResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryNextSequenceSendResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

