# Auto-generated from cosmos_proto — do not edit.
# Source: terp-rs/proto/src/gen/cosmos_proto.rs
# Package: cosmos_proto
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos_proto"

@dataclass
class InterfaceDescriptor:
    """InterfaceDescriptor describes an interface type to be used with accepts_interface and implements_interface and declared by declare_interface."""
    # name is the name of the interface. It should be a short-name (without a period) such that the fully qualified name of the interface will be package.name, ex. for the package a.b and interface named C, the fully-qualified name will be a.b.C.
    name: str = ""
    # description is a human-readable description of the interface and its purpose.
    description: str = ""
    TYPE_URL: str = field(default="/cosmos_proto.InterfaceDescriptor", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InterfaceDescriptor":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ScalarDescriptor:
    """ScalarDescriptor describes an scalar type to be used with the scalar field option and declared by declare_scalar. Scalars extend simple protobuf built-in types with additional syntax and semantics, for instance to represent big integers. Scalars should ideally define an encoding such that there is only one valid syntactical representation for a given semantic meaning, i.e. the encoding should be deterministic."""
    # name is the name of the scalar. It should be a short-name (without a period) such that the fully qualified name of the scalar will be package.name, ex. for the package a.b and scalar named C, the fully-qualified name will be a.b.C.
    name: str = ""
    # description is a human-readable description of the scalar and its encoding format. For instance a big integer or decimal scalar should specify precisely the expected encoding format.
    description: str = ""
    # field_type is the type of field with which this scalar can be used. Scalars can be used with one and only one type of field so that encoding standards and simple and clear. Currently only string and bytes fields are supported for scalars.
    field_type: int = 0
    TYPE_URL: str = field(default="/cosmos_proto.ScalarDescriptor", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ScalarDescriptor":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

