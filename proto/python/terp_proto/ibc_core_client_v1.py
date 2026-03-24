# Auto-generated from ibc.core.client.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.client.v1.rs
# Package: ibc.core.client.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.client.v1"

@dataclass
class IdentifiedClientState:
    """IdentifiedClientState defines a client state with an additional client identifier field."""
    # client identifier
    client_id: str = ""
    # client state
    client_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.IdentifiedClientState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "IdentifiedClientState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConsensusStateWithHeight:
    """ConsensusStateWithHeight defines a consensus state with an additional height field."""
    # consensus state height
    height: Optional[Height] = None
    # consensus state
    consensus_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.ConsensusStateWithHeight", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConsensusStateWithHeight":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ClientConsensusStates:
    """ClientConsensusStates defines all the stored consensus states for a given client."""
    # client identifier
    client_id: str = ""
    # consensus states and their heights associated with the client
    consensus_states: List[ConsensusStateWithHeight] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.client.v1.ClientConsensusStates", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ClientConsensusStates":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Height:
    """Height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients  Normally the RevisionHeight is incremented at each height while keeping RevisionNumber the same. However some consensus algorithms may choose to reset the height in certain conditions e.g. hard forks, state-machine breaking changes In these cases, the RevisionNumber is incremented so that height continues to be monitonically increasing even as the RevisionHeight gets reset  Please note that json tags for generated Go code are overridden to explicitly exclude the omitempty jsontag. This enforces the Go json marshaller to always emit zero values for both revision_number and revision_height."""
    # the revision that the client is currently on
    revision_number: str = "0"
    # the height within the given revision
    revision_height: str = "0"
    TYPE_URL: str = field(default="/ibc.core.client.v1.Height", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Height":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Params:
    """Params defines the set of IBC light client parameters."""
    # allowed_clients defines the list of allowed client state types which can be created and interacted with. If a client type is removed from the allowed clients list, usage of this client will be disabled until it is added again to the list.
    allowed_clients: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.client.v1.Params", init=False, repr=False)

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
    """GenesisState defines the ibc client submodule's genesis state."""
    # client states with their corresponding identifiers
    clients: List[IdentifiedClientState] = field(default_factory=list)
    # consensus states from each client
    clients_consensus: List[ClientConsensusStates] = field(default_factory=list)
    # metadata from each client
    clients_metadata: List[IdentifiedGenesisMetadata] = field(default_factory=list)
    params: Optional[Params] = None
    # Deprecated: create_localhost has been deprecated. The localhost client is automatically created at genesis.
    create_localhost: bool = False
    # the sequence for the next generated client identifier
    next_client_sequence: str = "0"
    TYPE_URL: str = field(default="/ibc.core.client.v1.GenesisState", init=False, repr=False)

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
class GenesisMetadata:
    """GenesisMetadata defines the genesis type for metadata that will be used to export all client store keys that are not client or consensus states."""
    # store key of metadata without clientID-prefix
    key: str = ""
    # metadata value
    value: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.GenesisMetadata", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "GenesisMetadata":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class IdentifiedGenesisMetadata:
    """IdentifiedGenesisMetadata has the client metadata with the corresponding client id."""
    client_id: str = ""
    client_metadata: List[GenesisMetadata] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.client.v1.IdentifiedGenesisMetadata", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "IdentifiedGenesisMetadata":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgCreateClient:
    """MsgCreateClient defines a message to create an IBC client"""
    # light client state
    client_state: Optional[Any  # Any] = None
    # consensus state associated with the client that corresponds to a given height.
    consensus_state: Optional[Any  # Any] = None
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgCreateClient", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCreateClient":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgCreateClientResponse:
    """MsgCreateClientResponse defines the Msg/CreateClient response type."""
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgCreateClientResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgCreateClientResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateClient:
    """MsgUpdateClient defines an sdk.Msg to update a IBC client state using the given client message."""
    # client unique identifier
    client_id: str = ""
    # client message to update the light client
    client_message: Optional[Any  # Any] = None
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpdateClient", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateClient":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateClientResponse:
    """MsgUpdateClientResponse defines the Msg/UpdateClient response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpdateClientResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateClientResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpgradeClient:
    """MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client state"""
    # client unique identifier
    client_id: str = ""
    # upgraded client state
    client_state: Optional[Any  # Any] = None
    # upgraded consensus state, only contains enough information to serve as a basis of trust in update logic
    consensus_state: Optional[Any  # Any] = None
    # proof that old chain committed to new client
    proof_upgrade_client: str = ""
    # proof that old chain committed to new consensus state
    proof_upgrade_consensus_state: str = ""
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpgradeClient", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpgradeClient":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpgradeClientResponse:
    """MsgUpgradeClientResponse defines the Msg/UpgradeClient response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpgradeClientResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpgradeClientResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRecoverClient:
    """MsgRecoverClient defines the message used to recover a frozen or expired client."""
    # the client identifier for the client to be updated if the proposal passes
    subject_client_id: str = ""
    # the substitute client identifier for the client which will replace the subject client
    substitute_client_id: str = ""
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgRecoverClient", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRecoverClient":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRecoverClientResponse:
    """MsgRecoverClientResponse defines the Msg/RecoverClient response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgRecoverClientResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRecoverClientResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgIbcSoftwareUpgrade:
    """MsgIBCSoftwareUpgrade defines the message used to schedule an upgrade of an IBC client using a v1 governance proposal"""
    plan: Optional[Any  # Any] = None
    # An UpgradedClientState must be provided to perform an IBC breaking upgrade. This will make the chain commit to the correct upgraded (self) client state before the upgrade occurs, so that connecting chains can verify that the new upgraded client is valid by verifying a proof on the previous version of the chain. This will allow IBC connections to persist smoothly across planned chain upgrades. Correspondingly, the UpgradedClientState field has been deprecated in the Cosmos SDK to allow for this logic to exist solely in the 02-client module.
    upgraded_client_state: Optional[Any  # Any] = None
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgIBCSoftwareUpgrade", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgIbcSoftwareUpgrade":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgIbcSoftwareUpgradeResponse:
    """MsgIBCSoftwareUpgradeResponse defines the Msg/IBCSoftwareUpgrade response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgIBCSoftwareUpgradeResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgIbcSoftwareUpgradeResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateParams:
    """MsgUpdateParams defines the sdk.Msg type to update the client parameters."""
    # signer address
    signer: str = ""
    # params defines the client parameters to update.  NOTE: All parameters must be supplied.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpdateParams", init=False, repr=False)

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
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgUpdateParamsResponse", init=False, repr=False)

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
class MsgDeleteClientCreator:
    """MsgDeleteClientCreator defines a message to delete the client creator of a client"""
    # client identifier
    client_id: str = ""
    # signer address
    signer: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgDeleteClientCreator", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgDeleteClientCreator":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgDeleteClientCreatorResponse:
    """MsgDeleteClientCreatorResponse defines the Msg/DeleteClientCreator response type."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.MsgDeleteClientCreatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgDeleteClientCreatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStateRequest:
    """QueryClientStateRequest is the request type for the Query/ClientState RPC method"""
    # client state unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStateResponse:
    """QueryClientStateResponse is the response type for the Query/ClientState RPC method. Besides the client state, it includes a proof and the height from which the proof was retrieved."""
    # client state associated with the request identifier
    client_state: Optional[Any  # Any] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Height] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStatesRequest:
    """QueryClientStatesRequest is the request type for the Query/ClientStates RPC method"""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStatesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStatesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStatesResponse:
    """QueryClientStatesResponse is the response type for the Query/ClientStates RPC method."""
    # list of stored ClientStates of the chain.
    client_states: List[IdentifiedClientState] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStatesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStatesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStateRequest:
    """QueryConsensusStateRequest is the request type for the Query/ConsensusState RPC method. Besides the consensus state, it includes a proof and the height from which the proof was retrieved."""
    # client identifier
    client_id: str = ""
    # consensus state revision number
    revision_number: str = "0"
    # consensus state revision height
    revision_height: str = "0"
    # latest_height overrides the height field and queries the latest stored ConsensusState
    latest_height: bool = False
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStateResponse:
    """QueryConsensusStateResponse is the response type for the Query/ConsensusState RPC method"""
    # consensus state associated with the client identifier at the given height
    consensus_state: Optional[Any  # Any] = None
    # merkle proof of existence
    proof: str = ""
    # height at which the proof was retrieved
    proof_height: Optional[Height] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStatesRequest:
    """QueryConsensusStatesRequest is the request type for the Query/ConsensusStates RPC method."""
    # client identifier
    client_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStatesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStatesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStatesResponse:
    """QueryConsensusStatesResponse is the response type for the Query/ConsensusStates RPC method"""
    # consensus states associated with the identifier
    consensus_states: List[ConsensusStateWithHeight] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStatesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStatesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStateHeightsRequest:
    """QueryConsensusStateHeightsRequest is the request type for Query/ConsensusStateHeights RPC method."""
    # client identifier
    client_id: str = ""
    # pagination request
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStateHeightsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStateHeightsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryConsensusStateHeightsResponse:
    """QueryConsensusStateHeightsResponse is the response type for the Query/ConsensusStateHeights RPC method"""
    # consensus state heights
    consensus_state_heights: List[Height] = field(default_factory=list)
    # pagination response
    pagination: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryConsensusStateHeightsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryConsensusStateHeightsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStatusRequest:
    """QueryClientStatusRequest is the request type for the Query/ClientStatus RPC method"""
    # client unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStatusRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStatusRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientStatusResponse:
    """QueryClientStatusResponse is the response type for the Query/ClientStatus RPC method. It returns the current status of the IBC client."""
    status: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientStatusResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientStatusResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientParamsRequest:
    """QueryClientParamsRequest is the request type for the Query/ClientParams RPC method."""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientParamsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientParamsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientParamsResponse:
    """QueryClientParamsResponse is the response type for the Query/ClientParams RPC method."""
    # params defines the parameters of the module.
    params: Optional[Params] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientParamsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientParamsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientCreatorRequest:
    """QueryClientCreatorRequest is the request type for the Query/ClientCreator RPC method."""
    # client unique identifier
    client_id: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientCreatorRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientCreatorRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryClientCreatorResponse:
    """QueryClientCreatorResponse is the response type for the Query/ClientCreator RPC method."""
    # creator of the client
    creator: str = ""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryClientCreatorResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryClientCreatorResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUpgradedClientStateRequest:
    """QueryUpgradedClientStateRequest is the request type for the Query/UpgradedClientState RPC method"""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryUpgradedClientStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUpgradedClientStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUpgradedClientStateResponse:
    """QueryUpgradedClientStateResponse is the response type for the Query/UpgradedClientState RPC method."""
    # client state associated with the request identifier
    upgraded_client_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryUpgradedClientStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUpgradedClientStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUpgradedConsensusStateRequest:
    """QueryUpgradedConsensusStateRequest is the request type for the Query/UpgradedConsensusState RPC method"""
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryUpgradedConsensusStateRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUpgradedConsensusStateRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryUpgradedConsensusStateResponse:
    """QueryUpgradedConsensusStateResponse is the response type for the Query/UpgradedConsensusState RPC method."""
    # Consensus state associated with the request identifier
    upgraded_consensus_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryUpgradedConsensusStateResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryUpgradedConsensusStateResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryVerifyMembershipRequest:
    """QueryVerifyMembershipRequest is the request type for the Query/VerifyMembership RPC method"""
    # client unique identifier.
    client_id: str = ""
    # the proof to be verified by the client.
    proof: str = ""
    # the height of the commitment root at which the proof is verified.
    proof_height: Optional[Height] = None
    # the value which is proven.
    value: str = ""
    # optional time delay
    time_delay: str = "0"
    # optional block delay
    block_delay: str = "0"
    # the commitment key path.
    merkle_path: Optional[Any  # MerklePath] = None
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryVerifyMembershipRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryVerifyMembershipRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryVerifyMembershipResponse:
    """QueryVerifyMembershipResponse is the response type for the Query/VerifyMembership RPC method"""
    # boolean indicating success or failure of proof verification.
    success: bool = False
    TYPE_URL: str = field(default="/ibc.core.client.v1.QueryVerifyMembershipResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryVerifyMembershipResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

