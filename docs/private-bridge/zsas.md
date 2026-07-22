ZIP: 227
Title: Issuance of Zcash Shielded Assets
Owners: Pablo Kogan <pablo@qed-it.com>
        Vivek Arte <vivek@qed-it.com>
        Daira-Emma Hopwood <daira@jacaranda.org>
        Jack Grigg <thestr4d@gmail.com>
Credits: Daniel Benarroch
         Aurelien Nicolas
         Deirdre Connolly
         Teor
Status: Draft
Category: Consensus
Created: 2022-05-01
License: MIT
Discussions-To: <https://github.com/zcash/zips/issues/618>
Pull-Request: <https://github.com/zcash/zips/pull/680>
Terminology 
The key words "MUST", "MUST NOT", "SHOULD", "RECOMMENDED", and "MAY" in this document are to be interpreted as described in BCP 14 1 when, and only when, they appear in all capitals.

The term "network upgrade" in this document is to be interpreted as described in ZIP 200 7.

The character § is used when referring to sections of the Zcash Protocol Specification. 24

The terms "Orchard" and "Action" in this document are to be interpreted as described in ZIP 224 9.

We define the following additional terms:

Asset: A type of note that can be transferred on the Zcash blockchain. Each Asset is identified by an Asset Identifier Specification: Asset Identifier, Asset Digest, and Asset Base.
ZEC is the default (and currently the only defined) Asset for the Zcash mainnet.
TAZ is the default (and currently the only defined) Asset for the Zcash testnet.
We use the term "Custom Asset" to refer to any Asset other than ZEC and TAZ.
Issuance Action: an instance of a single issuance of a Zcash Shielded Asset. It defines the issuance of a single Asset Identifier.
Issuance Bundle: the bundle in the transaction that contains all the issuance actions of that transaction.
Abstract 
This ZIP (ZIP 227) proposes the Orchard Zcash Shielded Assets (OrchardZSA) protocol, in conjunction with ZIP 226 10. This protocol is an extension of the Orchard protocol that enables the creation, transfer and burn of Custom Assets on the Zcash chain. The creation of such Assets is defined in this ZIP (ZIP 227), while the transfer and burn of such Assets is defined in ZIP 226 10. This ZIP must only be implemented in conjunction with ZIP 226 10. The proposed issuance mechanism is only valid for the OrchardZSA transfer protocol, because it produces notes that can only be transferred via this protocol.

Motivation 
This ZIP introduces the issuance mechanism for Custom Assets on the Zcash chain. While originally part of a single ZSA ZIP, the issuance mechanism turned out to be substantial enough to stand on its own and justify the creation of this supporting ZIP for ZIP 226 10.

This ZIP only enables transparent issuance. As a first step, transparency will allow for proper testing of the applications that will be most used in the Zcash ecosystem, and will enable the supply of Assets to be tracked.

The issuance mechanism described in this ZIP is broad enough for issuers to either create Assets on Zcash (i.e. Assets that originate on the Zcash blockchain), as well as for institutions to create bridges from other chains and issue Assets that wrap tokens from those chains. This enables what we hope will be a useful set of applications.

Use Cases 
The design presented in this ZIP enables issuance of shielded Assets in various modes:

The issuer may or may not know the receivers of the issued Asset in advance.
The Asset can be of non-fungible type, where each Asset type can be made part of a single “series”.
The supply of the Asset can be limited in advance or not.
The Asset can be a wrapped version of an Asset issued by another chain (as long as there is a bridge that supports transfer of that Asset between chains).
See the Concrete Applications section for more details.

Requirements 
Any user of the Zcash blockchain can issue Custom Assets on chain.
The issuance mechanism should enable public tracking of the supply of the Assets on the Zcash blockchain.
Issuing or changing the attributes of a specific Asset should require cryptographic authorization.
The Asset identification should be unique (among all shielded pools) and different issuer public keys should not be able to generate the same Asset Identifier.
An issuer should be able to issue different Assets in the same transaction. In other words, in a single "issuance bundle", the issuer should be able publish many "issuance actions", potentially creating multiple Custom Assets.
Every "issuance action" should contain a 
f
i
n
a
l
i
z
e
finalize boolean that defines whether the specific Custom Asset can have further tokens issued or not.
Specification: Issuance Keys, Issuer Identifier, and Issuance Authorization Signature Scheme 
Issuance Keys 
The OrchardZSA Protocol adds the following keys used for issuance:

The issuance authorizing key, denoted as 
i
s
k
,
isk, is the key used to authorize the issuance of Asset Identifiers by a given issuer, and is only used by that issuer.
The issuance validating key, denoted as 
i
k
,
ik, is the key that is used to validate issuance transactions. This key is used to validate the issuance of Asset Identifiers by a given issuer.
The relations between these keys are shown in the following diagram:


Diagram of Issuance Key Components for the OrchardZSA Protocol
Issuer Identifier 
The identifier for a particular issuer is denoted by 
i
s
s
u
e
r
.
issuer. This identifier is used by all blockchain users (specifically the owners of notes for that Asset, and consensus validators) to associate the Asset in question with the issuer.

i
s
s
u
e
r
issuer is equal to the encoding 
i
k
_
e
n
c
o
d
i
n
g
ik_encoding (specified below in Derivation of issuance validating key) of the issuance validating key 
i
k
ik used to validate the issuance of Asset Identifiers by that issuer.

Note: the equality of 
i
s
s
u
e
r
issuer and 
i
k
_
e
n
c
o
d
i
n
g
ik_encoding might not hold in future when key rotation is specified for issuance key pairs.

Issuance Authorization Signature Scheme 
The issuance authorization signature, encoded in the issueAuthSig field of an issuance bundle, is used to authorize the issuance of Custom Assets by the issuer.

The issuance authorization signature scheme, 
I
s
s
u
e
A
u
t
h
S
i
g
,
IssueAuthSig, comprises all the associated types and algorithms required for a signature scheme in §4.1.7 ‘Signature’ 27.

Batch verification MAY be used. Precomputation MAY be used if and only if it produces equivalent results.

Orchard ZSA Issuance Authorization Signature Scheme 
In the OrchardZSA protocol, we instantiate the issuance authorization signature scheme 
I
s
s
u
e
A
u
t
h
S
i
g
IssueAuthSig as a BIP-340 Schnorr signature over the secp256k1 curve. We define the constants as per the secp256k1 standard parameters, as described in BIP 340.

The associated types of the 
I
s
s
u
e
A
u
t
h
S
i
g
IssueAuthSig signature scheme, which are identical to BIP 340, are as follows:

I
s
s
u
e
A
u
t
h
S
i
g
.
M
e
s
s
a
g
e
=
B
Y
[
N
]
IssueAuthSig.Message=B 
Y 
[N]
 
 
I
s
s
u
e
A
u
t
h
S
i
g
.
S
i
g
n
a
t
u
r
e
=
B
Y
[
64
]
∪
{
⊥
}
IssueAuthSig.Signature=B 
Y 
[64]
 
 ∪{⊥}
I
s
s
u
e
A
u
t
h
S
i
g
.
P
u
b
l
i
c
=
B
Y
[
32
]
∪
{
⊥
}
IssueAuthSig.Public=B 
Y 
[32]
 
 ∪{⊥}
I
s
s
u
e
A
u
t
h
S
i
g
.
P
r
i
v
a
t
e
=
B
Y
[
32
]
IssueAuthSig.Private=B 
Y 
[32]
 
 
where 
B
Y
[
k
]
B 
Y 
[k]
 
  denotes the set of sequences of 
k
k bytes, and 
B
Y
[
N
]
B 
Y 
[N]
 
  denotes the type of byte sequences of arbitrary length, as defined in the Zcash protocol specification 25.

The issuance authorizing key generation algorithm and the issuance validating key derivation algorithm are defined in the Issuance Key Derivation section, while the corresponding signing and validation algorithms are defined in the Issuance Authorization Signing and Validation section.

Issuance Key Derivation 
Issuance authorizing key generation for hierarchical deterministic wallets 
The issuance authorizing key is generated using the Hardened-only key derivation process defined in ZIP 32 3. For the 
I
s
s
u
a
n
c
e
Issuance context, we define the following constants:

I
s
s
u
a
n
c
e
.
M
K
G
D
o
m
a
i
n
:
=
“ZcashSA_Issue_V1”
Issuance.MKGDomain:=“ZcashSA_Issue_V1”
I
s
s
u
a
n
c
e
.
C
K
D
D
o
m
a
i
n
:
=
0
x
81
Issuance.CKDDomain:=0x81
Let 
S
S be a seed byte sequence of a chosen length, which MUST be at least 32 and at most 252 bytes. We define the master extended issuance key 
m
I
s
s
u
a
n
c
e
:
=
M
K
G
h
I
s
s
u
a
n
c
e
(
S
)
.
m 
Issuance
​
 :=MKGh 
Issuance
 (S).

We use hardened-only child key derivation as defined in ZIP 32 4 for the issuance authorizing key.

C
K
D
s
k
(
(
s
k
p
a
r
,
c
p
a
r
)
,
i
)
→
(
s
k
i
,
c
i
)
CKDsk((sk 
par
​
 ,c 
par
​
 ),i)→(sk 
i
​
 ,c 
i
​
 ) :

Return 
C
K
D
h
I
s
s
u
a
n
c
e
(
(
s
k
p
a
r
,
c
p
a
r
)
,
i
)
CKDh 
Issuance
 ((sk 
par
​
 ,c 
par
​
 ),i)
We use the notation of ZIP 32 6 for shielded HD paths, and define the issuance authorizing key path as 
m
I
s
s
u
a
n
c
e
/
p
u
r
p
o
s
e
′
/
c
o
i
n
_
t
y
p
e
′
/
a
c
c
o
u
n
t
′
.
m 
Issuance
​
 /purpose 
′
 /coin_type 
′
 /account 
′
 . We fix the path levels as follows:

p
u
r
p
o
s
e
:
purpose: a constant set to 
227
227 (i.e. 
0
x
e
3
0xe3 ). 
p
u
r
p
o
s
e
′
purpose 
′
  is thus 
227
′
227 
′
  (or 
0
x
800000
e
3
0x800000e3 ) following the BIP 43 recommendation. 22
c
o
i
n
_
t
y
p
e
:
coin_type: Defined as in ZIP 32 5.
a
c
c
o
u
n
t
:
account: fixed to index 
0
.
0.
From the generated 
(
s
k
,
c
)
,
(sk,c), we set the issuance authorizing key to be 
i
s
k
:
=
s
k
.
isk:=sk.

Derivation of issuance validating key 
Define 
I
s
s
u
e
A
u
t
h
S
i
g
.
D
e
r
i
v
e
P
u
b
l
i
c
⦂
(
i
s
k
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
P
r
i
v
a
t
e
)
→
I
s
s
u
e
A
u
t
h
S
i
g
.
P
u
b
l
i
c
IssueAuthSig.DerivePublic⦂(isk⦂IssueAuthSig.Private)→IssueAuthSig.Public as:

i
k
:
=
PubKey
(
i
s
k
)
ik:=PubKey(isk)
Return 
⊥
⊥ if the 
PubKey
PubKey algorithm invocation fails, otherwise return 
i
k
.
ik.
where the 
PubKey
PubKey algorithm is defined in BIP 340 23, and the output of the algorithm is in big-endian order as defined in BIP 340.

The encoding of this validating key, 
i
k
_
e
n
c
o
d
i
n
g
,
ik_encoding, includes an initial byte indicating the signature scheme, which MUST be 
0
x
00
0x00 indicating BIP 340. That is, 
i
k
_
e
n
c
o
d
i
n
g
=
[
0
x
00
]
∣
∣
i
k
.
ik_encoding=[0x00]∣∣ik. This enables future ZIPs to specify alternative signature schemes. Note that this encoding currently only appears in the issuer field of an issuance bundle.

It is possible for the 
PubKey
PubKey algorithm to fail with very low probability, which means that 
I
s
s
u
e
A
u
t
h
S
i
g
.
D
e
r
i
v
e
P
u
b
l
i
c
IssueAuthSig.DerivePublic could return 
⊥
⊥ with very low probability. If this happens, discard the keys and repeat with a different 
i
s
k
.
isk.

This allows the issuer to use the same wallet it usually uses to transfer Assets, while keeping a disconnect from the other keys. Specifically, this method is aligned with the requirements and motivation of ZIP 32 2. It provides further anonymity and the ability to delegate issuance of an Asset (or in the future, use a multi-signature protocol) while the rest of the keys remain safe in the wallet.

Issuance Authorization Signing and Validation 
Define 
I
s
s
u
e
A
u
t
h
S
i
g
.
S
i
g
n
⦂
(
i
s
k
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
P
r
i
v
a
t
e
)
×
(
M
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
M
e
s
s
a
g
e
)
→
I
s
s
u
e
A
u
t
h
S
i
g
.
S
i
g
n
a
t
u
r
e
IssueAuthSig.Sign⦂(isk⦂IssueAuthSig.Private)×(M⦂IssueAuthSig.Message)→IssueAuthSig.Signature as:

Let the auxiliary data 
a
=
[
0
x
00
]
32
.
a=[0x00] 
32
 .
Let 
σ
=
S
i
g
n
(
i
s
k
,
M
)
σ=Sign(isk,M) with auxiliary data 
a
.
a.
Return 
⊥
⊥ if the 
S
i
g
n
Sign algorithm fails in the previous step, otherwise return 
σ
.
σ.
where the 
S
i
g
n
Sign algorithm is defined in BIP 340 23. Note that 
I
s
s
u
e
A
u
t
h
S
i
g
.
S
i
g
n
IssueAuthSig.Sign could return 
⊥
⊥ with very low probability.

Define 
I
s
s
u
e
A
u
t
h
S
i
g
.
V
a
l
i
d
a
t
e
⦂
(
i
k
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
P
u
b
l
i
c
)
×
(
M
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
M
e
s
s
a
g
e
)
×
(
σ
⦂
I
s
s
u
e
A
u
t
h
S
i
g
.
S
i
g
n
a
t
u
r
e
)
→
B
IssueAuthSig.Validate⦂(ik⦂IssueAuthSig.Public)×(M⦂IssueAuthSig.Message)×(σ⦂IssueAuthSig.Signature)→B as:

Return 
0
0 if 
σ
=
⊥
.
σ=⊥.
Return 
1
1 if 
V
e
r
i
f
y
(
k
e
y
,
M
,
σ
)
Verify(key,M,σ) succeeds, otherwise 
0
.
0.
where the 
V
e
r
i
f
y
Verify algorithm is defined in BIP 340 23.

The 
i
s
s
u
e
A
u
t
h
S
i
g
issueAuthSig field of an issuance bundle encodes the signature with an initial byte indicating the signature scheme, which MUST be 
0
x
00
0x00 indicating BIP 340. That is, 
i
s
s
u
e
A
u
t
h
S
i
g
=
[
0
x
00
]
∣
∣
σ
.
issueAuthSig=[0x00]∣∣σ. This enables future ZIPs to specify alternative signature schemes.

Specification: Asset Identifier, Asset Digest, and Asset Base 
The definition of the Asset Identifier, and that of the Asset Digest and Asset Base for a given Asset Identifier, will be described in this section. For context, the relations between the Asset Identifier, Asset Digest, and Asset Base are shown in the following diagram:


Diagram relating the Asset Identifier, Asset Digest, and Asset Base.
Note: To keep notations light and concise, we may omit 
A
s
s
e
t
I
d
AssetId in the subscript when the Asset Identifier is clear from the context.

Asset Identifier 
Every Asset has a globally-unique Asset Identifier, denoted 
A
s
s
e
t
I
d
.
AssetId. A given Asset Identifier is used across all Zcash protocols that support ZSAs -- that is, the OrchardZSA protocol and potentially future Zcash shielded protocols.

ZIP 227 Asset Identifiers 
Assets issued using the protocol specified in this ZIP are scoped to the 
i
s
s
u
e
r
issuer that issued them. Within that scope, Asset Identifier uniqueness is obtained by way of an asset description, 
a
s
s
e
t
_
d
e
s
c
,
asset_desc, which includes any information pertaining to the issuance. 
a
s
s
e
t
_
d
e
s
c
asset_desc is a non-empty byte sequence which SHOULD be a well-formed UTF-8 code unit sequence according to Unicode 15.0.0 or later.

Define

a
s
s
e
t
D
e
s
c
H
a
s
h
:
=
BLAKE2b-256
(
“ZSA-AssetDescCRH”
,
a
s
s
e
t
_
d
e
s
c
)
,
assetDescHash:=BLAKE2b-256(“ZSA-AssetDescCRH”,asset_desc),
We define Asset Identifiers for Assets issued under this ZIP as

A
s
s
e
t
I
d
:
=
(
i
s
s
u
e
r
,
a
s
s
e
t
D
e
s
c
H
a
s
h
)
AssetId:=(issuer,assetDescHash)
and define their canonical encoding as

E
n
c
o
d
e
A
s
s
e
t
I
d
(
A
s
s
e
t
I
d
)
=
E
n
c
o
d
e
A
s
s
e
t
I
d
(
(
i
s
s
u
e
r
,
a
s
s
e
t
D
e
s
c
H
a
s
h
)
)
:
=
[
0
x
00
]
∣
∣
i
s
s
u
e
r
∣
∣
a
s
s
e
t
D
e
s
c
H
a
s
h
EncodeAssetId(AssetId)=EncodeAssetId((issuer,assetDescHash)):=[0x00]∣∣issuer∣∣assetDescHash
Note that the initial 
0
x
00
0x00 byte is a version byte, enabling future ZIPs to specify alternative issuance protocols and Asset Identifiers. (This should not be confused with the first byte of 
i
s
s
u
e
r
,
issuer, currently equal to the first byte of 
i
k
_
e
n
c
o
d
i
n
g
,
ik_encoding, that indicates the issuance signature scheme.)

Wallets MUST NOT display just the 
a
s
s
e
t
_
d
e
s
c
asset_desc string to their users as the name of the Asset. Some possible alternatives include:

Wallets could allow clients to provide an additional configuration file that stores a one-to-one mapping of names to Asset Identifiers via a petname system 35. This allows clients to rename the Assets in a way they find useful. Default versions of this file with well-known Assets listed can be made available online as a starting point for clients.
The Asset Digest could be used as a more compact byte sequence to uniquely determine an Asset, and wallets could support clients scanning QR codes to load Asset information into their wallets.
Asset Digests 
From the Asset Identifier, we derive an Asset Digest

A
s
s
e
t
D
i
g
e
s
t
A
s
s
e
t
I
d
:
=
BLAKE2b-512
(
“ZSA-Asset-Digest”
,
E
n
c
o
d
e
A
s
s
e
t
I
d
(
A
s
s
e
t
I
d
)
)
,
AssetDigest 
AssetId
​
 :=BLAKE2b-512(“ZSA-Asset-Digest”,EncodeAssetId(AssetId)),
where 
E
n
c
o
d
e
A
s
s
e
t
I
d
(
A
s
s
e
t
I
d
)
EncodeAssetId(AssetId) is the canonical encoding scheme for the Asset Identifier.

Asset Bases 
From the Asset Digest, we derive a specific Asset Base that represents the Custom Asset within each shielded protocol:

A
s
s
e
t
B
a
s
e
A
s
s
e
t
I
d
:
=
Z
S
A
V
a
l
u
e
B
a
s
e
(
A
s
s
e
t
D
i
g
e
s
t
A
s
s
e
t
I
d
)
AssetBase 
AssetId
​
 :=ZSAValueBase(AssetDigest 
AssetId
​
 )
This Asset Base is included in shielded notes within the shielded protocol.

OrchardZSA Asset Bases 
In the case of the OrchardZSA protocol, we define

Z
S
A
V
a
l
u
e
B
a
s
e
(
A
s
s
e
t
D
i
g
e
s
t
)
:
=
G
r
o
u
p
H
a
s
h
P
(
"z.cash:OrchardZSA"
,
A
s
s
e
t
D
i
g
e
s
t
)
ZSAValueBase(AssetDigest):=GroupHash 
P
 ("z.cash:OrchardZSA",AssetDigest)
where 
G
r
o
u
p
H
a
s
h
P
GroupHash 
P
  is defined as in 32.


Diagram relating the Issuer identifier, asset description, asset description hash, Asset Identifier, Asset Digest, and Asset Base for the OrchardZSA Protocol.
Specification: Issue Note, Issuance Action, Issuance Bundle and Issuance Protocol 
Issue Note 
Let 
ℓ
v
a
l
u
e
ℓ 
value
​
  be as defined in §5.3 ‘Constants’ 30. An Issue Note represents that a value 
v
:
{
0..2
ℓ
v
a
l
u
e
−
1
}
v:{0..2 
ℓ 
value
​
 
 −1} of a specific Custom Asset is issued to a recipient. An Issue Note is a tuple 
(
d
,
p
k
d
,
v
,
A
s
s
e
t
B
a
s
e
,
ρ
,
r
s
e
e
d
)
,
(d,pk 
d
​
 ,v,AssetBase,ρ,rseed), where:

d
:
B
[
ℓ
d
]
d:B 
[ℓ 
d
​
 ]
  is the diversifier of the recipient's shielded payment address, as in §3.2 ‘Notes’ 26.
p
k
d
:
K
A
O
r
c
h
a
r
d
.
P
u
b
l
i
c
pk 
d
​
 :KA 
Orchard
 .Public is the recipient's diversified transmission key, as in §3.2 ‘Notes’ 26.
v
:
{
0..2
ℓ
v
a
l
u
e
−
1
}
v:{0..2 
ℓ 
value
​
 
 −1} is the value of the note in terms of the number of Asset tokens.
A
s
s
e
t
B
a
s
e
:
P
∗
AssetBase:P 
∗
  is the Asset Base corresponding to the ZSA being issued in the Issue Note.
ρ
:
F
q
P
ρ:F 
q 
P
​
 
​
  is used to derive the nullifier of the note, and is computed as in Computation of ρ.
r
s
e
e
d
:
B
[
Y
32
]
rseed:B 
[Y 
32
 ]
  MUST be sampled uniformly at random by the issuer.
ZIP 230 16 defines, in IssueNoteDescription, field encodings which together with 
i
s
s
u
e
r
issuer from the parent Issuance Bundle and 
A
s
s
e
t
D
e
s
c
H
a
s
h
AssetDescHash from the parent Issuance Action, specify an Issue Note.

Let 
N
o
t
e
I
s
s
u
e
Note 
Issue
  be the type of an Issue Note, i.e.

N
o
t
e
I
s
s
u
e
:
=
B
[
ℓ
d
]
×
K
A
O
r
c
h
a
r
d
.
P
u
b
l
i
c
×
{
0..2
ℓ
v
a
l
u
e
−
1
}
×
P
∗
×
F
q
P
×
B
[
Y
32
]
.
Note 
Issue
 :=B 
[ℓ 
d
​
 ]
 ×KA 
Orchard
 .Public×{0..2 
ℓ 
value
​
 
 −1}×P 
∗
 ×F 
q 
P
​
 
​
 ×B 
[Y 
32
 ]
 .
The note commitments of Issue Notes are computed in the same manner as for OrchardZSA Notes. They will be added to the note commitment tree as any other shielded note when the transaction issuing the Asset is included on chain. This prevents future usage of the note from being linked to the issuance transaction, as the nullifier key is not known to the validators and chain observers.

Issuance Action 
An issuance action, IssueAction, is the instance of issuing a specific Custom Asset, and contains the following fields:

assetDescHash: the hash of the Asset description, as defined in the ZIP 227 Asset Identifiers section.
vNotes: an array of Issue Notes containing the unencrypted output notes to the recipients of the Asset.
flagsIssuance: a byte that stores the 
f
i
n
a
l
i
z
e
finalize boolean that defines whether the issuance of that specific Custom Asset is finalized or not.
The 
f
i
n
a
l
i
z
e
finalize boolean is set by the Issuer to signal that there will be no further issuance of the specific Custom Asset. As we will see in Specification: Consensus Rule Changes, transactions that attempt to issue further amounts of a Custom Asset that has previously been finalized will be rejected.

The complete encoding of these fields into an IssueAction is defined in ZIP 230 15.

Issuance Bundle 
An issuance bundle is the aggregate of all the issuance-related information. Specifically, contains all the issuance actions and the issuer signature on the transaction SIGHASH that validates the issuance itself. It contains the following fields:

issuer: the issuer identifier, that allows the validators to verify that the 
A
s
s
e
t
I
d
AssetId is properly associated with the issuer.
vIssueActions: an array of issuance actions, of type IssueAction.
issueAuthSig: the encoding of a signature of the transaction SIGHASH, signed by the issuance authorizing key, 
i
s
k
,
isk, that validates the issuance.
The issuance bundle is added within the transaction format as a new bundle. The detailed encoding of the issuance bundle as a part of the V6 transaction format is defined in ZIP 230 17.

Computation of ρ 
We define a function 
D
e
r
i
v
e
I
s
s
u
e
d
R
h
o
:
F
q
P
×
{
0..2
32
−
1
}
×
{
0..2
32
−
1
}
→
F
q
P
DeriveIssuedRho:F 
q 
P
​
 
​
 ×{0..2 
32
 −1}×{0..2 
32
 −1}→F 
q 
P
​
 
​
  for Issue Notes in the OrchardZSA Protocol as follows:

D
e
r
i
v
e
I
s
s
u
e
d
R
h
o
(
n
f
,
i
A
,
i
N
)
:
=
T
o
B
a
s
e
O
r
c
h
a
r
d
(
P
R
F
e
x
p
a
n
d
(
I
2
L
E
O
S
P
256
(
n
f
)
,
[
0
x
84
]
∣
∣
I
2
L
E
O
S
P
32
(
i
A
)
∣
∣
I
2
L
E
O
S
P
32
(
i
N
)
)
)
,
DeriveIssuedRho(nf,i 
A
​
 ,i 
N
​
 ):=ToBase 
Orchard
 (PRF 
expand
 (I2LEOSP 
256
​
 (nf),[0x84]∣∣I2LEOSP 
32
​
 (i 
A
​
 )∣∣I2LEOSP 
32
​
 (i 
N
​
 ))),
where 
T
o
B
a
s
e
O
r
c
h
a
r
d
ToBase 
Orchard
  is defined in §4.2.3 ‘Orchard Key Components’ 28, and 
P
R
F
e
x
p
a
n
d
PRF 
expand
  is defined in §5.4.2 ‘Pseudo Random Functions’ 31.

The 
ρ
ρ field of an Issue Note is computed as

ρ
:
=
D
e
r
i
v
e
I
s
s
u
e
d
R
h
o
(
n
f
0
,
0
,
i
n
d
e
x
A
c
t
i
o
n
,
i
n
d
e
x
N
o
t
e
)
,
ρ:=DeriveIssuedRho(nf 
0,0
​
 ,index 
Action
​
 ,index 
Note
​
 ),
where 
n
f
0
,
0
nf 
0,0
​
  is the nullifier for the input note in the first Action in the first Action Group of the OrchardZSA Bundle of the transaction, 
i
n
d
e
x
A
c
t
i
o
n
index 
Action
​
  is the zero-based index of the Issuance Action in the Issuance Bundle, and 
i
n
d
e
x
N
o
t
e
index 
Note
​
  is the zero-based index of the Issue Note in the Issuance Action.

NOTE: This implicitly requires that there always is an Action Group in the OrchardZSA bundle of the transaction. This is enforced by a consensus rule in the Specification: Consensus Rule Changes section.

Issuance Protocol 
The issuer program performs the following operations:

For all actions IssueAction:

encode 
a
s
s
e
t
_
d
e
s
c
asset_desc as a UTF-8 byte string.
compute 
a
s
s
e
t
D
e
s
c
H
a
s
h
assetDescHash
compute 
A
s
s
e
t
D
i
g
e
s
t
AssetDigest from the issuer identifier 
i
s
s
u
e
r
issuer and 
a
s
s
e
t
D
e
s
c
H
a
s
h
assetDescHash as decribed in the Specification: Asset Identifier, Asset Digest, and Asset Base section.
compute 
A
s
s
e
t
B
a
s
e
AssetBase from 
A
s
s
e
t
D
i
g
e
s
t
AssetDigest as decribed in the Specification: Asset Identifier, Asset Digest, and Asset Base section.
set the 
f
i
n
a
l
i
z
e
finalize boolean as desired (if more issuance actions are to be created for this 
A
s
s
e
t
B
a
s
e
,
AssetBase, set 
f
i
n
a
l
i
z
e
=
0
,
finalize=0, otherwise set 
f
i
n
a
l
i
z
e
=
1
finalize=1 ).
for each recipient 
i
:
i:
generate an Issue Note, 
n
o
t
e
i
=
(
d
i
,
p
k
d
i
,
v
i
,
A
s
s
e
t
B
a
s
e
,
ρ
i
,
r
s
e
e
d
i
)
.
note 
i
​
 =(d 
i
​
 ,pk 
d 
i
​
 
​
 ,v 
i
​
 ,AssetBase,ρ 
i
​
 ,rseed 
i
​
 ).
encode the 
n
o
t
e
i
note 
i
​
  into the vector vNotes of the IssueAction.
encode the IssueAction into the vector vIssueActions of the bundle.
For the IssueBundle (see “ZSA Issuance Bundle Fields” in 17):

encode the vIssueActions vector.
fill the issuerLength and issuer fields using 
i
s
s
u
e
r
.
issuer.
sign the SIGHASH transaction hash with the issuance authorizing key, 
i
s
k
,
isk, using the 
I
s
s
u
e
A
u
t
h
S
i
g
IssueAuthSig signature scheme. The signature is then added to the issuance bundle.
Note: The note commitment is not included in the IssuanceAction itself. As explained below, it is computed later by the validators and added to the note commitment tree.

Specification: Reference Notes and Global Issuance State 
Reference Notes 
A reference note for a Custom Asset MUST be included by the issuer as the first Note in the Action of the Issuance Bundle where that Custom Asset is being issued for the first time.

A reference note for a Custom Asset is an Issue Note where the value 
v
v is set to 
0
,
0, the Asset Base ( 
A
s
s
e
t
B
a
s
e
AssetBase ) corresponds to that of the Custom Asset, and the recipient address 
(
d
,
p
k
d
)
(d,pk 
d
​
 ) is set to the default diversified payment address (i.e. the diversified payment address with diversifier index 
0
0 ) derived from the all-zero Orchard spending key using the algorithm specified in §4.2.3 ‘Orchard Key Components’ 28. This corresponds to a 43-byte u8 array with the following entries:

[
  204, 54, 96, 25, 89, 33, 59, 107, 12, 219, 150, 167, 92, 23, 195, 166, 104, 169, 127, 13, 106,
  140, 92, 225, 100, 165, 24, 234, 155, 169, 165, 14, 167, 81, 145, 253, 134, 27, 15, 241, 14,
  98, 176,
]
Global Issuance State 
The maximum total supply of any issued Custom Asset is denoted by the constant 
M
A
X
_
I
S
S
U
E
:
=
2
64
−
1
.
MAX_ISSUE:=2 
64
 −1.

Issuance requires the following additions to the global state:

A map, 
i
s
s
u
e
d
_
a
s
s
e
t
s
:
P
∗
→
{
0..
M
A
X
_
I
S
S
U
E
}
×
B
×
N
o
t
e
I
s
s
u
e
,
issued_assets:P 
∗
 →{0..MAX_ISSUE}×B×Note 
Issue
 , from the Asset Base, 
A
s
s
e
t
B
a
s
e
:
P
∗
,
AssetBase:P 
∗
 , to a tuple 
(
b
a
l
a
n
c
e
,
f
i
n
a
l
,
n
o
t
e
r
e
f
)
,
(balance,final,note 
ref
​
 ), for every Asset that has been issued. We use the notation 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
,
issued_assets(AssetBase).balance, 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
,
issued_assets(AssetBase).final, and 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
n
o
t
e
r
e
f
issued_assets(AssetBase).note 
ref
​
  to access, respectively, the elements of the tuple stored in the global state for a given 
A
s
s
e
t
B
a
s
e
.
AssetBase. If 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
=
⊥
,
issued_assets(AssetBase)=⊥, it is assumed that 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
=
0
,
issued_assets(AssetBase).balance=0, 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
=
0
,
issued_assets(AssetBase).final=0, and 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
n
o
t
e
r
e
f
=
⊥
.
issued_assets(AssetBase).note 
ref
​
 =⊥.

For any Asset represented by 
A
s
s
e
t
B
a
s
e
:
AssetBase:

i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
∈
{
0..
M
A
X
_
I
S
S
U
E
}
issued_assets(AssetBase).balance∈{0..MAX_ISSUE} stores the amount of the Asset in circulation, computed as the amount of the Asset that has been issued less the amount of the Asset that has been burnt.
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
:
B
issued_assets(AssetBase).final:B is a Boolean that stores the finalization status of the Asset (i.e.: whether the 
f
i
n
a
l
i
z
e
finalize flag has been set to 
1
1 in any preceding issuance transaction for the Asset). The value of 
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
issued_assets(AssetBase).final for any 
A
s
s
e
t
B
a
s
e
AssetBase cannot be changed from 
1
1 to 
0
.
0.
i
s
s
u
e
d
_
a
s
s
e
t
s
(
A
s
s
e
t
B
a
s
e
)
.
n
o
t
e
r
e
f
:
N
o
t
e
I
s
s
u
e
issued_assets(AssetBase).note 
ref
​
 :Note 
Issue
  stores the reference note for the Asset, as defined in the Reference Notes section.
Management of the Global Issuance State 
The issuance state, that is, the 
i
s
s
u
e
d
_
a
s
s
e
t
s
issued_assets map, MUST be updated by a node during the processing of any transaction that contains burn information, or an issuance bundle. The issuance state is chained as follows:

The input issuance state for the activation block of the OrchardZSA protocol is the empty map.
The input issuance state for the first transaction of a block is the final issuance state of the immediately preceding block.
The input issuance state of each subsequent transaction in the block is the output issuance state of the immediately preceding transaction.
The final issuance state of a block is the output issuance state of the last transaction in the block.
We describe the consensus rule changes that govern the management of the global issuance state in the Specification: Consensus Rule Changes section. We use 
i
s
s
u
e
d
_
a
s
s
e
t
s
I
N
issued_assets 
IN
​
  and 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
issued_assets 
OUT
​
  to denote the input issuance state and output issuance state for a transaction, respectively.

Specification: Consensus Rule Changes 
Let 
S
i
g
H
a
s
h
SigHash be the SIGHASH transaction hash of this transaction, as defined in §4.10 ‘SIGHASH Transaction Hashing’ 29 with the modifications described in ZIP 226 13, using 
S
I
G
H
A
S
H
_
A
L
L
.
SIGHASH_ALL.

For every transaction:

The nActionGroupsOrchard field MUST have a value of either 0 or 1 and the nAGExpiryHeight field MUST have a value of 0.
The output issuance state of the transaction MUST be initialized to be the same as the input issuance state, 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
=
i
s
s
u
e
d
_
a
s
s
e
t
s
I
N
.
issued_assets 
OUT
​
 =issued_assets 
IN
​
 .
The 
a
s
s
e
t
B
u
r
n
assetBurn set MUST satisfy the consensus rules specified in ZIP 226 12.
It MUST be the case that for all 
(
A
s
s
e
t
B
a
s
e
,
v
)
∈
a
s
s
e
t
B
u
r
n
,
(AssetBase,v)∈assetBurn, 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
≥
v
.
issued_assets 
OUT
​
 (AssetBase).balance≥v. The node then MUST update 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
issued_assets 
OUT
​
 (AssetBase) prior to processing the issuance bundle in the following manner. For every 
(
A
s
s
e
t
B
a
s
e
,
v
)
∈
A
s
s
e
t
B
u
r
n
,
(AssetBase,v)∈AssetBurn, 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
=
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
b
a
l
a
n
c
e
−
v
.
issued_assets 
OUT
​
 (AssetBase).balance=issued_assets 
OUT
​
 (AssetBase).balance−v.
If the transaction contains an issuance bundle:

It MUST also contain at least one OrchardZSA Action Group.
The encoding 
i
k
_
e
n
c
o
d
i
n
g
ik_encoding of the issuance key 
i
k
ik used to validate the issuance authorization signature MUST be taken from the issuer field, and MUST start with the byte value 
0
x
00
0x00 indicating a BIP 340 public key.
The issuance authorization signature, 
i
s
s
u
e
A
u
t
h
S
i
g
,
issueAuthSig, MUST be a valid 
I
s
s
u
e
A
u
t
h
S
i
g
IssueAuthSig signature over 
S
i
g
H
a
s
h
,
SigHash, i.e. 
I
s
s
u
e
A
u
t
h
S
i
g
.
V
a
l
i
d
a
t
e
(
i
k
,
S
i
g
H
a
s
h
,
i
s
s
u
e
A
u
t
h
S
i
g
)
=
1
.
IssueAuthSig.Validate(ik,SigHash,issueAuthSig)=1.
For every issuance action description ( 
I
s
s
u
e
A
c
t
i
o
n
i
,
 
1
≤
i
≤
n
I
s
s
u
e
A
c
t
i
o
n
s
IssueAction 
i
​
 , 1≤i≤nIssueActions ) in the issuance bundle:
Every IssueNoteDescription in the IssueAction MUST be a valid field encoding as defined in ZIP 230 16.
Let an Issue Note (with type 
N
o
t
e
I
s
s
u
e
Note 
Issue
  ) be constructed from the fields of each IssueNoteDescription, with the 
A
s
s
e
t
B
a
s
e
AssetBase derived from the assetDescHash field of the IssueAction and the issuer field of the issuance bundle, as described in the Specification: Asset Identifier, Asset Digest, and Asset Base section.
It MUST be the case that 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
≠
1
.
issued_assets 
OUT
​
 (AssetBase).final

=1.
If 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
n
o
t
e
r
e
f
=
⊥
,
issued_assets 
OUT
​
 (AssetBase).note 
ref
​
 =⊥, then let 
n
e
w
_
n
o
t
e
r
e
f
new_note 
ref
​
  be the first Issue Note in the Issuance Action.
The recipient address 
(
d
,
p
k
d
)
(d,pk 
d
​
 ) of 
n
e
w
_
n
o
t
e
r
e
f
new_note 
ref
​
  MUST be the default diversified payment address derived from the all-zero Orchard spending key, as described in the Reference Notes section.
The value of 
n
e
w
_
n
o
t
e
r
e
f
new_note 
ref
​
  MUST be 
0
.
0.
The node MUST update 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
n
o
t
e
r
e
f
=
n
e
w
_
n
o
t
e
r
e
f
.
issued_assets 
OUT
​
 (AssetBase).note 
ref
​
 =new_note 
ref
​
 .
For every Issue Note, 
n
o
t
e
j
note 
j
​
  for 
1
≤
j
≤
n
N
o
t
e
s
1≤j≤nNotes in IssueAction:
The 
ρ
ρ field of the issue note MUST have been computed as described in the Computation of ρ section.
It MUST be the case that 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
.
b
a
l
a
n
c
e
+
v
j
≤
M
A
X
_
I
S
S
U
E
,
issued_assets 
OUT
​
 .balance+v 
j
​
 ≤MAX_ISSUE, where 
v
j
v 
j
​
  is the value of 
n
o
t
e
j
.
note 
j
​
 . The node then MUST update 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
.
b
a
l
a
n
c
e
=
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
.
b
a
l
a
n
c
e
+
v
j
.
issued_assets 
OUT
​
 .balance=issued_assets 
OUT
​
 .balance+v 
j
​
 .
The node MUST compute the note commitment, 
c
m
i
,
j
,
cm 
i,j
​
 , as defined in the Note Structure and Commitment section of ZIP 226 11.
If 
f
i
n
a
l
i
z
e
=
1
finalize=1 within the flagsIssuance field of IssueAction, the node MUST set 
i
s
s
u
e
d
_
a
s
s
e
t
s
O
U
T
(
A
s
s
e
t
B
a
s
e
)
.
f
i
n
a
l
=
1
.
issued_assets 
OUT
​
 (AssetBase).final=1.
Addition to the Note Commitment Tree 
If the transaction is added to the block chain, the note commitments of all the OrchardZSA Notes and the Issue Notes in the transaction are added to the note commitment tree of the associated treestate. The order of addition to the tree is as specified below:

For each Action Group in the OrchardZSA Bundle:
For every Action in the Action Group, append the note commitment of every new OrchardZSA note in the Action to the note commitment tree.
For each Issue Action in the Issue Bundle:
For every Issue Note in the Issue Action, append the note commitment of the Issue Note to the note commitment tree.
Rationale 
The following is a list of rationale for different decisions made in the proposal:

The issuance key structure is independent of the original key tree, but derived in an analogous manner (via ZIP 32). This keeps the issuance details and the Asset Identifiers consistent across multiple shielded pools. It also separates the issuance authority from the spend authority, allowing for the potential transfer of issuance authority without compromising the spend authority.
The Custom Asset is described via a combination of the issuer identifier and an asset description string, to preclude the possibility of two different issuers creating colliding Custom Assets.
The requirement of at least one OrchardZSA Action Group in the presence of an issue bundle is both to allow for the computation of the 
ρ
ρ field of the issue notes, as well as to prevent replay attacks. If a transaction includes only an issue bundle, the SIGHASH transaction hash would be computed solely based on the issue bundle. A duplicate bundle would have the same SIGHASH transaction hash, potentially allowing for a replay attack.
Hash of the asset description 
In an earlier version of this ZIP, the asset description was a direct component of the Asset Identifier, and was stored on-chain in each issuance transaction. The Asset Identifier and issuance transactions now instead include a collision-resistant hash of the asset description, for the following reasons:

A hash output (32 bytes per Issue Action) incurs lower average bandwidth costs in issuance transactions than the asset description (previously up to 512 bytes).
The asset description can be longer than 512 bytes without incurring chain costs.
Including an asset description byte string directly in issuance transactions does not ensure that the "user-visible" asset description is consensus-visible, because the byte string could itself be a hash of another off-chain description (even if the consensus rules had required it to be a Unicode string instead of only recommending it).
The lack of key rotation in this issuance protocol means that it is not sufficient to mark an 
i
s
s
u
e
r
issuer as trusted and then accept whatever asset descriptions are issued by it. Each Asset Identifier needs to be independently verified, which requires some out-of-band protocol that can also convey the corresponding asset description.
If issuance transactions include the asset descriptions directly, wallets will discover them during scanning. This is an "attractive nuisance" because it would result in wallets being more likely to expose the asset description directly to users without any verification that the received asset has the value that a user might expect from that description. By instead using a collision-resistant hash of an asset description, wallets are forced to look up the corresponding asset description when a payment is received in an unknown asset. That lookup can be mediated by a trusted party or common trusted registry of known assets, or else will need to be approved directly by a user who can personally assert their interest in that specific asset.
Rationale for Global Issuance State 
It is necessary to ensure that the balance of any issued Custom Asset never becomes negative within a shielded pool, along the lines of ZIP 209 8. However, unlike for the shielded ZEC pools, there is no individual transaction field that directly corresponds to both the issued and burnt amounts for a given Asset. Therefore, we require that all nodes maintain a record of the current amount in circulation for every issued Custom Asset, and update this record based on the issuance and burn transactions processed. This allows for efficient detection of balance violations for any Asset, in which case we specify a consensus rule to reject the transaction or block.

We limit the total issuance of any Asset to a maximum of 
M
A
X
_
I
S
S
U
E
.
MAX_ISSUE. This is a practical limit that also allows an issuer to issue the complete supply of an Asset in a single transaction.

Nodes also need to reject transactions that issue Custom Assets that have been previously finalized. The 
i
s
s
u
e
d
_
a
s
s
e
t
s
issued_assets map allows nodes to store whether or not a given Asset has been finalized.

Concrete Applications 
Asset Features

By using the 
f
i
n
a
l
i
z
e
finalize boolean and the burning mechanism defined in 10, issuers can control the supply production of any Asset associated to their issuer keys. For example,
by setting 
f
i
n
a
l
i
z
e
=
1
finalize=1 from the first issuance action for that Asset Identifier, the issuer is in essence creating a one-time issuance transaction. This is useful when the max supply is capped from the beginning and the distribution is known in advance. All tokens are issued at once and distributed as needed.
Issuers can also stop the existing supply production of any Asset associated to their issuer keys. This could be done by
issuing a last set of tokens of that specific 
A
s
s
e
t
I
d
,
AssetId, for which 
f
i
n
a
l
i
z
e
=
1
,
finalize=1, or by
issuing a transaction with a single note in the issuance action pertaining to that 
A
s
s
e
t
I
d
,
AssetId, where the note will contain a 
v
a
l
u
e
=
0
.
value=0. This can be used for application-specific purposes (NFT collections) or for security purposes to revoke the Asset issuance (see Security and Privacy Considerations).
The issuance and burn mechanisms can be used in conjunction to determine the supply of Assets on the Zcash ecosystem. This allows for the bridging of Assets defined on other chains.
Furthermore, NFT issuance is enabled by issuing in a single bundle several issuance actions, where each 
A
s
s
e
t
I
d
AssetId corresponds to 
v
a
l
u
e
=
1
value=1 at the fundamental unit level. Issuers and users should make sure that 
f
i
n
a
l
i
z
e
=
1
finalize=1 for each of the actions in this scenario.
Modifications relative to ZIP 244 18
Relative to the sighash algorithm defined in ZIP 244, the sighash algorithm that applies to v6 transactions differs by including the issuance bundle components within the tree hash. See ZIP 246 19 for details.

Changes to ZIP 317 20
The conventional fee in ZEC is altered to take into account both the presence of issuance actions within a transaction, and the creation of new Custom Assets within the global chain state. See the Fee calculation section of ZIP 317 21 for details.

Rationale for paying fees in ZEC 
Click to show/hide
Security and Privacy Considerations 
Displaying Asset Identifier information to users 
Wallets need to communicate the names of the Assets in a non-confusing way to users, since the byte representation of the Asset Identifier would be hard to read for an end user. Possible solutions are provided in the Specification: Asset Identifier, Asset Digest, and Asset Base section.

Issuance Key Compromise 
The design of this protocol does not currently allow for rotation of the issuance validating key that would allow for replacing the key of a specific Asset. In case of compromise, the following actions are recommended:

If an issuer identifier is compromised, the 
f
i
n
a
l
i
z
e
finalize boolean for all the Assets issued with that key should be set to 
1
;
1; then the issuer should change to a new issuance authorizing key (hence a new issuer identifier), and issue new Assets, each with a new 
A
s
s
e
t
I
d
.
AssetId.
Bridging Assets 
For bridging purposes, the secure method of off-boarding Assets is to burn an Asset with the burning mechanism in ZIP 226 10. Users should be aware of issuers that demand the Assets be sent to a specific address on the Zcash chain to be redeemed elsewhere, as this may not reflect the real reserve value of the specific Asset.

Other Considerations 
Implementing Zcash Nodes 
Although not enforced in the global state, it is RECOMMENDED that Zcash full validators keep track of the total supply of Assets as a mutable mapping 
i
s
s
u
a
n
c
e
S
u
p
p
l
y
I
n
f
o
M
a
p
issuanceSupplyInfoMap from 
A
s
s
e
t
I
d
AssetId to 
(
t
o
t
a
l
S
u
p
p
l
y
,
f
i
n
a
l
i
z
e
)
(totalSupply,finalize) in order to properly keep track of the total supply for different Asset Identifiers. This is useful for wallets and other applications that need to keep track of the total supply of Assets.

Test Vectors 
LINK TBD
Reference Implementation 
LINK TBD
LINK TBD
Deployment 
TBD

References 
1	Information on BCP 14 — "RFC 2119: Key words for use in RFCs to Indicate Requirement Levels" and "RFC 8174: Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words"
2	ZIP 32: Shielded Hierarchical Deterministic Wallets
3	ZIP 32: Shielded Hierarchical Deterministic Wallets — Specification: Hardened-only key derivation
4	ZIP 32: Shielded Hierarchical Deterministic Wallets — Hardened-only child key derivation
5	ZIP 32: Shielded Hierarchical Deterministic Wallets — Key path levels
6	ZIP 32: Shielded Hierarchical Deterministic Wallets — Orchard key path
7	ZIP 200: Network Upgrade Mechanism
8	ZIP 209: Prohibit Out-of-Range Shielded Chain Value Pool Balances
9	ZIP 224: Orchard
10	ZIP 226: Transfer and Burn of Zcash Shielded Assets
11	ZIP 226: Transfer and Burn of Zcash Shielded Assets — Note Structure and Commitment
12	ZIP 226: Transfer and Burn of Zcash Shielded Assets — Additional Consensus Rules for the assetBurn set
13	ZIP 226: Transfer and Burn of Zcash Shielded Assets — TxId Digest
14	ZIP 226: Transfer and Burn of Zcash Shielded Assets — Authorizing Data Commitment
15	ZIP 230: Version 6 Transaction Format — Issuance Action Description (IssueAction)
16	ZIP 230: Version 6 Transaction Format — Issue Note Description (IssueNoteDescription)
17	ZIP 230: Version 6 Transaction Format — Transaction Format
18	ZIP 244: Transaction Identifier Non-Malleability
19	ZIP 246: Digests for the Version 6 Transaction Format
20	ZIP 317: Proportional Transfer Fee Mechanism
21	ZIP 317: Proportional Transfer Fee Mechanism — Fee calculation
22	BIP 43: Purpose Field for Deterministic Wallets
23	BIP 340: Schnorr Signatures for secp256k1
24	Zcash Protocol Specification, Version 2025.6.2 [NU6.1] or later.
25	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 2: Notation
26	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 3.2: Notes
27	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 4.1.7: Signature
28	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 4.2.3: Orchard Key Components
29	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 4.10: SIGHASH Transaction Hashing
30	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 5.3: Constants
31	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 5.4.2: Pseudo Random Functions
32	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 5.4.9.8: Group Hash into Pallas and Vesta
33	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 5.6.4.2: Orchard Raw Payment Addresses
34	Zcash Protocol Specification, Version 2025.6.2 [NU6.1]. Section 7.1: Transaction Encoding and Consensus
35	An Introduction to Petname Systems. Marc Stiegler, updated June 2010.
