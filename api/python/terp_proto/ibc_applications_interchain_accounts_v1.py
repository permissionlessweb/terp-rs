# Auto-generated from ibc.applications.interchain_accounts.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.interchain_accounts.v1.rs
# Package: ibc.applications.interchain_accounts.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.interchain_accounts.v1"

@dataclass
class InterchainAccount:
    """An InterchainAccount is defined as a BaseAccount & the address of the account owner on the controller chain"""
    base_account: Optional[Any  # Any] = None
    account_owner: str = ""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.v1.InterchainAccount", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InterchainAccount":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class InterchainAccountPacketData:
    """InterchainAccountPacketData is comprised of a raw transaction, type of transaction and optional memo field."""
    data: str = ""
    memo: str = ""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.v1.InterchainAccountPacketData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InterchainAccountPacketData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CosmosTx:
    """CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain."""
    messages: List[Any] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.v1.CosmosTx", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CosmosTx":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Metadata:
    """Metadata defines a set of protocol specific data encoded into the ICS27 channel version bytestring See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>"""
    # version defines the ICS27 protocol version
    version: str = ""
    # controller_connection_id is the connection identifier associated with the controller chain
    controller_connection_id: str = ""
    # host_connection_id is the connection identifier associated with the host chain
    host_connection_id: str = ""
    # address defines the interchain account address to be fulfilled upon the OnChanOpenTry handshake step NOTE: the address field is empty on the OnChanOpenInit handshake step
    address: str = ""
    # encoding defines the supported codec format
    encoding: str = ""
    # tx_type defines the type of transactions the interchain account can execute
    tx_type: str = ""
    TYPE_URL: str = field(default="/ibc.applications.interchain_accounts.v1.Metadata", init=False, repr=False)

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

