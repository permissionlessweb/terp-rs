# Auto-generated from cosmos.ics23.v1 — do not edit.
# Source: terp-rs/proto/src/gen/cosmos.ics23.v1.rs
# Package: cosmos.ics23.v1
# Run `just py-gen` to regenerate.
from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from typing import Any, List, Optional

PACKAGE = "cosmos.ics23.v1"

@dataclass
class ExistenceProof:
    """*  ExistenceProof takes a key and a value and a set of steps to perform on it. The result of peforming all these steps will provide a "root hash", which can be compared to the value in a header.  Since it is computationally infeasible to produce a hash collission for any of the used cryptographic hash functions, if someone can provide a series of operations to transform a given key and value into a root hash that matches some trusted root, these key and values must be in the referenced merkle tree.  The only possible issue is maliablity in LeafOp, such as providing extra prefix data, which should be controlled by a spec. Eg. with lengthOp as NONE, prefix = FOO, key = BAR, value = CHOICE and prefix = F, key = OOBAR, value = CHOICE would produce the same value.  With LengthOp this is tricker but not impossible. Which is why the "leafPrefixEqual" field in the ProofSpec is valuable to prevent this mutability. And why all trees should length-prefix the data before hashing it."""
    key: str = ""
    value: str = ""
    leaf: Optional[LeafOp] = None
    path: List[InnerOp] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.ics23.v1.ExistenceProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ExistenceProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class NonExistenceProof:
    """NonExistenceProof takes a proof of two neighbors, one left of the desired key, one right of the desired key. If both proofs are valid AND they are neighbors, then there is no valid proof for the given key."""
    # TODO: remove this as unnecessary??? we prove a range
    key: str = ""
    left: Optional[ExistenceProof] = None
    right: Optional[ExistenceProof] = None
    TYPE_URL: str = field(default="/cosmos.ics23.v1.NonExistenceProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "NonExistenceProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CommitmentProof:
    """CommitmentProof is either an ExistenceProof or a NonExistenceProof, or a Batch of such messages"""
    proof: Any = None
    TYPE_URL: str = field(default="/cosmos.ics23.v1.CommitmentProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CommitmentProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class LeafOp:
    """*  LeafOp represents the raw key-value data we wish to prove, and must be flexible to represent the internal transformation from the original key-value pairs into the basis hash, for many existing merkle trees.  key and value are passed in. So that the signature of this operation is: leafOp(key, value) -> output  To process this, first prehash the keys and values if needed (ANY means no hash in this case): hkey = prehashKey(key) hvalue = prehashValue(value)  Then combine the bytes, and hash it output = hash(prefix || length(hkey) || hkey || length(hvalue) || hvalue)"""
    hash: int = 0
    prehash_key: int = 0
    prehash_value: int = 0
    length: int = 0
    # prefix is a fixed bytes that may optionally be included at the beginning to differentiate a leaf node from an inner node.
    prefix: str = ""
    TYPE_URL: str = field(default="/cosmos.ics23.v1.LeafOp", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "LeafOp":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class InnerOp:
    """*  InnerOp represents a merkle-proof step that is not a leaf. It represents concatenating two children and hashing them to provide the next result.  The result of the previous step is passed in, so the signature of this op is: innerOp(child) -> output  The result of applying InnerOp should be: output = op.hash(op.prefix || child || op.suffix)  where the || operator is concatenation of binary data, and child is the result of hashing all the tree below this step.  Any special data, like prepending child with the length, or prepending the entire operation with some value to differentiate from leaf nodes, should be included in prefix and suffix. If either of prefix or suffix is empty, we just treat it as an empty string"""
    hash: int = 0
    prefix: str = ""
    suffix: str = ""
    TYPE_URL: str = field(default="/cosmos.ics23.v1.InnerOp", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InnerOp":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class ProofSpec:
    """*  ProofSpec defines what the expected parameters are for a given proof type. This can be stored in the client and used to validate any incoming proofs.  verify(ProofSpec, Proof) -> Proof | Error  As demonstrated in tests, if we don't fix the algorithm used to calculate the LeafHash for a given tree, there are many possible key-value pairs that can generate a given hash (by interpretting the preimage differently). We need this for proper security, requires client knows a priori what tree format server uses. But not in code, rather a configuration object."""
    # any field in the ExistenceProof must be the same as in this spec. except Prefix, which is just the first bytes of prefix (spec can be longer)
    leaf_spec: Optional[LeafOp] = None
    inner_spec: Optional[InnerSpec] = None
    # max_depth (if > 0) is the maximum number of InnerOps allowed (mainly for fixed-depth tries) the max_depth is interpreted as 128 if set to 0
    max_depth: int = 0
    # min_depth (if > 0) is the minimum number of InnerOps allowed (mainly for fixed-depth tries)
    min_depth: int = 0
    # prehash_key_before_comparison is a flag that indicates whether to use the prehash_key specified by LeafOp to compare lexical ordering of keys for non-existence proofs.
    prehash_key_before_comparison: bool = False
    TYPE_URL: str = field(default="/cosmos.ics23.v1.ProofSpec", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "ProofSpec":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class InnerSpec:
    """InnerSpec contains all store-specific structure info to determine if two proofs from a given store are neighbors.  This enables:  isLeftMost(spec: InnerSpec, op: InnerOp) isRightMost(spec: InnerSpec, op: InnerOp) isLeftNeighbor(spec: InnerSpec, left: InnerOp, right: InnerOp)"""
    # Child order is the ordering of the children node, must count from 0 iavl tree is \[0, 1\] (left then right) merk is \[0, 2, 1\] (left, right, here)
    child_order: List[int] = field(default_factory=list)
    child_size: int = 0
    min_prefix_length: int = 0
    # the max prefix length must be less than the minimum prefix length + child size
    max_prefix_length: int = 0
    # empty child is the prehash image that is used when one child is nil (eg. 20 bytes of 0)
    empty_child: str = ""
    # hash is the algorithm that must be used for each InnerOp
    hash: int = 0
    TYPE_URL: str = field(default="/cosmos.ics23.v1.InnerSpec", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "InnerSpec":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class BatchProof:
    """BatchProof is a group of multiple proof types than can be compressed"""
    entries: List[BatchEntry] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.ics23.v1.BatchProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "BatchProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class BatchEntry:
    """Use BatchEntry not CommitmentProof, to avoid recursion"""
    proof: Any = None
    TYPE_URL: str = field(default="/cosmos.ics23.v1.BatchEntry", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "BatchEntry":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CompressedBatchProof:
    entries: List[CompressedBatchEntry] = field(default_factory=list)
    lookup_inners: List[InnerOp] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.ics23.v1.CompressedBatchProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CompressedBatchProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CompressedBatchEntry:
    """Use BatchEntry not CommitmentProof, to avoid recursion"""
    proof: Any = None
    TYPE_URL: str = field(default="/cosmos.ics23.v1.CompressedBatchEntry", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CompressedBatchEntry":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CompressedExistenceProof:
    key: str = ""
    value: str = ""
    leaf: Optional[LeafOp] = None
    # these are indexes into the lookup_inners table in CompressedBatchProof
    path: List[int] = field(default_factory=list)
    TYPE_URL: str = field(default="/cosmos.ics23.v1.CompressedExistenceProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CompressedExistenceProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

@dataclass
class CompressedNonExistenceProof:
    # TODO: remove this as unnecessary??? we prove a range
    key: str = ""
    left: Optional[CompressedExistenceProof] = None
    right: Optional[CompressedExistenceProof] = None
    TYPE_URL: str = field(default="/cosmos.ics23.v1.CompressedNonExistenceProof", init=False, repr=False)

    def encode(self) -> bytes:
        """Encode to protobuf binary (requires native extension)."""
        from terp_proto._native import encode_message  # type: ignore[import]
        return encode_message(self.TYPE_URL, json.dumps(asdict(self)).encode())

    @classmethod
    def decode(cls, data: bytes) -> "CompressedNonExistenceProof":
        """Decode from protobuf binary (requires native extension)."""
        from terp_proto._native import decode_message  # type: ignore[import]
        return cls(**json.loads(decode_message(cls.TYPE_URL, data)))

    def to_dict(self) -> dict:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(asdict(self))

