use md5;
use rand::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}

pub fn hashcash(input: &MD5HashCashInput) -> MD5HashCashOutput {
    loop
    {
        let seed: u64 = random();
        let mut seed_hexa = format!("{:x}", seed).to_string().to_uppercase();
        seed_hexa = format!("{:0>16}", seed_hexa);
        let seed_with_message=  seed_hexa + &input.message;
        let hashcode128 = calcul_md5_to_u128(seed_with_message);
        let current_complexity = calcul_complexity(hashcode128);
        let mut hashcode = format!("{:x}", hashcode128).to_string().to_uppercase();
        hashcode = format!("{:0>32}", hashcode);
        if current_complexity >= input.complexity{
            return MD5HashCashOutput {
                seed,
                hashcode
            }
        }
    }
}

fn calcul_complexity(hashcode: u128) -> u32 {
    hashcode.leading_zeros()
}

fn calcul_md5_to_u128(message: String) -> u128 {
    u128::from_be_bytes(md5::compute(message).0)
}

#[test]
fn test_compute_complexity() {
    let result = calcul_complexity(12345678901234567890123456789012u128);
    assert_eq!(result, 24);
}

#[test]
fn test_compute_md5_to_u128() {
    let result = calcul_md5_to_u128("12345678901234567890123456789012u128".to_string());
    assert_eq!(result.to_string(), "200197347730891166182504784119272975114");
}