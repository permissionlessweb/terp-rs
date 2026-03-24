# Auto-generated from cosmos.base.query.v1beta1 — do not edit.
# Source: terp-rs/proto/src/gen/cosmos.base.query.v1beta1.rs
# Package: cosmos.base.query.v1beta1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos.base.query.v1beta1"

@dataclass
class PageRequest:
    """PageRequest is to be embedded in gRPC request messages for efficient pagination. Ex:  message SomeRequest { Foo some_parameter = 1; PageRequest pagination = 2; }"""
    # key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set.
    key: str = ""
    # offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set.
    offset: str = "0"
    # limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app.
    limit: str = "0"
    # count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set.
    count_total: bool = False
    # reverse is set to true if results are to be returned in the descending order.
    reverse: bool = False
    TYPE_URL: str = field(default="/cosmos.base.query.v1beta1.PageRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PageRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class PageResponse:
    """PageResponse is to be embedded in gRPC response messages where the corresponding request message has used PageRequest.  message SomeResponse { repeated Bar results = 1; PageResponse page = 2; }"""
    # next_key is the key to be passed to PageRequest.key to query the next page most efficiently. It will be empty if there are no more results.
    next_key: str = ""
    # total is total number of results available if PageRequest.count_total was set, its value is undefined otherwise
    total: str = "0"
    TYPE_URL: str = field(default="/cosmos.base.query.v1beta1.PageResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "PageResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

