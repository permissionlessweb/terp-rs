# Auto-generated from ibc.core.connection.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.connection.v1.rs
# Package: ibc.core.connection.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.connection.v1"

@dataclass
class ConnectionEnd:
    """ConnectionEnd defines a stateful object on a chain connected to another separate one. NOTE: there must only be 2 defined ConnectionEnds to establish a connection between two chains."""
    # client associated with this connection.
    client_id: str = ""
    # IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection.
    versions: List[Version] = field(default_factory=list)
    # current state of the connection end.
    state: int = 0
    # counterparty chain associated with this connection.
    counterparty: Optional[Counterparty] = None
    # delay period that must pass before a consensus state can be used for packet-verification NOTE: delay period logic is only implemented by some clients.
    delay_period: str = "0"
    TYPE_URL: str = field(default="/ibc.core.connection.v1.ConnectionEnd", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConnectionEnd":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class IdentifiedConnection:
    """IdentifiedConnection defines a connection with additional connection identifier field."""
    # connection identifier.
    id: str = ""
    # client associated with this connection.
    client_id: str = ""
    # IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection
    versions: List[Version] = field(default_factory=list)
    # current state of the connection end.
    state: int = 0
    # counterparty chain associated with this connection.
    counterparty: Optional[Counterparty] = None
    # delay period associated with this connection.
    delay_period: str = "0"
    TYPE_URL: str = field(default="/ibc.core.connection.v1.IdentifiedConnection", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "IdentifiedConnection":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Counterparty:
    """Counterparty defines the counterparty chain associated with a connection end."""
    # identifies the client on the counterparty chain associated with a given connection.
    client_id: str = ""
    # identifies the connection end on the counterparty chain associated with a given connection.
    connection_id: str = ""
    # commitment merkle prefix of the counterparty chain.
    prefix: Optional[Any  # MerklePrefix] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.Counterparty", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Counterparty":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ClientPaths:
    """ClientPaths define all the connection paths for a client state."""
    # list of connection paths
    paths: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.connection.v1.ClientPaths", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ClientPaths":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConnectionPaths:
    """ConnectionPaths define all the connection paths for a given client state."""
    # client state unique identifier
    client_id: str = ""
    # list of connection paths
    paths: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.connection.v1.ConnectionPaths", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConnectionPaths":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Version:
    """Version defines the versioning scheme used to negotiate the IBC version in the connection handshake."""
    # unique version identifier
    identifier: str = ""
    # list of features compatible with the specified identifier
    features: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.connection.v1.Version", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Version":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Params:
    """Params defines the set of Connection parameters."""
    # maximum expected time per block (in nanoseconds), used to enforce block delay. This parameter should reflect the largest amount of time that the chain might reasonably take to produce the next block under normal operating conditions. A safe choice is 3-5x the expected time per block.
    max_expected_time_per_block: str = "0"
    TYPE_URL: str = field(default="/ibc.core.connection.v1.Params", init=False, repr=False)

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
class GenesisState:
    """GenesisState defines the ibc connection submodule's genesis state."""
    connections: List[IdentifiedConnection] = field(default_factory=list)
    client_connection_paths: List[ConnectionPaths] = field(default_factory=list)
    # the sequence for the next generated connection identifier
    next_connection_sequence: str = "0"
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.GenesisState", init=False, repr=False)

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
class MsgConnectionOpenInit:
    """MsgConnectionOpenInit defines the msg sent by an account on Chain A to initialize a connection with Chain B."""
    client_id: str = ""
    counterparty: Optional[Counterparty] = None
    version: Optional[Version] = None
    delay_period: str = "0"
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenInit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenInit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenInitResponse:
    """MsgConnectionOpenInitResponse defines the Msg/ConnectionOpenInit response type."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenInitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenInitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenTry:
    """MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a connection on Chain B."""
    client_id: str = ""
    # Deprecated: this field is unused. Crossing hellos are no longer supported in core IBC.
    previous_connection_id: str = ""
    # Deprecated: this field is unused.
    client_state: Optional[Any  # Any] = None
    counterparty: Optional[Counterparty] = None
    delay_period: str = "0"
    counterparty_versions: List[Version] = field(default_factory=list)
    proof_height: Optional[Any  # Height] = None
    # proof of the initialization the connection on Chain A: `UNINITIALIZED ->  INIT`
    proof_init: str = ""
    # Deprecated: this field is unused.
    proof_client: str = ""
    # Deprecated: this field is unused.
    proof_consensus: str = ""
    # Deprecated: this field is unused.
    consensus_height: Optional[Any  # Height] = None
    signer: str = ""
    # Deprecated: this field is unused.
    host_consensus_state_proof: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenTry", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenTry":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenTryResponse:
    """MsgConnectionOpenTryResponse defines the Msg/ConnectionOpenTry response type."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenTryResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenTryResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenAck:
    """MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to acknowledge the change of connection state to TRYOPEN on Chain B."""
    connection_id: str = ""
    counterparty_connection_id: str = ""
    version: Optional[Version] = None
    # Deprecated: this field is unused.
    client_state: Optional[Any  # Any] = None
    proof_height: Optional[Any  # Height] = None
    # proof of the initialization the connection on Chain B: `UNINITIALIZED ->  TRYOPEN`
    proof_try: str = ""
    # Deprecated: this field is unused.
    proof_client: str = ""
    # Deprecated: this field is unused.
    proof_consensus: str = ""
    # Deprecated: this field is unused.
    consensus_height: Optional[Any  # Height] = None
    signer: str = ""
    # Deprecated: this field is unused.
    host_consensus_state_proof: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenAck", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenAck":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenAckResponse:
    """MsgConnectionOpenAckResponse defines the Msg/ConnectionOpenAck response type."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenAckResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenAckResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenConfirm:
    """MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to acknowledge the change of connection state to OPEN on Chain A."""
    connection_id: str = ""
    # proof for the change of the connection state on Chain A: `INIT -> OPEN`
    proof_ack: str = ""
    proof_height: Optional[Any  # Height] = None
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenConfirm", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenConfirm":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgConnectionOpenConfirmResponse:
    """MsgConnectionOpenConfirmResponse defines the Msg/ConnectionOpenConfirm response type."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgConnectionOpenConfirmResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgConnectionOpenConfirmResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParams:
    """MsgUpdateParams defines the sdk.Msg type to update the connection parameters."""
    # signer address
    signer: str = ""
    # params defines the connection parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgUpdateParams", init=False, repr=False)

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
    """MsgUpdateParamsResponse defines the MsgUpdateParams response type."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.MsgUpdateParamsResponse", init=False, repr=False)

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
class QueryConnectionRequest:
    """QueryConnectionRequest is the request type for the Query/Connection RPC method"""
    # connection unique identifier
    connection_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionResponse:
    """QueryConnectionResponse is the response type for the Query/Connection RPC method. Besides the connection end, it includes a proof and the height from which the proof was retrieved."""
    # connection associated with the request identifier
    connection: Optional[ConnectionEnd] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionsRequest:
    """QueryConnectionsRequest is the request type for the Query/Connections RPC method"""
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionsResponse:
    """QueryConnectionsResponse is the response type for the Query/Connections RPC method."""
    # list of stored connections of the chain.
    connections: List[IdentifiedConnection] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    # query block height
    height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientConnectionsRequest:
    """QueryClientConnectionsRequest is the request type for the Query/ClientConnections RPC method"""
    # client identifier associated with a connection
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryClientConnectionsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientConnectionsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientConnectionsResponse:
    """QueryClientConnectionsResponse is the response type for the Query/ClientConnections RPC method"""
    # slice of all the connection paths associated with a client.
    connection_paths: List[str] = field(default_factory=list)
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was generated
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryClientConnectionsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientConnectionsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionClientStateRequest:
    """QueryConnectionClientStateRequest is the request type for the Query/ConnectionClientState RPC method"""
    # connection identifier
    connection_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionClientStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionClientStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionClientStateResponse:
    """QueryConnectionClientStateResponse is the response type for the Query/ConnectionClientState RPC method"""
    # client state associated with the channel
    identified_client_state: Optional[Any  # Any] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionClientStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionClientStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionConsensusStateRequest:
    """QueryConnectionConsensusStateRequest is the request type for the Query/ConnectionConsensusState RPC method"""
    # connection identifier
    connection_id: str = ""
    revision_number: str = "0"
    revision_height: str = "0"
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionConsensusStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionConsensusStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionConsensusStateResponse:
    """QueryConnectionConsensusStateResponse is the response type for the Query/ConnectionConsensusState RPC method"""
    # consensus state associated with the channel
    consensus_state: Optional[Any  # Any] = None
    # client ID associated with the consensus state
    client_id: str = ""
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Any  # Height] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionConsensusStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionConsensusStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionParamsRequest:
    """QueryConnectionParamsRequest is the request type for the Query/ConnectionParams RPC method."""
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionParamsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionParamsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConnectionParamsResponse:
    """QueryConnectionParamsResponse is the response type for the Query/ConnectionParams RPC method."""
    # params defines the parameters of the module.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.core.connection.v1.QueryConnectionParamsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConnectionParamsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

