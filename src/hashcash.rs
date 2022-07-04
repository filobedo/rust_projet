use md5;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

#[derive(Debug)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}



pub fn hashcash(input: &MD5HashCashInput) -> MD5HashCashOutput {

    loop
    {
        let mut seed: u64 = random();
        let mut seed_with_message = format!("{}{}", seed.to_string(), input.message);
        let mut hashcode128 = u128::from_be_bytes(md5::compute(seed_with_message).0);
        let current_complexity = hashcode128.leading_zeros();
        let hashcode = format!("{:x}", hashcode128);
        if current_complexity >= input.complexity{
            return MD5HashCashOutput {
                seed,
                hashcode
            }
        }
    }
}