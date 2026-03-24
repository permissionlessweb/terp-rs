# Auto-generated from ibc.core.types.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.types.v1.rs
# Package: ibc.core.types.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.types.v1"

@dataclass
class GenesisState:
    """GenesisState defines the ibc module's genesis state."""
    # ICS002 - Clients genesis state
    client_genesis: Optional[GenesisState] = None
    # ICS003 - Connections genesis state
    connection_genesis: Optional[Any  # Any] = None
    # ICS004 - Channel genesis state
    channel_genesis: Optional[GenesisState] = None
    # ICS002 - Clients/v2 genesis state
    client_v2_genesis: Optional[Any  # Any] = None
    # ICS004 - Channel/v2 genesis state
    channel_v2_genesis: Optional[Any  # Any] = None
    TYPE_URL: str = field(default="/ibc.core.types.v1.GenesisState", init=False, repr=False)

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

