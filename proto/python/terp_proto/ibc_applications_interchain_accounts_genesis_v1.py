# Auto-generated from ibc.applications.interchain_accounts.genesis.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.interchain_accounts.genesis.v1.rs
# Package: ibc.applications.interchain_accounts.genesis.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.interchain_accounts.genesis.v1"

@dataclass
class GenesisState:
    """GenesisState defines the interchain accounts genesis state"""
    controller_genesis_state: Optional[ControllerGenesisState] = None
    host_genesis_state: Optional[HostGenesisState] = None
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.genesis.v1.GenesisState", init=False, repr=False)

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
class ControllerGenesisState:
    """ControllerGenesisState defines the interchain accounts controller genesis state"""
    active_channels: List[ActiveChannel] = field(default_factory=list)
    interchain_accounts: List[RegisteredInterchainAccount] = field(default_factory=list)
    ports: List[str] = field(default_factory=list)
    params: Optional[Any  # Params] = None
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.genesis.v1.ControllerGenesisState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ControllerGenesisState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class HostGenesisState:
    """HostGenesisState defines the interchain accounts host genesis state"""
    active_channels: List[ActiveChannel] = field(default_factory=list)
    interchain_accounts: List[RegisteredInterchainAccount] = field(default_factory=list)
    port: str = ""
    params: Optional[Any  # Params] = None
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.genesis.v1.HostGenesisState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "HostGenesisState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ActiveChannel:
    """ActiveChannel contains a connection ID, port ID and associated active channel ID, as well as a boolean flag to indicate if the channel is middleware enabled"""
    connection_id: str = ""
    port_id: str = ""
    channel_id: str = ""
    is_middleware_enabled: bool = False
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.genesis.v1.ActiveChannel", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ActiveChannel":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class RegisteredInterchainAccount:
    """RegisteredInterchainAccount contains a connection ID, port ID and associated interchain account address"""
    connection_id: str = ""
    port_id: str = ""
    account_address: str = ""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.genesis.v1.RegisteredInterchainAccount", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "RegisteredInterchainAccount":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

