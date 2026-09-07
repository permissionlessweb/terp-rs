# Auto-generated from ibc.applications.rate_limiting.v1 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.applications.rate_limiting.v1.rs
# Package: ibc.applications.rate_limiting.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.applications.rate_limiting.v1"

@dataclass
class MsgAddRateLimit:
    """Gov tx to add a new rate limit"""
    # signer defines the x/gov module account address or other authority signing the message
    signer: str = ""
    # Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom
    denom: str = ""
    # ChannelId for the rate limit, on the side of the rate limited chain
    channel_or_client_id: str = ""
    # MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_send: str = ""
    # MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_recv: str = ""
    # DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day)
    duration_hours: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgAddRateLimit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAddRateLimit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgAddRateLimitResponse:
    """MsgAddRateLimitResponse is the return type for AddRateLimit function."""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgAddRateLimitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgAddRateLimitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateRateLimit:
    """Gov tx to update an existing rate limit"""
    # signer defines the x/gov module account address or other authority signing the message
    signer: str = ""
    # Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom
    denom: str = ""
    # ChannelId for the rate limit, on the side of the rate limited chain
    channel_or_client_id: str = ""
    # MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_send: str = ""
    # MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_recv: str = ""
    # DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day)
    duration_hours: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgUpdateRateLimit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateRateLimit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgUpdateRateLimitResponse:
    """MsgUpdateRateLimitResponse is the return type for UpdateRateLimit."""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgUpdateRateLimitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgUpdateRateLimitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRemoveRateLimit:
    """Gov tx to remove a rate limit"""
    # signer defines the x/gov module account address or other authority signing the message
    signer: str = ""
    # Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom
    denom: str = ""
    # ChannelId for the rate limit, on the side of the rate limited chain
    channel_or_client_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgRemoveRateLimit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRemoveRateLimit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgRemoveRateLimitResponse:
    """MsgRemoveRateLimitResponse is the response type for RemoveRateLimit"""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgRemoveRateLimitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgRemoveRateLimitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgResetRateLimit:
    """Gov tx to reset the flow on a rate limit"""
    # signer defines the x/gov module account address or other authority signing the message
    signer: str = ""
    # Denom for the rate limit, as it appears on the rate limited chain When rate limiting a non-native token, this will be an ibc denom
    denom: str = ""
    # ChannelId for the rate limit, on the side of the rate limited chain
    channel_or_client_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgResetRateLimit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgResetRateLimit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class MsgResetRateLimitResponse:
    """MsgResetRateLimitResponse is the response type for ResetRateLimit."""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.MsgResetRateLimitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MsgResetRateLimitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Path:
    """Path holds the denom and channelID that define the rate limited route"""
    denom: str = ""
    channel_or_client_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.Path", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Path":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Quota:
    """Quota defines the rate limit thresholds for transfer packets"""
    # MaxPercentSend defines the threshold for outflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_send: str = ""
    # MaxPercentSend defines the threshold for inflows The threshold is defined as a percentage (e.g. 10 indicates 10%)
    max_percent_recv: str = ""
    # DurationHours specifies the number of hours before the rate limit is reset (e.g. 24 indicates that the rate limit is reset each day)
    duration_hours: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.Quota", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Quota":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class Flow:
    """Flow tracks all the inflows and outflows of a channel."""
    # Inflow defines the total amount of inbound transfers for the given rate limit in the current window
    inflow: str = ""
    # Outflow defines the total amount of outbound transfers for the given rate limit in the current window
    outflow: str = ""
    # ChannelValue stores the total supply of the denom at the start of the rate limit. This is used as the denominator when checking the rate limit threshold The ChannelValue is fixed for the duration of the rate limit window
    channel_value: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.Flow", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "Flow":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class RateLimit:
    """RateLimit stores all the context about a given rate limit, including the relevant denom and channel, rate limit thresholds, and current progress towards the limits"""
    path: Optional[Path] = None
    quota: Optional[Quota] = None
    flow: Optional[Flow] = None
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.RateLimit", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "RateLimit":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class WhitelistedAddressPair:
    """WhitelistedAddressPair represents a sender-receiver combo that is not subject to rate limit restrictions"""
    sender: str = ""
    receiver: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.WhitelistedAddressPair", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "WhitelistedAddressPair":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class HourEpoch:
    """HourEpoch is the epoch type."""
    epoch_number: str = "0"
    duration: Optional[Any  # Duration] = None
    epoch_start_time: Optional[Any  # Timestamp] = None
    epoch_start_height: str = "0"
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.HourEpoch", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "HourEpoch":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllRateLimitsRequest:
    """Queries all rate limits"""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllRateLimitsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllRateLimitsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllRateLimitsResponse:
    """QueryAllRateLimitsResponse returns all the rate limits stored on the chain."""
    rate_limits: List[RateLimit] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllRateLimitsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllRateLimitsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitRequest:
    """Queries a specific rate limit by channel ID and denom"""
    denom: str = ""
    channel_or_client_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitResponse:
    """QueryRateLimitResponse returns a rate limit by denom and channel_or_client_id combination."""
    rate_limit: Optional[RateLimit] = None
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitsByChainIdRequest:
    """Queries all the rate limits for a given chain"""
    chain_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitsByChainIdRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitsByChainIdResponse:
    """QueryRateLimitsByChainIDResponse returns all rate-limits by a chain."""
    rate_limits: List[RateLimit] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitsByChainIDResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitsByChainIdResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitsByChannelOrClientIdRequest:
    """Queries all the rate limits for a given channel or client ID"""
    channel_or_client_id: str = ""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitsByChannelOrClientIdRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryRateLimitsByChannelOrClientIdResponse:
    """QueryRateLimitsByChannelOrClientIDResponse returns all rate-limits by a channel or client id."""
    rate_limits: List[RateLimit] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryRateLimitsByChannelOrClientIDResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryRateLimitsByChannelOrClientIdResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllBlacklistedDenomsRequest:
    """Queries all blacklisted denoms"""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllBlacklistedDenomsRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllBlacklistedDenomsResponse:
    """QueryAllBlacklistedDenomsResponse returns all the blacklisted denosm."""
    denoms: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllBlacklistedDenomsResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllBlacklistedDenomsResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllWhitelistedAddressesRequest:
    """Queries all whitelisted address pairs"""
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesRequest", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllWhitelistedAddressesRequest":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class QueryAllWhitelistedAddressesResponse:
    """QueryAllWhitelistedAddressesResponse returns all whitelisted pairs."""
    address_pairs: List[WhitelistedAddressPair] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.QueryAllWhitelistedAddressesResponse", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "QueryAllWhitelistedAddressesResponse":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class GenesisState:
    """GenesisState defines the ratelimit module's genesis state."""
    rate_limits: List[RateLimit] = field(default_factory=list)
    whitelisted_address_pairs: List[WhitelistedAddressPair] = field(default_factory=list)
    blacklisted_denoms: List[str] = field(default_factory=list)
    pending_send_packet_sequence_numbers: List[str] = field(default_factory=list)
    hour_epoch: Optional[HourEpoch] = None
    TYPE_URL: str = field(default="/ibc.applications.rate_limiting.v1.GenesisState", init=False, repr=False)

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

