# Auto-generated from ibc.applications.gmp.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.gmp.v1.rs
# Package: ibc.applications.gmp.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.gmp.v1"

@dataclass
class MsgSendCall:
    """MsgSendCall defines a msg to send a call to a contract/receiver on a ICS27-2 enabled chain."""
    # the client by which the packet will be sent
    source_client: str = ""
    # the sender address
    sender: str = ""
    # the recipient address on the destination chain
    receiver: str = ""
    # The salt used to generate the caller account address
    salt: str = ""
    # The payload of the call
    payload: str = ""
    # Timeout timestamp in absolute nanoseconds since unix epoch.
    timeout_timestamp: str = "0"
    # optional memo
    memo: str = ""
    # optional encoding
    encoding: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.MsgSendCall", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSendCall":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSendCallResponse:
    """MsgSendCallResponse defines the Msg/SendCall response type."""
    # sequence number of the GMP packet sent
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.MsgSendCallResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSendCallResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class AccountIdentifier:
    """AccountIdentifier is used to identify a ICS27 account."""
    # The (local) client identifier
    client_id: str = ""
    # The sender of the packet
    sender: str = ""
    # The salt of the packet
    salt: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.AccountIdentifier", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "AccountIdentifier":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Ics27Account:
    """An ICS27Account is defined as a BaseAccount & the account identifier"""
    address: str = ""
    account_id: Optional[AccountIdentifier] = None
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.ICS27Account", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Ics27Account":
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
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.CosmosTx", init=False, repr=False)

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
class GmpPacketData:
    """GMPPacketData defines a struct for the packet payload"""
    # the sender address
    sender: str = ""
    # the recipient address on the destination chain
    receiver: str = ""
    # The salt used to generate the caller account address
    salt: str = ""
    # The payload of the call
    payload: str = ""
    # optional memo
    memo: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.GMPPacketData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GmpPacketData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Acknowledgement:
    """Acknowledgement defines a struct for the ICS27-2 acknowledgement"""
    # The result of the call
    result: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.Acknowledgement", init=False, repr=False)

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
class QueryAccountAddressRequest:
    """QueryAccountAddressRequest is the request type for the Query/AccountAddress RPC method."""
    # The (local) client identifier
    client_id: str = ""
    # The sender of the packet
    sender: str = ""
    # The salt of the packet (in hex format)
    salt: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.QueryAccountAddressRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAccountAddressRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAccountAddressResponse:
    """QueryAccountAddressResponse is the response type for the Query/AccountAddress RPC method."""
    # The interchain account address
    account_address: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.QueryAccountAddressResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAccountAddressResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAccountIdentifierRequest:
    """QueryAccountIdentifierRequest is the request type for querying the account identifier by account address."""
    account_address: str = ""
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.QueryAccountIdentifierRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAccountIdentifierRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAccountIdentifierResponse:
    """QueryAccountIdentifierResponse is the response type for querying the account identifier by account address."""
    account_id: Optional[AccountIdentifier] = None
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.QueryAccountIdentifierResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAccountIdentifierResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the 27-gmp genesis state"""
    # The list of registered ICS27 accounts
    ics27_accounts: List[RegisteredIcs27Account] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.GenesisState", init=False, repr=False)

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
class RegisteredIcs27Account:
    """RegisteredICS27Account contains an account identifier and associated interchain account address"""
    # / The address of the ics27 account
    account_address: str = ""
    # / The account identifier
    account_id: Optional[AccountIdentifier] = None
    TYPE_URL: str = field(default="/ibc.applications.gmp.v1.RegisteredICS27Account", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "RegisteredIcs27Account":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

