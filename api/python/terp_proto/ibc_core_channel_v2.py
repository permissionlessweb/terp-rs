# Auto-generated from ibc.core.channel.v2 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.channel.v2.rs
# Package: ibc.core.channel.v2
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.channel.v2"

@dataclass
class GenesisState:
    """GenesisState defines the ibc channel/v2 submodule's genesis state."""
    acknowledgements: List[PacketState] = field(default_factory=list)
    commitments: List[PacketState] = field(default_factory=list)
    receipts: List[PacketState] = field(default_factory=list)
    async_packets: List[PacketState] = field(default_factory=list)
    send_sequences: List[PacketSequence] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.GenesisState", init=False, repr=False)

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
class PacketState:
    """PacketState defines the generic type necessary to retrieve and store packet commitments, acknowledgements, and receipts. Caller is responsible for knowing the context necessary to interpret this state as a commitment, acknowledgement, or a receipt."""
    # client unique identifier.
    client_id: str = ""
    # packet sequence.
    sequence: str = "0"
    # embedded data that represents packet state.
    data: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.PacketState", init=False, repr=False)

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
class PacketSequence:
    """PacketSequence defines the genesis type necessary to retrieve and store next send sequences."""
    # client unique identifier.
    client_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v2.PacketSequence", init=False, repr=False)

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
class Packet:
    """Packet defines a type that carries data across different chains through IBC"""
    # number corresponds to the order of sends and receives, where a Packet with an earlier sequence number must be sent and received before a Packet with a later sequence number.
    sequence: str = "0"
    # identifies the sending client on the sending chain.
    source_client: str = ""
    # identifies the receiving client on the receiving chain.
    destination_client: str = ""
    # timeout timestamp in seconds after which the packet times out.
    timeout_timestamp: str = "0"
    # a list of payloads, each one for a specific application.
    payloads: List[Payload] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.Packet", init=False, repr=False)

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
class Payload:
    """Payload contains the source and destination ports and payload for the application (version, encoding, raw bytes)"""
    # specifies the source port of the packet.
    source_port: str = ""
    # specifies the destination port of the packet.
    destination_port: str = ""
    # version of the specified application.
    version: str = ""
    # the encoding used for the provided value.
    encoding: str = ""
    # the raw bytes for the payload.
    value: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.Payload", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Payload":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Acknowledgement:
    """Acknowledgement contains a list of all ack results associated with a single packet. In the case of a successful receive, the acknowledgement will contain an app acknowledgement for each application that received a payload in the same order that the payloads were sent in the packet. If the receive is not successful, the acknowledgement will contain a single app acknowledgment which will be a constant error acknowledgment as defined by the IBC v2 protocol."""
    app_acknowledgements: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.Acknowledgement", init=False, repr=False)

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
class RecvPacketResult:
    """RecvPacketResult speecifies the status of a packet as well as the acknowledgement bytes."""
    # status of the packet
    status: int = 0
    # acknowledgement of the packet
    acknowledgement: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.RecvPacketResult", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "RecvPacketResult":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSendPacket:
    """MsgSendPacket sends an outgoing IBC packet."""
    source_client: str = ""
    timeout_timestamp: str = "0"
    payloads: List[Payload] = field(default_factory=list)
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgSendPacket", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSendPacket":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSendPacketResponse:
    """MsgSendPacketResponse defines the Msg/SendPacket response type."""
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgSendPacketResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSendPacketResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRecvPacket:
    """MsgRecvPacket receives an incoming IBC packet."""
    packet: Optional[Packet] = None
    proof_commitment: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgRecvPacket", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgRecvPacketResponse", init=False, repr=False)

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
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgTimeout", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgTimeoutResponse", init=False, repr=False)

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
class MsgAcknowledgement:
    """MsgAcknowledgement receives incoming IBC acknowledgement."""
    packet: Optional[Packet] = None
    acknowledgement: Optional[Acknowledgement] = None
    proof_acked: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgAcknowledgement", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.MsgAcknowledgementResponse", init=False, repr=False)

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
class QueryNextSequenceSendRequest:
    """QueryNextSequenceSendRequest is the request type for the Query/QueryNextSequenceSend RPC method"""
    # client unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryNextSequenceSendRequest", init=False, repr=False)

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
    """QueryNextSequenceSendResponse is the response type for the Query/QueryNextSequenceSend RPC method"""
    # next sequence send number
    next_sequence_send: str = "0"
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryNextSequenceSendResponse", init=False, repr=False)

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

@dataclass
class QueryPacketCommitmentRequest:
    """QueryPacketCommitmentRequest is the request type for the Query/PacketCommitment RPC method."""
    # client unique identifier
    client_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketCommitmentRequest", init=False, repr=False)

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
    """QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method."""
    # packet associated with the request fields
    commitment: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketCommitmentResponse", init=False, repr=False)

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
    """QueryPacketCommitmentsRequest is the request type for the Query/PacketCommitments RPC method."""
    # client unique identifier
    client_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketCommitmentsRequest", init=False, repr=False)

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
    """QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method."""
    # collection of packet commitments for the requested channel identifier.
    commitments: List[PacketState] = field(default_factory=list)
    # pagination response.
    pagination: Optional[Any  # Any] = None
    # query block height.
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketCommitmentsResponse", init=False, repr=False)

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
class QueryPacketAcknowledgementRequest:
    """QueryPacketAcknowledgementRequest is the request type for the Query/PacketAcknowledgement RPC method."""
    # client unique identifier
    client_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketAcknowledgementRequest", init=False, repr=False)

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
    """QueryPacketAcknowledgementResponse is the response type for the Query/PacketAcknowledgement RPC method."""
    # acknowledgement associated with the request fields
    acknowledgement: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketAcknowledgementResponse", init=False, repr=False)

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
    # client unique identifier
    client_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    # list of packet sequences
    packet_commitment_sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketAcknowledgementsRequest", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketAcknowledgementsResponse", init=False, repr=False)

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
class QueryPacketReceiptRequest:
    """QueryPacketReceiptRequest is the request type for the Query/PacketReceipt RPC method."""
    # client unique identifier
    client_id: str = ""
    # packet sequence
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketReceiptRequest", init=False, repr=False)

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
    """QueryPacketReceiptResponse is the response type for the Query/PacketReceipt RPC method."""
    # success flag for if receipt exists
    received: bool = False
    # merkle proof of existence or absence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryPacketReceiptResponse", init=False, repr=False)

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
class QueryUnreceivedPacketsRequest:
    """QueryUnreceivedPacketsRequest is the request type for the Query/UnreceivedPackets RPC method"""
    # client unique identifier
    client_id: str = ""
    # list of packet sequences
    sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryUnreceivedPacketsRequest", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryUnreceivedPacketsResponse", init=False, repr=False)

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
    # client unique identifier
    client_id: str = ""
    # list of acknowledgement sequences
    packet_ack_sequences: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryUnreceivedAcksRequest", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.channel.v2.QueryUnreceivedAcksResponse", init=False, repr=False)

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

