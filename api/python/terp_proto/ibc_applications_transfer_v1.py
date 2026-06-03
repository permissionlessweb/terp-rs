# Auto-generated from ibc.applications.transfer.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.transfer.v1.rs
# Package: ibc.applications.transfer.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.transfer.v1"

@dataclass
class Params:
    """Params defines the set of IBC transfer parameters. NOTE: To prevent a single token from being transferred, set the TransfersEnabled parameter to true and then set the bank module's SendEnabled parameter for the denomination to false."""
    # send_enabled enables or disables all cross-chain token transfers from this chain.
    send_enabled: bool = False
    # receive_enabled enables or disables all cross-chain token transfers to this chain.
    receive_enabled: bool = False
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.Params", init=False, repr=False)

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
class MsgTransfer:
    """MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between ICS20 enabled chains. See ICS Spec here: <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>"""
    # the port on which the packet will be sent
    source_port: str = ""
    # the channel by which the packet will be sent
    source_channel: str = ""
    # token to be transferred
    token: Optional[Any  # Any] = None
    # the sender address
    sender: str = ""
    # the recipient address on the destination chain
    receiver: str = ""
    # Timeout height relative to the current block height. If you are sending with IBC v1 protocol, either timeout_height or timeout_timestamp must be set. If you are sending with IBC v2 protocol, timeout_timestamp must be set, and timeout_height must be omitted.
    timeout_height: Optional[Any  # Any] = None
    # Timeout timestamp in absolute nanoseconds since unix epoch. If you are sending with IBC v1 protocol, either timeout_height or timeout_timestamp must be set. If you are sending with IBC v2 protocol, timeout_timestamp must be set.
    timeout_timestamp: str = "0"
    # optional memo
    memo: str = ""
    # optional encoding
    encoding: str = ""
    # boolean flag to indicate if the transfer message is sent with the IBC v2 protocol but uses v1 channel identifiers. In this case, the v1 channel identifiers function as aliases to the underlying client ids. This only needs to be set if the channel IDs are V1 channel identifiers.
    use_aliasing: bool = False
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.MsgTransfer", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTransfer":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgTransferResponse:
    """MsgTransferResponse defines the Msg/Transfer response type."""
    # sequence number of the transfer packet sent
    sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.MsgTransferResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgTransferResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParams:
    """MsgUpdateParams is the Msg/UpdateParams request type."""
    # signer address
    signer: str = ""
    # params defines the transfer parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.MsgUpdateParams", init=False, repr=False)

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
    """MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message."""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.MsgUpdateParamsResponse", init=False, repr=False)

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
class DenomTrace:
    """DenomTrace contains the base denomination for ICS20 fungible tokens and the source tracing information path."""
    # path defines the chain of port/channel identifiers used for tracing the source of the fungible token.
    path: str = ""
    # base denomination of the relayed fungible token.
    base_denom: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.DenomTrace", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "DenomTrace":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Token:
    """Token defines a struct which represents a token to be transferred."""
    # the token denomination
    denom: Optional[Denom] = None
    # the token amount to be transferred
    amount: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.Token", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Token":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Denom:
    """Denom holds the base denom of a Token and a trace of the chains it was sent through."""
    # the base token denomination
    base: str = ""
    # the trace of the token
    trace: List[Hop] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.Denom", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Denom":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Hop:
    """Hop defines a port ID, channel ID pair specifying a unique "hop" in a trace"""
    port_id: str = ""
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.Hop", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Hop":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class FungibleTokenPacketData:
    """FungibleTokenPacketData defines a struct for the packet payload See FungibleTokenPacketData spec: <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>"""
    # the token denomination to be transferred
    denom: str = ""
    # the token amount to be transferred
    amount: str = ""
    # the sender address
    sender: str = ""
    # the recipient address on the destination chain
    receiver: str = ""
    # optional memo
    memo: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.FungibleTokenPacketData", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "FungibleTokenPacketData":
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
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryParamsRequest", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryParamsResponse", init=False, repr=False)

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
class QueryDenomRequest:
    """QueryDenomRequest is the request type for the Query/Denom RPC method"""
    # hash (in hex format) or denom (full denom with ibc prefix) of the on chain denomination.
    hash: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomResponse:
    """QueryDenomResponse is the response type for the Query/Denom RPC method."""
    # denom returns the requested denomination.
    denom: Optional[Denom] = None
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomsRequest:
    """QueryDenomsRequest is the request type for the Query/Denoms RPC method"""
    # pagination defines an optional pagination for the request.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomsResponse:
    """QueryDenomsResponse is the response type for the Query/Denoms RPC method."""
    # denoms returns all denominations.
    denoms: List[Denom] = field(default_factory=list)
    # pagination defines the pagination in the response.
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomHashRequest:
    """QueryDenomHashRequest is the request type for the Query/DenomHash RPC method"""
    # The denomination trace (\[port_id\]/\[channel_id\])+/\[denom\]
    trace: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomHashRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomHashRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryDenomHashResponse:
    """QueryDenomHashResponse is the response type for the Query/DenomHash RPC method."""
    # hash (in hex format) of the denomination trace information.
    hash: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryDenomHashResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryDenomHashResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryEscrowAddressRequest:
    """QueryEscrowAddressRequest is the request type for the EscrowAddress RPC method."""
    # unique port identifier
    port_id: str = ""
    # unique channel identifier
    channel_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryEscrowAddressRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryEscrowAddressRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryEscrowAddressResponse:
    """QueryEscrowAddressResponse is the response type of the EscrowAddress RPC method."""
    # the escrow account address
    escrow_address: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryEscrowAddressResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryEscrowAddressResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryTotalEscrowForDenomRequest:
    """QueryTotalEscrowForDenomRequest is the request type for TotalEscrowForDenom RPC method."""
    denom: str = ""
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryTotalEscrowForDenomRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryTotalEscrowForDenomRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryTotalEscrowForDenomResponse:
    """QueryTotalEscrowForDenomResponse is the response type for TotalEscrowForDenom RPC method."""
    amount: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.QueryTotalEscrowForDenomResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryTotalEscrowForDenomResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Allocation:
    """Allocation defines the spend limit for a particular port and channel"""
    # the port on which the packet will be sent
    source_port: str = ""
    # the channel by which the packet will be sent
    source_channel: str = ""
    # spend limitation on the channel
    spend_limit: List[Any] = field(default_factory=list)
    # allow list of receivers, an empty allow list permits any receiver address
    allow_list: List[str] = field(default_factory=list)
    # allow list of memo strings, an empty list prohibits all memo strings; a list only with "\*" permits any memo string
    allowed_packet_data: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.Allocation", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Allocation":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class TransferAuthorization:
    """TransferAuthorization allows the grantee to spend up to spend_limit coins from the granter's account for ibc transfer on a specific channel"""
    # port and channel amounts
    allocations: List[Allocation] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.TransferAuthorization", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "TransferAuthorization":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the ibc-transfer genesis state"""
    port_id: str = ""
    denoms: List[Denom] = field(default_factory=list)
    params: Optional[Params] = None
    # total_escrowed contains the total amount of tokens escrowed by the transfer module
    total_escrowed: List[Any] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.transfer.v1.GenesisState", init=False, repr=False)

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

