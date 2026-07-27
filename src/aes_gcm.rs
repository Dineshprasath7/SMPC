use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use rand::{Rng, TryRng};

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; 12]), String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    let _ = rand::rng().try_fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("Encryption Failed");

    Ok((ciphertext, nonce_bytes))
}

pub fn decrypt(
    key: &[u8; 32],
    nonce_bytes: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let nonce = Nonce::from_slice(nonce_bytes);

    let result: Vec<u8> = cipher
        .decrypt(nonce, ciphertext)
        .expect("Decryption Failed");
    Ok(result)
}
