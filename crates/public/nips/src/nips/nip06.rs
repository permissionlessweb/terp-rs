// NIP-06 - Basic key derivation from mnemonic seed phrase
Table of Contents
Basic key derivation from mnemonic seed phrase
Test vectors
Basic key derivation from mnemonic seed phrase
draft optional

BIP39 is used to generate mnemonic seed words and derive a binary seed from them.

BIP32 is used to derive the path m/44'/1237'/<account>'/0/0 (according to the Nostr entry on SLIP44).

A basic client can simply use an account of 0 to derive a single key. For more advanced use-cases you can increment account, allowing generation of practically infinite keys from the 5-level path with hardened derivation.

Other types of clients can still get fancy and use other derivation paths for their own other purposes.

