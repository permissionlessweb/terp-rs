# Auto-generated from terp.smartaccount.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/terp.smartaccount.v1beta1.rs
# Package: terp.smartaccount.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "terp.smartaccount.v1beta1"

@dataclass
class MsgAddAuthenticator:
    """MsgAddAuthenticatorRequest defines the Msg/AddAuthenticator request type."""
    sender: str = ""
    authenticator_type: str = ""
    data: str = ""
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgAddAuthenticator", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAddAuthenticator":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgAddAuthenticatorResponse:
    """MsgAddAuthenticatorResponse defines the Msg/AddAuthenticator response type."""
    success: bool = False
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgAddAuthenticatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAddAuthenticatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRemoveAuthenticator:
    """MsgRemoveAuthenticatorRequest defines the Msg/RemoveAuthenticator request type."""
    sender: str = ""
    id: str = "0"
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgRemoveAuthenticator", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRemoveAuthenticator":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRemoveAuthenticatorResponse:
    """MsgRemoveAuthenticatorResponse defines the Msg/RemoveAuthenticator response type."""
    success: bool = False
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgRemoveAuthenticatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRemoveAuthenticatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSetActiveState:
    sender: str = ""
    active: bool = False
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgSetActiveState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSetActiveState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSetActiveStateResponse:
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.MsgSetActiveStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSetActiveStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class AgAuthData:
    """AgAuthData is a Serialized array of signing.SignatureV2. We Marshal & Unmarshal via `UnmarshalSignatureJSON` & `MarshalSignatureJSON`"""
    data: str = ""
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.AgAuthData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "AgAuthData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class TxExtension:
    """TxExtension allows for additional authenticator-specific data in transactions."""
    # selected_authenticators holds the authenticator_id for the chosen authenticator per message.
    selected_authenticators: List[str] = field(default_factory=list)
    # optional, used to provide aggregate key signature data to module for authentication.
    agg_auth: Optional[AgAuthData] = None
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.TxExtension", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "TxExtension":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class BlsConfig:
    """BlsConfig"""
    # list of pubkeys that are points in aggregate key set
    pubkeys: List[str] = field(default_factory=list)
    # minimum threshold of points in order for tx to be valid
    threshold: str = "0"
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.BlsConfig", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "BlsConfig":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class AccountAuthenticator:
    """AccountAuthenticator represents a foundational model for all authenticators. It provides extensibility by allowing concrete types to interpret and validate transactions based on the encapsulated data."""
    # ID uniquely identifies the authenticator instance.
    id: str = "0"
    # Type specifies the category of the AccountAuthenticator. This type information is essential for differentiating authenticators and ensuring precise data retrieval from the storage layer. Config is a versatile field used in conjunction with the specific type of account authenticator to facilitate complex authentication processes. The interpretation of this field is overloaded, enabling multiple authenticators to utilize it for their respective purposes.
    config: str = ""
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.AccountAuthenticator", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "AccountAuthenticator":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Params:
    """Params defines the parameters for the module."""
    # MaximumUnauthenticatedGas defines the maximum amount of gas that can be used to authenticate a transaction in ante handler without having fee payer authenticated.
    maximum_unauthenticated_gas: str = "0"
    # IsSmartAccountActive defines the state of the authenticator. If set to false, the authenticator module will not be used and the classic cosmos sdk authentication will be used instead.
    is_smart_account_active: bool = False
    # CircuitBreakerControllers defines list of addresses that are allowed to set is_smart_account_active without going through governance.
    circuit_breaker_controllers: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.Params", init=False, repr=False)

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
class QueryParamsRequest:
    """QueryParamsRequest is request type for the Query/Params RPC method."""
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.QueryParamsRequest", init=False, repr=False)

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
    """QueryParamsResponse is response type for the Query/Params RPC method."""
    # params holds all the parameters of this module.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.QueryParamsResponse", init=False, repr=False)

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

@dataclass
class GetAuthenticatorsRequest:
    """MsgGetAuthenticatorsRequest defines the Msg/GetAuthenticators request type."""
    account: str = ""
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.GetAuthenticatorsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GetAuthenticatorsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GetAuthenticatorsResponse:
    """MsgGetAuthenticatorsResponse defines the Msg/GetAuthenticators response type."""
    account_authenticators: List[AccountAuthenticator] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.GetAuthenticatorsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GetAuthenticatorsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GetAuthenticatorRequest:
    """MsgGetAuthenticatorRequest defines the Msg/GetAuthenticator request type."""
    account: str = ""
    authenticator_id: str = "0"
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.GetAuthenticatorRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GetAuthenticatorRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GetAuthenticatorResponse:
    """MsgGetAuthenticatorResponse defines the Msg/GetAuthenticator response type."""
    account_authenticator: Optional[AccountAuthenticator] = None
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.GetAuthenticatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GetAuthenticatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class AuthenticatorData:
    """AuthenticatorData represents a genesis exported account with Authenticators. The address is used as the key, and the account authenticators are stored in the authenticators field."""
    # address is an account address, one address can have many authenticators
    address: str = ""
    # authenticators are the account's authenticators, these can be multiple types including SignatureVerification, AllOfs, CosmWasmAuthenticators, etc
    authenticators: List[AccountAuthenticator] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.AuthenticatorData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "AuthenticatorData":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the authenticator module's genesis state."""
    # params define the parameters for the authenticator module.
    params: Optional[Params] = None
    # next_authenticator_id is the next available authenticator ID.
    next_authenticator_id: str = "0"
    # authenticator_data contains the data for multiple accounts, each with their authenticators.
    authenticator_data: List[AuthenticatorData] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.smartaccount.v1beta1.GenesisState", init=False, repr=False)

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

