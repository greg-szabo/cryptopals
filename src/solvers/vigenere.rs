use crate::hamming;
use crate::caesar;

pub fn key_size(x: &Vec<u8>) -> Option<usize> {

    // Key size has to be at least 2.
    if x.len() < 4 {
        return None;
    }

    // Calculate Hamming distance for each key size from the first 2*key_size bytes
    let mut key_sizes = vec![0.0, 0.0]; // 0 key width doesn't make sense, 1 key width is Cesar cipher
    use std::cmp::min;
    for key_size in 2..=min(x.len() / 2, 40) { // Maximum key size: 40
        let mut distance = hamming(&x[0..key_size].to_vec(),&x[key_size..2 * key_size].to_vec()) as f64;
        // Try to take sample of the next set of bytes too.
        if 4 * key_size <= x.len() {
            let distance2 = hamming(&x[0..key_size].to_vec(),&x[2 * key_size..3 * key_size].to_vec()) as f64;
            let distance3 = hamming(&x[0..key_size].to_vec(),&x[3 * key_size..4 * key_size].to_vec()) as f64;
            let distance4 = hamming(&x[key_size..2 * key_size].to_vec(),&x[2 * key_size..3 * key_size].to_vec()) as f64;
            let distance5 = hamming(&x[key_size..2 * key_size].to_vec(),&x[3 * key_size..4 * key_size].to_vec()) as f64;
            let distance6 = hamming(&x[2 * key_size..3 * key_size].to_vec(),&x[3 * key_size..4 * key_size].to_vec()) as f64;
            distance = (distance + distance2 + distance3 + distance4 + distance5 + distance6) / 6.0;
        }
        key_sizes.push(100.0 * distance / key_size as f64);
    };

    // Find the key size with the smallest Hamming distance

    let (mut smallest_key_size_index, mut smallest_key_size_value) = (2,key_sizes[2]);
    for (index, &item) in key_sizes.iter().enumerate() {
        if index < 2 {
            continue;
        }
        if item < smallest_key_size_value {
            smallest_key_size_index = index;
            smallest_key_size_value = item;
        }
    }
    Some(smallest_key_size_index)
}

pub fn solve(encrypted: &Vec<u8>) -> Option<Vec<u8>> {

    let mut result = vec![];

    // Get the best possible key size
    let keysize = key_size(encrypted);
    if keysize == None {
        return None;
    }
    let keysize = keysize.unwrap();

    // Create blocks
    for k in 0..keysize {
        let mut block = vec![];
        let mut i = k;
        while i < encrypted.len() {
            block.push(encrypted[i]);
            i += keysize;
        };
        result.push(caesar::solve(&block).0);
    };
    Some(result)
}
