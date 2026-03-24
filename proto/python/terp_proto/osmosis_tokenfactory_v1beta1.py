# Auto-generated from osmosis.tokenfactory.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/osmosis.tokenfactory.v1beta1.rs
# Package: osmosis.tokenfactory.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "osmosis.tokenfactory.v1beta1"

@dataclass
class DenomAuthorityMetadata:
    """DenomAuthorityMetadata specifies metadata for addresses that have specific capabilities over a token factory denom. Right now there is only one Admin permission, but is planned to be extended to the future."""
    # Can be empty for no admin, or a valid osmosis address
    admin: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.DenomAuthorityMetadata", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "DenomAuthorityMetadata":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Params:
    """Params defines the parameters for the tokenfactory module."""
    denom_creation_fee: List[Any] = field(default_factory=list)
    # if denom_creation_fee is an empty array, then this field is used to add more gas consumption to the base cost. <https://github.com/CosmWasm/token-factory/issues/11>
    denom_creation_gas_consume: str = "0"
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.Params", init=False, repr=False)

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
class MsgCreateDenom:
    """MsgCreateDenom defines the message structure for the CreateDenom gRPC service method. It allows an account to create a new denom. It requires a sender address and a sub denomination. The (sender_address, sub_denomination) tuple must be unique and cannot be re-used.  The resulting denom created is defined as \<factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is originally set to be the creator, but this can be changed later. The token denom does not indicate the current admin."""
    sender: str = ""
    # subdenom can be up to 44 "alphanumeric" characters long.
    subdenom: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgCreateDenom", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCreateDenom":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgCreateDenomResponse:
    """MsgCreateDenomResponse is the return value of MsgCreateDenom It returns the full string of the newly created denom"""
    new_token_denom: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgCreateDenomResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCreateDenomResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgMint:
    """MsgMint is the sdk.Msg type for allowing an admin account to mint more of a token.  For now, we only support minting to the sender account"""
    sender: str = ""
    amount: Optional[Any  # Coin] = None
    mint_to_address: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgMint", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgMint":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgMintResponse:
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgMintResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgMintResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgBurn:
    """MsgBurn is the sdk.Msg type for allowing an admin account to burn a token.  For now, we only support burning from the sender account."""
    sender: str = ""
    amount: Optional[Any  # Coin] = None
    burn_from_address: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgBurn", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgBurn":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgBurnResponse:
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgBurnResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgBurnResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChangeAdmin:
    """MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign adminship of a denom to a new account"""
    sender: str = ""
    denom: str = ""
    new_admin: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgChangeAdmin", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChangeAdmin":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgChangeAdminResponse:
    """MsgChangeAdminResponse defines the response structure for an executed MsgChangeAdmin message."""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgChangeAdminResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgChangeAdminResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSetDenomMetadata:
    """MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set the denom's bank metadata"""
    sender: str = ""
    metadata: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSetDenomMetadata":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgSetDenomMetadataResponse:
    """MsgSetDenomMetadataResponse defines the response structure for an executed MsgSetDenomMetadata message."""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadataResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgSetDenomMetadataResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgForceTransfer:
    sender: str = ""
    amount: Optional[Any  # Coin] = None
    transfer_from_address: str = ""
    transfer_to_address: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgForceTransfer", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgForceTransfer":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgForceTransferResponse:
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgForceTransferResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgForceTransferResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParams:
    """MsgUpdateParams is the Msg/UpdateParams request type.  Since: cosmos-sdk 0.47"""
    # sender is the address of the governance account.
    authority: str = ""
    # params defines the x/mint parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgUpdateParams", init=False, repr=False)

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
    """MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.  Since: cosmos-sdk 0.47"""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.MsgUpdateParamsResponse", init=False, repr=False)

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
class QueryParamsRequest:
    """QueryParamsRequest is the request type for the Query/Params RPC method."""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryParamsRequest", init=False, repr=False)

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
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryParamsResponse", init=False, repr=False)

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
class QueryDenomAuthorityMetadataRequest:
    """QueryDenomAuthorityMetadataRequest defines the request structure for the DenomAuthorityMetadata gRPC query."""
    denom: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomAuthorityMetadataRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomAuthorityMetadataResponse:
    """QueryDenomAuthorityMetadataResponse defines the response structure for the DenomAuthorityMetadata gRPC query."""
    authority_metadata: Optional[DenomAuthorityMetadata] = None
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomAuthorityMetadataResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomsFromCreatorRequest:
    """QueryDenomsFromCreatorRequest defines the request structure for the DenomsFromCreator gRPC query."""
    creator: str = ""
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomsFromCreatorRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomsFromCreatorResponse:
    """QueryDenomsFromCreatorRequest defines the response structure for the DenomsFromCreator gRPC query."""
    denoms: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomsFromCreatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the tokenfactory module's genesis state."""
    # params defines the paramaters of the module.
    params: Optional[Params] = None
    factory_denoms: List[GenesisDenom] = field(default_factory=list)
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.GenesisState", init=False, repr=False)

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
class GenesisDenom:
    """GenesisDenom defines a tokenfactory denom that is defined within genesis state. The structure contains DenomAuthorityMetadata which defines the denom's admin."""
    denom: str = ""
    authority_metadata: Optional[DenomAuthorityMetadata] = None
    TYPE_URL: str = field(default="/osmosis.tokenfactory.v1beta1.GenesisDenom", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GenesisDenom":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

