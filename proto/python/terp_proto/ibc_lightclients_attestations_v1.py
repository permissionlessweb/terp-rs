# Auto-generated from ibc.lightclients.attestations.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.lightclients.attestations.v1.rs
# Package: ibc.lightclients.attestations.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.lightclients.attestations.v1"

@dataclass
class ClientState:
    """ClientState defines an attestor-based light client that tracks the current consensus state and if the client is frozen."""
    # trusted attestor set (EOA addresses)
    attestor_addresses: List[str] = field(default_factory=list)
    # quorum threshold (minimum number of unique attestor signatures required)
    min_required_sigs: int = 0
    # highest height that has been trusted
    latest_height: str = "0"
    # when true, all verification and updates MUST fail
    is_frozen: bool = False
    TYPE_URL: str = field(default="/ibc.lightclients.attestations.v1.ClientState", init=False, repr=False)

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
    """ConsensusState defines an attestor consensus state. The timestamp of a consensus state is stored per height."""
    # trusted UNIX timestamp (nanoseconds) for the height
    timestamp: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.attestations.v1.ConsensusState", init=False, repr=False)

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
class AttestationProof:
    """AttestationProof is used for client updates and membership verification. All attestor signatures cover sha256(attestationData)."""
    # the attestation data that was signed (ABI-encoded StateAttestation or PacketAttestation)
    attestation_data: str = ""
    # array of 65-byte ECDSA signatures (r||s||v)
    signatures: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.lightclients.attestations.v1.AttestationProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "AttestationProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

