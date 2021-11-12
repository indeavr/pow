use std::env;
use rand::{RngCore};
use sha2::{Digest, Sha256};
use sha2::digest::generic_array::GenericArray;

const NUM_ZEROS: usize = 4;
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
        // get some random data:
        let mut random_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut random_bytes);

        let nonce = format!("{:x?}", &random_bytes);
        println!("Nonce: {:?}", &nonce);

        let result = sha256_hash(&random_bytes);
        println!("Result: {:?}", &result);

        // let result_in_hex = format!("{:x}", result);
        let is_match = check_result_hex(&format!("{:x}", &result));
        println!("Is Match: {}", is_match);

        if is_match {
            break (result, random_bytes);
        }
    };

    println!("<<< FOUND-COUNT >>>{}", count);
    println!("raw-- Nonce: {:?}, Result: {:?}", &nonce, &found_result);
    println!("hex-- Nonce: {:x?}, Result: {:?}", &nonce, format!("{:x}", &found_result));
}

fn check_result_hex(hex: &str) -> bool {
    let first_n = &hex[..NUM_ZEROS];

    println!("First N: {:?}", first_n);
    first_n.chars().all(|b| b == '0')
}
