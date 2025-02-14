mod key_management;
mod transaction;
mod network;

use key_management::generate_keypair;
use transaction::sign_transaction;
use network::broadcast_transaction;

fn main() {
    // generate the keypair (private key and address)
    let (private_key, address) = generate_keypair();
    println!("Generated Ethereum Address: {}", address);

    // get user input for the transaction (e.g., nonce, recipient, and value)
    let nonce: u64 = 0; // TODO: get it from network
    let to = "0xRecipientAddressHere";
    let value: u64 = 1_000_000_000_000_000; // 0.001 ETH

    // create and sign the transaction
    let signed_tx = sign_transaction(&private_key, nonce, to, value);

    // broadcast the transaction using Infura or another node provider
    let infura_url = "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID";
    broadcast_transaction(&signed_tx, infura_url);
}
