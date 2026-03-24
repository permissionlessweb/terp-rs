# Auto-generated from terp.feeshare.v1 — do not edit.
# Source: terp-rs/proto/src/gen/terp.feeshare.v1.rs
# Package: terp.feeshare.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "terp.feeshare.v1"

@dataclass
class FeeShare:
    """FeeShare defines an instance that organizes fee distribution conditions for the owner of a given smart contract"""
    # contract_address is the bech32 address of a registered contract in string form
    contract_address: str = ""
    # deployer_address is the bech32 address of message sender. It must be the same as the contracts admin address.
    deployer_address: str = ""
    # withdrawer_address is the bech32 address of account receiving the transaction fees.
    withdrawer_address: str = ""
    TYPE_URL: str = field(default="/terp.feeshare.v1.FeeShare", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "FeeShare":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the module's genesis state."""
    # params are the feeshare module parameters
    params: Optional[Params] = None
    # FeeShare is a slice of active registered contracts for fee distribution
    fee_share: List[FeeShare] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.feeshare.v1.GenesisState", init=False, repr=False)

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
class Params:
    """Params defines the feeshare module params"""
    # enable_feeshare defines a parameter to enable the feeshare module
    enable_fee_share: bool = False
    # developer_shares defines the proportion of the transaction fees to be distributed to the registered contract owner
    developer_shares: str = ""
    # allowed_denoms defines the list of denoms that are allowed to be paid to the contract withdraw addresses. If said denom is not in the list, the fees will ONLY be sent to the community pool. If this list is empty, all denoms are allowed.
    allowed_denoms: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/terp.feeshare.v1.Params", init=False, repr=False)

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
class MsgRegisterFeeShare:
    """MsgRegisterFeeShare defines a message that registers a FeeShare"""
    # contract_address in bech32 format
    contract_address: str = ""
    # deployer_address is the bech32 address of message sender. It must be the same the contract's admin address
    deployer_address: str = ""
    # withdrawer_address is the bech32 address of account receiving the transaction fees
    withdrawer_address: str = ""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgRegisterFeeShare", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRegisterFeeShare":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRegisterFeeShareResponse:
    """MsgRegisterFeeShareResponse defines the MsgRegisterFeeShare response type"""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgRegisterFeeShareResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRegisterFeeShareResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateFeeShare:
    """MsgUpdateFeeShare defines a message that updates the withdrawer address for a registered FeeShare"""
    # contract_address in bech32 format
    contract_address: str = ""
    # deployer_address is the bech32 address of message sender. It must be the same the contract's admin address
    deployer_address: str = ""
    # withdrawer_address is the bech32 address of account receiving the transaction fees
    withdrawer_address: str = ""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgUpdateFeeShare", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateFeeShare":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateFeeShareResponse:
    """MsgUpdateFeeShareResponse defines the MsgUpdateFeeShare response type"""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgUpdateFeeShareResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateFeeShareResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgCancelFeeShare:
    """MsgCancelFeeShare defines a message that cancels a registered FeeShare"""
    # contract_address in bech32 format
    contract_address: str = ""
    # deployer_address is the bech32 address of message sender. It must be the same the contract's admin address
    deployer_address: str = ""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgCancelFeeShare", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCancelFeeShare":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgCancelFeeShareResponse:
    """MsgCancelFeeShareResponse defines the MsgCancelFeeShare response type"""
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgCancelFeeShareResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCancelFeeShareResponse":
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
    # authority is the address that controls the module (defaults to x/gov unless overwritten).
    authority: str = ""
    # params defines the x/feeshare parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgUpdateParams", init=False, repr=False)

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
    TYPE_URL: str = field(default="/terp.feeshare.v1.MsgUpdateParamsResponse", init=False, repr=False)

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
class QueryFeeSharesRequest:
    """QueryFeeSharesRequest is the request type for the Query/FeeShares RPC method."""
    # pagination defines an optional pagination for the request.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryFeeSharesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryFeeSharesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryFeeSharesResponse:
    """QueryFeeSharesResponse is the response type for the Query/FeeShares RPC method."""
    # FeeShare is a slice of all stored Reveneue
    feeshare: List[FeeShare] = field(default_factory=list)
    # pagination defines the pagination in the response.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryFeeSharesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryFeeSharesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryFeeShareRequest:
    """QueryFeeShareRequest is the request type for the Query/FeeShare RPC method."""
    # contract_address of a registered contract in bech32 format
    contract_address: str = ""
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryFeeShareRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryFeeShareRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryFeeShareResponse:
    """QueryFeeShareResponse is the response type for the Query/FeeShare RPC method."""
    # FeeShare is a stored Reveneue for the queried contract
    feeshare: Optional[FeeShare] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryFeeShareResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryFeeShareResponse":
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
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryParamsRequest", init=False, repr=False)

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
    # params is the returned FeeShare parameter
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryParamsResponse", init=False, repr=False)

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
class QueryDeployerFeeSharesRequest:
    """QueryDeployerFeeSharesRequest is the request type for the Query/DeployerFeeShares RPC method."""
    # deployer_address in bech32 format
    deployer_address: str = ""
    # pagination defines an optional pagination for the request.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryDeployerFeeSharesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDeployerFeeSharesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDeployerFeeSharesResponse:
    """QueryDeployerFeeSharesResponse is the response type for the Query/DeployerFeeShares RPC method."""
    # contract_addresses is the slice of registered contract addresses for a deployer
    contract_addresses: List[str] = field(default_factory=list)
    # pagination defines the pagination in the response.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryDeployerFeeSharesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDeployerFeeSharesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryWithdrawerFeeSharesRequest:
    """QueryWithdrawerFeeSharesRequest is the request type for the Query/WithdrawerFeeShares RPC method."""
    # withdrawer_address in bech32 format
    withdrawer_address: str = ""
    # pagination defines an optional pagination for the request.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryWithdrawerFeeSharesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryWithdrawerFeeSharesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryWithdrawerFeeSharesResponse:
    """QueryWithdrawerFeeSharesResponse is the response type for the Query/WithdrawerFeeShares RPC method."""
    # contract_addresses is the slice of registered contract addresses for a withdrawer
    contract_addresses: List[str] = field(default_factory=list)
    # pagination defines the pagination in the response.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/terp.feeshare.v1.QueryWithdrawerFeeSharesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryWithdrawerFeeSharesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

