# Auto-generated from ibc.core.client.v2 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.client.v2.rs
# Package: ibc.core.client.v2
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.client.v2"

@dataclass
class CounterpartyInfo:
    """CounterpartyInfo defines the key that the counterparty will use to message our client"""
    # merkle prefix key is the prefix that ics provable keys are stored under
    merkle_prefix: List[str] = field(default_factory=list)
    # client identifier is the identifier used to send packet messages to our client
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v2.CounterpartyInfo", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CounterpartyInfo":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisCounterpartyInfo:
    """GenesisCounterpartyInfo defines the state associating a client with a counterparty."""
    # ClientId is the ID of the given client.
    client_id: str = ""
    # CounterpartyInfo is the counterparty info of the given client.
    counterparty_info: Optional[CounterpartyInfo] = None
    TYPE_URL: str = field(default="/ibc.core.client.v2.GenesisCounterpartyInfo", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GenesisCounterpartyInfo":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the ibc client v2 submodule's genesis state."""
    # counterparty info for each client
    counterparty_infos: List[GenesisCounterpartyInfo] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.client.v2.GenesisState", init=False, repr=False)

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
class Config:
    """Config is a **per-client** configuration struct that sets which relayers are allowed to relay v2 IBC messages for a given client. If it is set, then only relayers in the allow list can send v2 messages If it is not set, then the client allows permissionless relaying of v2 messages"""
    # allowed_relayers defines the set of allowed relayers for IBC V2 protocol for the given client
    allowed_relayers: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.client.v2.Config", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Config":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRegisterCounterparty:
    """MsgRegisterCounterparty defines a message to register a counterparty on a client"""
    # client identifier
    client_id: str = ""
    # counterparty merkle prefix
    counterparty_merkle_prefix: List[str] = field(default_factory=list)
    # counterparty client identifier
    counterparty_client_id: str = ""
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v2.MsgRegisterCounterparty", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRegisterCounterparty":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRegisterCounterpartyResponse:
    """MsgRegisterCounterpartyResponse defines the Msg/RegisterCounterparty response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v2.MsgRegisterCounterpartyResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRegisterCounterpartyResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateClientConfig:
    """MsgUpdateClientConfig defines the sdk.Msg type to update the configuration for a given client"""
    # client identifier
    client_id: str = ""
    # allowed relayers  NOTE: All fields in the config must be supplied.
    config: Optional[Config] = None
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v2.MsgUpdateClientConfig", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateClientConfig":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateClientConfigResponse:
    """MsgUpdateClientConfigResponse defines the MsgUpdateClientConfig response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v2.MsgUpdateClientConfigResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateClientConfigResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryCounterpartyInfoRequest:
    """QueryCounterpartyInfoRequest is the request type for the Query/CounterpartyInfo RPC method"""
    # client state unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v2.QueryCounterpartyInfoRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryCounterpartyInfoRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryCounterpartyInfoResponse:
    """QueryCounterpartyInfoResponse is the response type for the Query/CounterpartyInfo RPC method."""
    counterparty_info: Optional[CounterpartyInfo] = None
    TYPE_URL: str = field(default="/ibc.core.client.v2.QueryCounterpartyInfoResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryCounterpartyInfoResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConfigRequest:
    """QueryConfigRequest is the request type for the Query/Config RPC method"""
    # client state unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v2.QueryConfigRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConfigRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConfigResponse:
    """QueryConfigResponse is the response type for the Query/Config RPC method"""
    config: Optional[Config] = None
    TYPE_URL: str = field(default="/ibc.core.client.v2.QueryConfigResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConfigResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

