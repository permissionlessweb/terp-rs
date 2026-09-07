# Auto-generated from cosmos.upgrade.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/cosmos.upgrade.v1beta1.rs
# Package: cosmos.upgrade.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos.upgrade.v1beta1"

@dataclass
class Plan:
    """Plan specifies information about a planned upgrade and when it should occur."""
    # Sets the name for the upgrade. This name will be used by the upgraded version of the software to apply any special "on-upgrade" commands during the first BeginBlock method after the upgrade is applied. It is also used to detect whether a software version can handle a given upgrade. If no upgrade handler with this name has been set in the software, it will be assumed that the software is out-of-date when the upgrade Time or Height is reached and the software will exit.
    name: str = ""
    # Deprecated: Time based upgrades have been deprecated. Time based upgrade logic has been removed from the SDK. If this field is not empty, an error will be thrown.
    time: Optional[Any  # Timestamp] = None
    # The height at which the upgrade must be performed.
    height: str = "0"
    # Any application specific upgrade info to be included on-chain such as a git commit that validators could automatically upgrade to
    info: str = ""
    # Deprecated: UpgradedClientState field has been deprecated. IBC upgrade logic has been moved to the IBC module in the sub module 02-client. If this field is not empty, an error will be thrown.
    upgraded_client_state: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/cosmos.upgrade.v1beta1.Plan", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Plan":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class SoftwareUpgradeProposal:
    """SoftwareUpgradeProposal is a gov Content type for initiating a software upgrade. Deprecated: This legacy proposal is deprecated in favor of Msg-based gov proposals, see MsgSoftwareUpgrade."""
    # title of the proposal
    title: str = ""
    # description of the proposal
    description: str = ""
    # plan of the proposal
    plan: Optional[Plan] = None
    TYPE_URL: str = field(default="/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "SoftwareUpgradeProposal":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CancelSoftwareUpgradeProposal:
    """CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software upgrade. Deprecated: This legacy proposal is deprecated in favor of Msg-based gov proposals, see MsgCancelUpgrade."""
    # title of the proposal
    title: str = ""
    # description of the proposal
    description: str = ""
    TYPE_URL: str = field(default="/cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CancelSoftwareUpgradeProposal":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ModuleVersion:
    """ModuleVersion specifies a module and its consensus version."""
    # name of the app module
    name: str = ""
    # consensus version of the app module
    version: str = "0"
    TYPE_URL: str = field(default="/cosmos.upgrade.v1beta1.ModuleVersion", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ModuleVersion":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

