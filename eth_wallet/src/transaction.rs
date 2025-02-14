use rlp::RlpStream;
use secp256k1::{Secp256k1, SecretKey, Message};
use sha3::{Digest, Keccak256};

pub fn sign_transaction(private_key: &SecretKey, nonce: u64, to: &str, value: u64) -> Vec<u8> {
    /* Create an RLP encoded transaction
    let mut stream = RlpStream::new_list(9);
    stream.append(&nonce);
    stream.append(&50_000_000_000u64); // Example gas price
    stream.append(&21000u64); // Gas limit for a simple transaction
    stream.append(&to);
    stream.append(&value);
    stream.append_empty_data(); // No additional data
    stream.append(&1u64); // Chain ID for mainnet (EIP155)
    stream.append_empty_data();
    stream.append_empty_data();

    let encoded = stream.out();

    // Hash transaction using keccak256
    let hash = Keccak256::digest(&encoded);
    let message = Message::from_slice(&hash).unwrap();

    // Sign with secp256k1
    let secp = Secp256k1::new();
    let signature = secp.sign_ecdsa(&message, private_key);

    signature.serialize_compact().to_vec()
}
