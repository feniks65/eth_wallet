use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_keypair() -> (SecretKey, String) {
    // generate a random private key
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let mut private_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut private_key_bytes);
    let private_key = SecretKey::from_slice(&private_key_bytes)
        .expect("32 bytes, within curve order");

    // derive the public key
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // Hash the public key to get the address
    lethpublic_key_bytes = public_key.serialize_uncompressed();
    let address_hash = Keccak256::digest(&public_key_bytes[1..]);
    let address = hex::encode(&address_hash[12..]);

    (private_key, format!("0x{}", address))
}
