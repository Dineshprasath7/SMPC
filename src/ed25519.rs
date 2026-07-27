use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey, ed25519::signature};
use rand::rngs::SysRng;
use rand_core::UnwrapErr;

use crate::shamir_secret_sharing::{reconstruct_secret, split_secret};

/// Generates a new Ed25519 keypair.
///
/// Returns:
/// - SigningKey (private key)
/// - VerifyingKey (public key)
fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let mut csprng = UnwrapErr(SysRng);
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    println!("Signing_Key => {:?}", signing_key);
    println!("Verifying_Key => {:?}", verifying_key);
    (signing_key, verifying_key)
}
pub fn sign_and_verify_msg() -> bool {
    let (signing_key, verifying_key) = generate_key_pair();

    let message = b"Hello from Ed25519";

    let signature = signing_key.sign(message);

    println!("Message => {:?}", message);
    println!("Signature => {:?}", signature);

    let shares = split_secret(&signature.to_bytes(), 3, 2);
    let recovered = reconstruct_secret(&shares);

    let recovered_key: &[u8] = &recovered;
    println!("Recovered Key => {:?}", recovered_key);
    // Verify the signature using the public key
    match verifying_key.verify_strict(message, &signature) {
        Ok(_) => {
            println!("Signature verification successful");
            true
        }
        Err(_) => {
            println!("Signature verification failed");
            false
        }
    }
}
