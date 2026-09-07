# Auto-generated from ibc.applications.interchain_accounts.host.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.interchain_accounts.host.v1.rs
# Package: ibc.applications.interchain_accounts.host.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.interchain_accounts.host.v1"

@dataclass
class Params:
    """Params defines the set of on-chain interchain accounts parameters. The following parameters may be used to disable the host submodule."""
    # host_enabled enables or disables the host submodule.
    host_enabled: bool = False
    # allow_messages defines a list of sdk message typeURLs allowed to be executed on a host chain.
    allow_messages: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.Params", init=False, repr=False)

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
class QueryRequest:
    """QueryRequest defines the parameters for a particular query request by an interchain account."""
    # path defines the path of the query request as defined by ADR-021. <https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-021-protobuf-query-encoding.md#custom-query-registration-and-routing>
    path: str = ""
    # data defines the payload of the query request as defined by ADR-021. <https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-021-protobuf-query-encoding.md#custom-query-registration-and-routing>
    data: str = ""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.QueryRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParams:
    """MsgUpdateParams defines the payload for Msg/UpdateParams"""
    # signer address
    signer: str = ""
    # params defines the 27-interchain-accounts/host parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.MsgUpdateParams", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateParams":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParamsResponse:
    """MsgUpdateParamsResponse defines the response for Msg/UpdateParams"""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.MsgUpdateParamsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateParamsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgModuleQuerySafe:
    """MsgModuleQuerySafe defines the payload for Msg/ModuleQuerySafe"""
    # signer address
    signer: str = ""
    # requests defines the module safe queries to execute.
    requests: List[QueryRequest] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.MsgModuleQuerySafe", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgModuleQuerySafe":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgModuleQuerySafeResponse:
    """MsgModuleQuerySafeResponse defines the response for Msg/ModuleQuerySafe"""
    # height at which the responses were queried
    height: str = "0"
    # protobuf encoded responses for each query
    responses: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.MsgModuleQuerySafeResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgModuleQuerySafeResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryParamsRequest:
    """QueryParamsRequest is the request type for the Query/Params RPC method."""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.QueryParamsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryParamsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryParamsResponse:
    """QueryParamsResponse is the response type for the Query/Params RPC method."""
    # params defines the parameters of the module.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.host.v1.QueryParamsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryParamsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

