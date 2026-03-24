# Auto-generated from cosmos.auth.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/cosmos.auth.v1beta1.rs
# Package: cosmos.auth.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos.auth.v1beta1"

@dataclass
class BaseAccount:
    """BaseAccount defines a base account type. It contains all the necessary fields for basic account functionality. Any custom account type should extend this type for additional functionality (e.g. vesting)."""
    address: str = ""
    pub_key: Optional[Any  # Any] = None
    account_number: str = "0"
    sequence: str = "0"
    TYPE_URL: str = field(default="/cosmos.auth.v1beta1.BaseAccount", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "BaseAccount":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ModuleAccount:
    """ModuleAccount defines an account for modules that holds coins on a pool."""
    base_account: Optional[BaseAccount] = None
    name: str = ""
    permissions: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.auth.v1beta1.ModuleAccount", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ModuleAccount":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ModuleCredential:
    """ModuleCredential represents a unclaimable pubkey for base accounts controlled by modules."""
    # module_name is the name of the module used for address derivation (passed into address.Module).
    module_name: str = ""
    # derivation_keys is for deriving a module account address (passed into address.Module) adding more keys creates sub-account addresses (passed into address.Derive)
    derivation_keys: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.auth.v1beta1.ModuleCredential", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ModuleCredential":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Params:
    """Params defines the parameters for the auth module."""
    max_memo_characters: str = "0"
    tx_sig_limit: str = "0"
    tx_size_cost_per_byte: str = "0"
    sig_verify_cost_ed25519: str = "0"
    sig_verify_cost_secp256k1: str = "0"
    TYPE_URL: str = field(default="/cosmos.auth.v1beta1.Params", init=False, repr=False)

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

