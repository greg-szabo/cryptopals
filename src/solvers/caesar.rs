use crate::xor;
use crate::english_std_dev;

pub fn solve(x: &Vec<u8>) -> (u8, f64) {

    // Count the encrypted bytes
    let mut sorted_count = vec![0u8; 256];
    for &b in x.iter() {
        sorted_count[b as usize] += 1;
    }

    // Find the highest occuring byte
    let mut most_common_encoded_byte = 0;
    let mut highest_occurence = sorted_count[0];
    for (i, &item) in sorted_count.iter().enumerate() {
        if item > highest_occurence {
            most_common_encoded_byte = i;
            highest_occurence = item;
        }
    }

    // Go through the most common characters of the English language and use them to decode the highest occurring byte
    let mut minimum_score = std::f64::MAX;
    let mut found_key = 255;
    for &possible_most_common_character in "etaoin shrdlu".as_bytes() { //c
        let possible_encryption_key = most_common_encoded_byte as u8 ^ possible_most_common_character;

        // Check the decoded bytes' standard deviation from the frequency of the letters in the English language
        let possible_decoded_std_dev = english_std_dev(&xor(x,&vec![possible_encryption_key]));

        // Check if we found a lower standard deviation than before
        if possible_decoded_std_dev < minimum_score {
            minimum_score = possible_decoded_std_dev;
            found_key = possible_encryption_key;
        };


    }
    (found_key, minimum_score)
}
