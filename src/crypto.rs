use blake3::Hasher;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

/// Generate a new Ed25519 keypair
/// Returns (private_key_hex, public_key_hex)
pub fn generate_keypair() -> (String, String) {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let private_key = hex::encode(signing_key.to_bytes());
    let public_key = hex::encode(verifying_key.to_bytes());

    (private_key, public_key)
}

/// Sign data with private key
/// Returns signature as hex string
pub fn sign_data(private_key_hex: &str, data: &[u8]) -> String {
    let bytes = hex::decode(private_key_hex).expect("Invalid private key hex");
    let signing_key =
        SigningKey::from_bytes(bytes.as_slice().try_into().expect("Invalid key length"));
    let signature = signing_key.sign(data);
    hex::encode(signature.to_bytes())
}

/// Verify signature against data and public key
pub fn verify_signature(public_key_hex: &str, data: &[u8], signature_hex: &str) -> bool {
    let pk_bytes = hex::decode(public_key_hex).expect("Invalid public key hex");
    let sig_bytes = hex::decode(signature_hex).expect("Invalid signature hex");

    let verifying_key =
        VerifyingKey::from_bytes(pk_bytes.as_slice().try_into().expect("Invalid key length"))
            .expect("Invalid verifying key");
    let signature = Signature::from_bytes(
        sig_bytes
            .as_slice()
            .try_into()
            .expect("Invalid signature length"),
    );

    verifying_key.verify(data, &signature).is_ok()
}

/// Hash data using BLAKE3
pub fn hash_data(data: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hex::encode(hash.as_slice())
}

/// Generate wallet address from public key
/// Format: nomad1<38 hex chars>
pub fn generate_address(public_key: &str) -> String {
    let hash = hash_data(public_key.as_bytes());
    format!("nomad1{}", &hash[..38])
}

/// Derive public key from private key
pub fn derive_public_key(private_key_hex: &str) -> Result<String, String> {
    let bytes = hex::decode(private_key_hex).map_err(|e| e.to_string())?;
    let signing_key = SigningKey::from_bytes(
        bytes.as_slice().try_into().map_err(|_| "Invalid key length".to_string())?,
    );
    let verifying_key = signing_key.verifying_key();
    Ok(hex::encode(verifying_key.to_bytes()))
}


/// Create a deterministic keypair from a seed phrase (simplified)
pub fn keypair_from_seed(seed: &str) -> (String, String) {
    // In production, use BIP39/BIP32 derivation
    let mut hasher = Hasher::new();
    hasher.update(seed.as_bytes());
    let seed_bytes = hasher.finalize().as_bytes()[..32].to_vec();

    let signing_key =
        SigningKey::from_bytes(seed_bytes.as_slice().try_into().expect("Invalid seed"));
    let verifying_key = signing_key.verifying_key();

    let private_key = hex::encode(signing_key.to_bytes());
    let public_key = hex::encode(verifying_key.to_bytes());

    (private_key, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (private_key, public_key) = generate_keypair();
        assert_eq!(private_key.len(), 64); // 32 bytes = 64 hex chars
        assert_eq!(public_key.len(), 64);
    }

    #[test]
    fn test_sign_and_verify() {
        let (private_key, public_key) = generate_keypair();
        let data = b"test message";

        let signature = sign_data(&private_key, data);
        assert!(verify_signature(&public_key, data, &signature));

        // Verify with wrong data should fail
        assert!(!verify_signature(&public_key, b"wrong data", &signature));
    }

    #[test]
    fn test_hash_data() {
        let hash1 = hash_data(b"hello");
        let hash2 = hash_data(b"hello");
        let hash3 = hash_data(b"world");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // BLAKE3 produces 256-bit hash = 64 hex chars
    }

    #[test]
    fn test_generate_address() {
        let (_, public_key) = generate_keypair();
        let address = generate_address(&public_key);

        assert!(address.starts_with("nomad1"));
        assert_eq!(address.len(), 44); // "nomad1" + 38 chars
    }

    #[test]
    fn test_keypair_from_seed() {
        let (pk1, pub1) = keypair_from_seed("test seed");
        let (pk2, pub2) = keypair_from_seed("test seed");
        let (pk3, pub3) = keypair_from_seed("different seed");

        assert_eq!(pk1, pk2);
        assert_eq!(pub1, pub2);
        assert_ne!(pk1, pk3);
        assert_ne!(pub1, pub3);
    }
}
