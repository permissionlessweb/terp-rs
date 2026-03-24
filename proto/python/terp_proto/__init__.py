"""
terp-proto: Python bindings for terp-core Cosmos SDK proto types.

Usage:
    from terp_proto.terp_clock_v1 import GenesisState, Params
    from terp_proto.cosmos_bank_v1beta1 import MsgSend

Low-level encode/decode (requires native extension — run `just py-build`):
    from terp_proto._native import encode_message, decode_message, registered_types
"""

# Re-export generated modules (populated by `just py-gen`)
try:
    from terp_proto import (
        terp_clock_v1,
        terp_drip_v1,
        terp_feeshare_v1,
        terp_smartaccount_v1beta1,
        cosmos_bank_v1beta1,
        cosmos_auth_v1beta1,
        cosmos_base_v1beta1,
        cosmos_upgrade_v1beta1,
        ibc_applications_transfer_v1,
        ibc_core_channel_v1,
        ibc_core_client_v1,
        ibc_core_connection_v1,
        osmosis_tokenfactory_v1beta1,
    )
except ImportError:
    # Generated modules not yet present — run `just py-gen`
    pass

__all__ = [
    "terp_clock_v1",
    "terp_drip_v1",
    "terp_feeshare_v1",
    "terp_smartaccount_v1beta1",
    "cosmos_bank_v1beta1",
    "cosmos_auth_v1beta1",
    "cosmos_base_v1beta1",
    "cosmos_upgrade_v1beta1",
    "ibc_applications_transfer_v1",
    "ibc_core_channel_v1",
    "ibc_core_client_v1",
    "ibc_core_connection_v1",
    "osmosis_tokenfactory_v1beta1",
]
