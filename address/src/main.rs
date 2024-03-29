use anyhow::Result;
use bitcoin::{secp256k1::SecretKey, Network, PrivateKey};

fn main() -> Result<()> {
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let private_key = PrivateKey::new(secret_key, Network::Testnet);

    println!("Private key: {private_key}");

    Ok(())
}
