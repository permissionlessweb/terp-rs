// encrypted payload: https://nips.nostr.com/44

//! NIP-44 v2: Encrypted Payloads (Versioned)
//! https://github.com/nostr-protocol/nips/blob/master/44.md

use base64::Engine;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use secp256k1::{ecdh::shared_secret_point, Parity, PublicKey, SecretKey, XOnlyPublicKey};
use sha2::Sha256;
use std::convert::TryInto;
use crate::NipResult;


/// Re-export for convenience
pub type Nip44Result<T> = Result<T, Nip44Error>;

/// Conversation key (long-term shared secret between two keys)
pub type ConversationKey = [u8; 32];

/// Version 2 (only supported)
const VERSION: u8 = 2;

/// Calculate conversation key from your private key + their public key
pub fn get_conversation_key(privkey: &SecretKey, their_pubkey: &XOnlyPublicKey) -> ConversationKey {
    let pubkey = PublicKey::from_x_only_public_key(*their_pubkey, Parity::Even);
    let mut shared_point = shared_secret_point(&pubkey, privkey).as_slice().to_vec();
    shared_point.resize(32, 0); // keep only x-coordinate

    let (conversation_key, _) = Hkdf::<Sha256>::extract(Some(b"nip44-v2"), &shared_point);
    conversation_key.into()
}

struct MessageKeys([u8; 76]);

impl MessageKeys {
    fn zero() -> Self {
        MessageKeys([0; 76])
    }

    fn encryption(&self) -> [u8; 32] {
        self.0[0..32].try_into().unwrap()
    }

    fn nonce(&self) -> [u8; 12] {
        self.0[32..44].try_into().unwrap()
    }

    fn auth(&self) -> [u8; 32] {
        self.0[44..76].try_into().unwrap()
    }
}

fn get_message_keys(conversation_key: &[u8; 32], nonce: &[u8; 32]) -> Nip44Result<MessageKeys> {
    let hk = Hkdf::<Sha256>::from_prk(conversation_key)
        .map_err(|_| Nip44Error::HkdfLength(conversation_key.len()))?;

    let mut keys = MessageKeys::zero();
    hk.expand(nonce, &mut keys.0)
        .map_err(|_| Nip44Error::HkdfLength(76))?;

    Ok(keys)
}

fn calc_padded_len(unpadded_len: usize) -> usize {
    if unpadded_len < 32 {
        return 32;
    }
    let next_power = 1usize << ((unpadded_len - 1).ilog2() + 1);
    let chunk = if next_power <= 256 {
        32
    } else {
        next_power / 8
    };
    chunk * (((unpadded_len - 1) / chunk) + 1)
}

fn pad(plaintext: &str) -> Nip44Result<Vec<u8>> {
    let len = plaintext.len();
    if len < 1 {
        return Err(Nip44Error::MessageIsEmpty);
    }
    if len > 65535 {
        return Err(Nip44Error::MessageIsTooLong);
    }

    let padded_len = calc_padded_len(len);
    let mut padded = Vec::with_capacity(2 + padded_len);
    padded.extend_from_slice(&(len as u16).to_be_bytes());
    padded.extend_from_slice(plaintext.as_bytes());
    padded.resize(2 + padded_len, 0);
    Ok(padded)
}

fn unpad(padded: &[u8]) -> Nip44Result<String> {
    if padded.len() < 2 {
        return Err(Nip44Error::InvalidPadding);
    }
    let unpadded_len = u16::from_be_bytes(padded[0..2].try_into().unwrap()) as usize;

    if unpadded_len == 0
        || padded.len() < 2 + unpadded_len
        || padded.len() != 2 + calc_padded_len(unpadded_len)
    {
        return Err(Nip44Error::InvalidPadding);
    }

    let unpadded = &padded[2..2 + unpadded_len];
    Ok(String::from_utf8(unpadded.to_vec()).map_err(|_| Nip44Error::Utf8Decode)?)
}

/// Encrypt plaintext for a recipient
pub fn encrypt(conversation_key: &ConversationKey, plaintext: &str) -> Nip44Result<String> {
    let mut nonce = [0u8; 32];
    OsRng.fill_bytes(&mut nonce);

    let keys = get_message_keys(conversation_key, &nonce)?;

    let mut buffer = pad(plaintext)?;
    let mut cipher = ChaCha20::new(&keys.encryption().into(), &keys.nonce().into());
    cipher.apply_keystream(&mut buffer);

    let mut mac = Hmac::<Sha256>::new_from_slice(&keys.auth())?;
    mac.update(&nonce);
    mac.update(&buffer);
    let mac_bytes = mac.finalize().into_bytes();

    let mut payload = vec![VERSION];
    payload.extend_from_slice(&nonce);
    payload.extend_from_slice(&buffer);
    payload.extend_from_slice(&mac_bytes);

    Ok(base64::engine::general_purpose::STANDARD.encode(payload))
}

/// Decrypt NIP-44 v2 payload
pub fn decrypt(conversation_key: &ConversationKey, payload: &str) -> Nip44Result<String> {
    if payload.as_bytes().first() == Some(&b'#') {
        return Err(Nip44Error::UnsupportedFutureVersion);
    }

    let data = base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(Nip44Error::Base64Decode)?;

    if data.len() < 99 || data[0] != VERSION {
        return Err(Nip44Error::UnknownVersion);
    }

    let nonce: [u8; 32] = data[1..33]
        .try_into()
        .map_err(|_| Nip44Error::InvalidData)?;
    let ciphertext = &data[33..data.len() - 32];
    let mac = &data[data.len() - 32..];

    let keys = get_message_keys(conversation_key, &nonce)?;

    // Verify MAC
    let mut calculated_mac = Hmac::<Sha256>::new_from_slice(&keys.auth())?;
    calculated_mac.update(&nonce);
    calculated_mac.update(ciphertext);
    let calculated = calculated_mac.finalize().into_bytes();

    if !constant_time_eq::constant_time_eq(mac, calculated.as_slice()) {
        return Err(Nip44Error::InvalidMac);
    }

    let mut buffer = ciphertext.to_vec();
    let mut cipher = ChaCha20::new(&keys.encryption().into(), &keys.nonce().into());
    cipher.apply_keystream(&mut buffer);

    unpad(&buffer)
}

// ====================== Integration with your library ======================

/// High-level helper that works with your existing `RawNostrEvent` / `PubKey`
pub fn encrypt_for_event(
    sender_privkey: &SecretKey,
    recipient_pubkey: &XOnlyPublicKey,
    plaintext: &str,
) -> NipResult<String> {
    let conv_key = get_conversation_key(sender_privkey, recipient_pubkey);
    encrypt(&conv_key, plaintext).map_err(Into::into)
}

pub fn decrypt_for_event(
    recipient_privkey: &SecretKey,
    sender_pubkey: &XOnlyPublicKey,
    encrypted_payload: &str,
) -> NipResult<String> {
    let conv_key = get_conversation_key(recipient_privkey, sender_pubkey);
    decrypt(&conv_key, encrypted_payload).map_err(Into::into)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Nip44Error {
    #[error("Base64 decode: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("InvalidLength: {0}")]
    InvalidLength(#[from] sha2::digest::InvalidLength),

    #[error("HKDF length error: {0}")]
    HkdfLength(usize),

    #[error("Invalid MAC")]
    InvalidMac,

    #[error("Invalid padding")]
    InvalidPadding,

    #[error("Message is empty")]
    MessageIsEmpty,

    #[error("Message too long")]
    MessageIsTooLong,

    #[error("Unsupported future version")]
    UnsupportedFutureVersion,

    #[error("Unknown encryption version")]
    UnknownVersion,

    #[error("UTF-8 decode error")]
    Utf8Decode,

    #[error("Invalid data")]
    InvalidData,
}

impl From<Nip44Error> for crate::NipError {
    fn from(e: Nip44Error) -> Self {
        crate::NipError::Crypto(format!("NIP-44: {}", e))
    }
}
