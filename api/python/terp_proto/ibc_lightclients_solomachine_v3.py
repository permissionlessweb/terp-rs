# Auto-generated from ibc.lightclients.solomachine.v3 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.lightclients.solomachine.v3.rs
# Package: ibc.lightclients.solomachine.v3
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.lightclients.solomachine.v3"

@dataclass
class ClientState:
    """ClientState defines a solo machine client that tracks the current consensus state and if the client is frozen."""
    # latest sequence of the client state
    sequence: str = "0"
    # frozen sequence of the solo machine
    is_frozen: bool = False
    consensus_state: Optional[ConsensusState] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.ClientState", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.ConsensusState", init=False, repr=False)

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
    timestamp: str = "0"
    signature: str = ""
    new_public_key: Optional[Any  # Any] = None
    new_diversifier: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.Header", init=False, repr=False)

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
    sequence: str = "0"
    signature_one: Optional[SignatureAndData] = None
    signature_two: Optional[SignatureAndData] = None
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.Misbehaviour", init=False, repr=False)

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
    path: str = ""
    data: str = ""
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.SignatureAndData", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.TimestampedSignatureData", init=False, repr=False)

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
    # the sequence number
    sequence: str = "0"
    # the proof timestamp
    timestamp: str = "0"
    # the public key diversifier
    diversifier: str = ""
    # the standardised path bytes
    path: str = ""
    # the marshaled data bytes
    data: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.SignBytes", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.lightclients.solomachine.v3.HeaderData", init=False, repr=False)

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

