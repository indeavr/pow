use std::env;
use rand::{RngCore};
use sha2::{Digest, Sha256};
use sha2::digest::generic_array::GenericArray;

const NONCE_SIZE: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    let byte64_string = &args[1];
    println!("Input: {}, len: {}", &byte64_string, &byte64_string.len());

    let sha256_hash = |nonce: &[u8]| -> GenericArray<u8, _> {
        let mut hasher = Sha256::new();
        hasher.update(&byte64_string.as_bytes());
        hasher.update(&nonce);
        let res = hasher.finalize();
        res
    };

    let mut count = 0;
    let (found_result, nonce) = loop {
        count += 1;
        let mut random_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut random_bytes);

        let result = sha256_hash(&random_bytes);
        let is_match = check_result_as_hex(&hex::encode(&result));

        if is_match {
            break (result, random_bytes);
        }

        if count % 5000 == 0 {
            println!("Tick:  {}", count);
        }
    };

    println!("Nonce: {:?}", &hex::encode(&nonce));
    println!("Result: {:?}", &hex::encode(&found_result));
}

fn check_result_as_hex(hex: &str) -> bool {
    let len = hex.len();
    let last = &hex[len - 2..];
    if last == "fe" {
        let second_to_last = &hex[len - 4..len - 2];
        if second_to_last == "ca" {
            return true;
        }
    }
    false
}
