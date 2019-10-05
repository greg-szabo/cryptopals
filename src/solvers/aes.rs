use openssl::symm;
use std::collections::HashMap;

pub enum Mode {
    ECB,
}

pub fn decrypt(encrypted: &Vec<u8>, key: &Vec<u8>, mode: Mode) -> Option<Vec<u8>> {
    match mode {
        Mode::ECB => {
            let cipher = symm::Cipher::aes_128_ecb();
            let decoded = symm::decrypt(cipher, key.as_slice(), None, encrypted.as_slice());
            if decoded.is_err() {
                return None;
            }
            decoded.ok()
        },
    }
}

fn convert_u8s_to_u128(encrypted: &Vec<u8>) -> Vec<u128> {
    let mut result = vec![];
    let mut build_multiplier = 0;
    let mut build_number = 0;
    for b in encrypted.iter() {
        if build_multiplier == 16 {
            build_multiplier = 0;
            result.push(build_number);
            build_number = 0;
        }
        build_number += (256 as u128).checked_pow(build_multiplier).unwrap() * *b as u128;
        build_multiplier += 1;
    }
    result.push(build_number);
    result
}

fn find_ecb_repeats(encrypted: &Vec<u8>) -> bool {
    let block_numbers = convert_u8s_to_u128(encrypted);
    let mut collect: HashMap<u128, usize> = HashMap::new();
    for &b in block_numbers.iter() {
        if collect.contains_key(&b) {
            collect.insert(b, collect[&b] + 1);
        } else {
            collect.insert(b, 1);
        }
    }
    println!("{:?}",collect);
    for r in collect.iter() {
        if *r.1 > 1 {
            return true;
        }
    }
    false
}

pub fn detect(encrypted: &Vec<u8>, mode: Mode) -> bool {
    match mode {
        Mode::ECB => {
            find_ecb_repeats(encrypted)
        },
    }
}
