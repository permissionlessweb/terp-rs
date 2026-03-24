# Auto-generated from ibc.core.commitment.v2 — do not edit.
# Source: terp-rs/proto/src/gen/ibc.core.commitment.v2.rs
# Package: ibc.core.commitment.v2
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "ibc.core.commitment.v2"

@dataclass
class MerklePath:
    """MerklePath is the path used to verify commitment proofs, which can be an arbitrary structured object (defined by a commitment type). ICS-23 verification supports membership proofs for nested merkle trees. The ICS-24 standard provable keys MUST be stored in the lowest level tree with an optional prefix. The IC24 provable tree may then be stored in a higher level tree(s) that hash up to the root hash stored in the consensus state of the client. Each element of the path represents the key of a merkle tree from the root to the leaf. The elements of the path before the final element must be the path to the tree that contains the ICS24 provable store. Thus, it should remain constant for all ICS24 proofs. The final element of the path is the key of the leaf in the ICS24 provable store, Thus IBC core will append the ICS24 path to the final element of the MerklePath stored in the counterparty to create the full path to the leaf for proof verification. Examples: Cosmos SDK: The Cosmos SDK commits to a multi-tree where each store is an IAVL tree and all store hashes are hashed in a simple merkle tree to get the final root hash. Thus, the MerklePath in the counterparty MerklePrefix has the following structure: \["ibc", ""\] The core IBC handler will append the ICS24 path to the final element of the MerklePath like so: \["ibc", "{packetCommitmentPath}"\] which will then be used for final verification. Ethereum: The Ethereum client commits to a single Patricia merkle trie. The ICS24 provable store is managed by the smart contract state. Each smart contract has a specific prefix reserved within the global trie. Thus the MerklePath in the counterparty is the prefix to the smart contract state in the global trie. Since there is only one tree in the commitment structure of ethereum the MerklePath in the counterparty MerklePrefix has the following structure: \["IBCCoreContractAddressStoragePrefix"\] The core IBC handler will append the ICS24 path to the final element of the MerklePath like so: \["IBCCoreContractAddressStoragePrefix{packetCommitmentPath}"\] which will then be used for final verification. Thus the MerklePath in the counterparty MerklePrefix is the nested key path from the root hash of the consensus state down to the ICS24 provable store. The IBC handler retrieves the counterparty key path to the ICS24 provable store from the MerklePath and appends the ICS24 path to get the final key path to the value being verified by the client against the root hash in the client's consensus state."""
    key_path: List[str] = field(default_factory=list)
    TYPE_URL: str = field(default="/ibc.core.commitment.v2.MerklePath", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "MerklePath":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

