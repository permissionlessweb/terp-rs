# Auto-generated from ibc.lightclients.tendermint.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.lightclients.tendermint.v1.rs
# Package: ibc.lightclients.tendermint.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.lightclients.tendermint.v1"

@dataclass
class ClientState:
    """ClientState from Tendermint tracks the current validator set, latest height, and a possible frozen height."""
    chain_id: str = ""
    trust_level: Optional[Fraction] = None
    # duration of the period since the LatestTimestamp during which the submitted headers are valid for upgrade
    trusting_period: Optional[Any  # Duration] = None
    # duration of the staking unbonding period
    unbonding_period: Optional[Any  # Duration] = None
    # defines how much new (untrusted) header's Time can drift into the future.
    max_clock_drift: Optional[Any  # Duration] = None
    # Block height when the client was frozen due to a misbehaviour
    frozen_height: Optional[Any  # Any] = None
    # Latest height the client was updated to
    latest_height: Optional[Any  # Any] = None
    # Proof specifications used in verifying counterparty state
    proof_specs: List[Any] = field(default_factory=list)
    # Path at which next upgraded client will be committed. Each element corresponds to the key for a single CommitmentProof in the chained proof. NOTE: ClientState must stored under `{upgradePath}/{upgradeHeight}/clientState` ConsensusState must be stored under `{upgradepath}/{upgradeHeight}/consensusState` For SDK chains using the default upgrade module, upgrade_path should be \[\]string{"upgrade", "upgradedIBCState"}\`
    upgrade_path: List[str] = field(default_factory=list)
    # allow_update_after_expiry is deprecated
    allow_update_after_expiry: bool = False
    # allow_update_after_misbehaviour is deprecated
    allow_update_after_misbehaviour: bool = False
    TYPE_URL: str = field(default="/ibc.lightclients.tendermint.v1.ClientState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ClientState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ConsensusState:
    """ConsensusState defines the consensus state from Tendermint."""
    # timestamp that corresponds to the block height in which the ConsensusState was stored.
    timestamp: Optional[Any  # Timestamp] = None
    # commitment root (i.e app hash)
    root: Optional[Any  # Any] = None
    next_validators_hash: str = ""
    TYPE_URL: str = field(default="/ibc.lightclients.tendermint.v1.ConsensusState", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ConsensusState":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Misbehaviour:
    """Misbehaviour is a wrapper over two conflicting Headers that implements Misbehaviour interface expected by ICS-02"""
    # ClientID is deprecated
    client_id: str = ""
    header_1: Optional[Header] = None
    header_2: Optional[Header] = None
    TYPE_URL: str = field(default="/ibc.lightclients.tendermint.v1.Misbehaviour", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Misbehaviour":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Header:
    """Header defines the Tendermint client consensus Header. It encapsulates all the information necessary to update from a trusted Tendermint ConsensusState. The inclusion of TrustedHeight and TrustedValidators allows this update to process correctly, so long as the ConsensusState for the TrustedHeight exists, this removes race conditions among relayers The SignedHeader and ValidatorSet are the new untrusted update fields for the client. The TrustedHeight is the height of a stored ConsensusState on the client that will be used to verify the new untrusted header. The Trusted ConsensusState must be within the unbonding period of current time in order to correctly verify, and the TrustedValidators must hash to TrustedConsensusState.NextValidatorsHash since that is the last trusted validator set at the TrustedHeight."""
    signed_header: Optional[Any  # Any] = None
    validator_set: Optional[Any  # Any] = None
    trusted_height: Optional[Any  # Any] = None
    trusted_validators: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.lightclients.tendermint.v1.Header", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Header":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Fraction:
    """Fraction defines the protobuf message type for tmmath.Fraction that only supports positive values."""
    numerator: str = "0"
    denominator: str = "0"
    TYPE_URL: str = field(default="/ibc.lightclients.tendermint.v1.Fraction", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Fraction":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

