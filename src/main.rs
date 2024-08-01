use std::time::Instant;
use rand::RngCore;

// For rust-crypto
use crypto::aes::{ctr, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;

const DATA_SIZE: usize = 150 * 1024 * 1024; // 150MB

fn main() {
    let mut data = vec![0u8; DATA_SIZE];
    rand::thread_rng().fill_bytes(&mut data);

    let key = [0u8; 32]; // 256-bit key
    let nonce = [0u8; 16]; // AES block size

    println!("Benchmarking AES implementations for 150MB of data:");

    // Rust-crypto CTR
    let start = Instant::now();
    let mut rust_crypto_ctr = ctr(KeySize::KeySize256, &key, &nonce);
    let mut output = vec![0u8; DATA_SIZE];
    rust_crypto_ctr.process(&data, &mut output);
    println!("Rust-crypto CTR: {:?}", start.elapsed());
}
