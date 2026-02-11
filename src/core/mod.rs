pub mod encoder;
pub mod decoder;

use crc32fast::Hasher;
use argon2::Argon2;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use anyhow::{Result};
use rand::RngCore;

pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    let _ = argon2.hash_password_into(password.as_bytes(), salt, &mut key);
    key
}

pub fn encrypt_data(data: &[u8], key: &[u8; 32], index: u32) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    // Create a unique nonce for each chunk based on its index
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[0..4].copy_from_slice(&index.to_le_bytes());
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher.encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))
}

pub fn decrypt_data(encrypted_data: &[u8], key: &[u8; 32], index: u32) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[0..4].copy_from_slice(&index.to_le_bytes());
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}. Wrong password or corrupted data.", e))
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}
