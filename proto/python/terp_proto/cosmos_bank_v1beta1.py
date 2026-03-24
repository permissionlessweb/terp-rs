# Auto-generated from cosmos.bank.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/cosmos.bank.v1beta1.rs
# Package: cosmos.bank.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos.bank.v1beta1"

@dataclass
class Params:
    """Params defines the parameters for the bank module."""
    # Deprecated: Use of SendEnabled in params is deprecated. For genesis, use the newly added send_enabled field in the genesis object. Storage, lookup, and manipulation of this information is now in the keeper.  As of cosmos-sdk 0.47, this only exists for backwards compatibility of genesis files.
    send_enabled: List[SendEnabled] = field(default_factory=list)
    default_send_enabled: bool = False
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.Params", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Params":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class SendEnabled:
    """SendEnabled maps coin denom to a send_enabled status (whether a denom is sendable)."""
    denom: str = ""
    enabled: bool = False
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.SendEnabled", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "SendEnabled":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Input:
    """Input models transaction input."""
    address: str = ""
    coins: List[Coin] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.Input", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Input":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Output:
    """Output models transaction outputs."""
    address: str = ""
    coins: List[Coin] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.Output", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Output":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Supply:
    """Supply represents a struct that passively keeps track of the total supply amounts in the network. This message is deprecated now that supply is indexed by denom."""
    total: List[Coin] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.Supply", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Supply":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class DenomUnit:
    """DenomUnit represents a struct that describes a given denomination unit of the basic token."""
    # denom represents the string name of the given denom unit (e.g uatom).
    denom: str = ""
    # exponent represents power of 10 exponent that one must raise the base_denom to in order to equal the given DenomUnit's denom 1 denom = 10^exponent base_denom (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with exponent = 6, thus: 1 atom = 10^6 uatom).
    exponent: int = 0
    # aliases is a list of string aliases for the given denom
    aliases: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.DenomUnit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "DenomUnit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Metadata:
    """Metadata represents a struct that describes a basic token."""
    description: str = ""
    # denom_units represents the list of DenomUnit's for a given coin
    denom_units: List[DenomUnit] = field(default_factory=list)
    # base represents the base denom (should be the DenomUnit with exponent = 0).
    base: str = ""
    # display indicates the suggested denom that should be displayed in clients.
    display: str = ""
    # name defines the name of the token (eg: Cosmos Atom)
    name: str = ""
    # symbol is the token symbol usually shown on exchanges (eg: ATOM). This can be the same as the display.
    symbol: str = ""
    # URI to a document (on or off-chain) that contains additional information. Optional.
    uri: str = ""
    # URIHash is a sha256 hash of a document pointed by URI. It's used to verify that the document didn't change. Optional.
    uri_hash: str = ""
    TYPE_URL: str = field(default="/cosmos.bank.v1beta1.Metadata", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Metadata":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

