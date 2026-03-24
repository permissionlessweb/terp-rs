# Auto-generated from ibc.lightclients.solomachine.v2 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.lightclients.solomachine.v2.rs
# Package: ibc.lightclients.solomachine.v2
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.lightclients.solomachine.v2"

@dataclass
class ClientState:
    """ClientState defines a solo machine client that tracks the current consensus state and if the client is frozen."""
    # latest sequence of the client state
    sequence: str = "0"
    # frozen sequence of the solo machine
    is_frozen: bool = False
    consensus_state: Optional[ConsensusState] = None
    # when set to true, will allow governance to update a solo machine client. The client will be unfrozen if it is frozen.
    allow_update_after_proposal: bool = False
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ClientState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ClientState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConsensusState:
    """ConsensusState defines a solo machine consensus state. The sequence of a consensus state is contained in the "height" key used in storing the consensus state."""
    # public key of the solo machine
    public_key: Optional[Any  # Any] = None
    # diversifier allows the same public key to be reused across different solo machine clients (potentially on different chains) without being considered misbehaviour.
    diversifier: str = ""
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ConsensusState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConsensusState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Header:
    """Header defines a solo machine consensus header"""
    # sequence to update solo machine public key at
    sequence: str = "0"
    timestamp: str = "0"
    signature: str = ""
    new_public_key: Optional[Any  # Any] = None
    new_diversifier: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.Header", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Header":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Misbehaviour:
    """Misbehaviour defines misbehaviour for a solo machine which consists of a sequence and two signatures over different messages at that sequence."""
    client_id: str = ""
    sequence: str = "0"
    signature_one: Optional[SignatureAndData] = None
    signature_two: Optional[SignatureAndData] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.Misbehaviour", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Misbehaviour":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class SignatureAndData:
    """SignatureAndData contains a signature and the data signed over to create that signature."""
    signature: str = ""
    data_type: int = 0
    data: str = ""
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.SignatureAndData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "SignatureAndData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class TimestampedSignatureData:
    """TimestampedSignatureData contains the signature data and the timestamp of the signature."""
    signature_data: str = ""
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.TimestampedSignatureData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "TimestampedSignatureData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class SignBytes:
    """SignBytes defines the signed bytes used for signature verification."""
    sequence: str = "0"
    timestamp: str = "0"
    diversifier: str = ""
    # type of the data used
    data_type: int = 0
    # marshaled data
    data: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.SignBytes", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "SignBytes":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class HeaderData:
    """HeaderData returns the SignBytes data for update verification."""
    # header public key
    new_pub_key: Optional[Any  # Any] = None
    # header diversifier
    new_diversifier: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.HeaderData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "HeaderData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ClientStateData:
    """ClientStateData returns the SignBytes data for client state verification."""
    path: str = ""
    client_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ClientStateData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ClientStateData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConsensusStateData:
    """ConsensusStateData returns the SignBytes data for consensus state verification."""
    path: str = ""
    consensus_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ConsensusStateData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConsensusStateData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConnectionStateData:
    """ConnectionStateData returns the SignBytes data for connection state verification."""
    path: str = ""
    connection: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ConnectionStateData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConnectionStateData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ChannelStateData:
    """ChannelStateData returns the SignBytes data for channel state verification."""
    path: str = ""
    channel: Optional[Any  # Channel] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.ChannelStateData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ChannelStateData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PacketCommitmentData:
    """PacketCommitmentData returns the SignBytes data for packet commitment verification."""
    path: str = ""
    commitment: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.PacketCommitmentData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketCommitmentData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PacketAcknowledgementData:
    """PacketAcknowledgementData returns the SignBytes data for acknowledgement verification."""
    path: str = ""
    acknowledgement: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.PacketAcknowledgementData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketAcknowledgementData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PacketReceiptAbsenceData:
    """PacketReceiptAbsenceData returns the SignBytes data for packet receipt absence verification."""
    path: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.PacketReceiptAbsenceData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PacketReceiptAbsenceData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class NextSequenceRecvData:
    """NextSequenceRecvData returns the SignBytes data for verification of the next sequence to be received."""
    path: str = ""
    next_seq_recv: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v2.NextSequenceRecvData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "NextSequenceRecvData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

