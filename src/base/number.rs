const BASE64_INDEX: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];

#[derive(Default, Clone, Eq, PartialEq)]
pub struct Number {
    bytes: Vec<u8>,
}

use std::fmt;
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const HEX_INDEX: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut result = vec![];
        for &b in &self.bytes {
            result.push((b & 0b11110000) >> 4);
            result.push(b & 0b00001111);
        }
        write!(
            f,
            "{}",
            result
                .iter()
                .map(|&x| HEX_INDEX[x as usize])
                .collect::<String>()
        )
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.bytes)
    }
}
/*
impl Iterator for Number {
    type Item = u8;
    fn next(&self) -> Option<u8> {
        self.bytes.iter().map(|&x| x).next()
    }
}
*/
impl Number {
    pub fn new() -> Self {
        Number { bytes: vec![] }
    }

    pub fn from_hexstr(hexstr: &str) -> Self {
        let mut bytes = vec![];
        let mut iter = hexstr.chars();

        // if the hex string length is odd, we start with a leading 0.
        let mut i = hexstr.len() % 2;
        if i == 1 {
            bytes.push(
                iter.next()
                    .expect("invalid first character")
                    .to_digit(16)
                    .expect("invalid hex character in input") as u8,
            );
        };

        // Now we have an even number of characters to process
        while i < hexstr.len() {
            let big = iter
                .next()
                .expect("invalid character")
                .to_digit(16)
                .expect("invalid hex character in input") as u8;
            let little = iter
                .next()
                .expect("invalid character")
                .to_digit(16)
                .expect("invalid hex character in input") as u8;
            let number = (big << 4) | little;
            bytes.push(number);
            i += 2;
        }
        Number { bytes }
    }

    pub fn from_str(input: &str) -> Self {
        Number {
            bytes: input.as_bytes().to_vec(),
        }
    }

    pub fn from_bytes(input: &Vec<u8>) -> Self {
        Number {
            bytes: input.clone(),
        }
    }

    pub fn from_b64bytes(input: &Vec<u8>) -> Self {
        let mut result = Number::new();
        let mut result_remainder = 0;
        let mut result_remainder_mask = 0;
        use std::collections::BTreeMap;
        let base64_reverse_index: BTreeMap<char, u8> =
            BASE64_INDEX.iter().map(|x| *x).zip(0u8..=64).collect();
        for i in input {
            // Line breaks don't count
            if i == &10 || i == &13 {
                continue;
            }
            // '=' means the encoding finished
            if i == &61 {
                continue;
            }
            // Check for valid base64-encoded bytes
            if !(i == &43
                || (i > &46 && i <= &57)
                || (i >= &65 && i <= &90)
                || (i >= &97 && i <= &122))
            {
                panic!(format!("invalid byte {}", *i));
            }
            let b = base64_reverse_index[&(*i as char)];
            match result_remainder_mask {
                0b00000000 => {
                    result_remainder = b << 2;
                    result_remainder_mask = 0b11111100;
                }
                0b11111100 => {
                    result
                        .bytes
                        .push(result_remainder | ((b & 0b00110000) >> 4));
                    result_remainder = (b & 0b00001111) << 4;
                    result_remainder_mask = 0b11110000;
                }
                0b11110000 => {
                    result
                        .bytes
                        .push(result_remainder | ((b & 0b00111100) >> 2));
                    result_remainder = (b & 0b00000011) << 6;
                    result_remainder_mask = 0b11000000;
                }
                0b11000000 => {
                    result.bytes.push(result_remainder | b);
                    result_remainder = 0;
                    result_remainder_mask = 0;
                }
                _ => {
                    panic!("Invalid state while decoding base64.");
                }
            };
        }
        if result_remainder_mask != 0b11000000 && result_remainder_mask != 0b00000000 {
            panic!(
                "Dangling bits at the end while decoding base64. Remainder: {:8b}, Mask: {:8b}",
                result_remainder, result_remainder_mask
            )
        }
        result
    }

    pub fn from_b64file(filename: &str) -> Self {
        use std::{
            fs::File,
            io::prelude::*,
        };

        // Read file into Vector
        let mut file = File::open(filename).expect(format!("could not read {}",filename).as_str());
        let mut input = Vec::new();
        let _f = file.read_to_end(&mut input);

        Number::from_b64bytes(&input)
    }

    pub fn to_b64string(&self) -> String {
        let mut b64bytes: Vec<u8> = vec![];
        let mut remainder: u8 = 0;
        let mut remainder_mask: u8 = 0b0;
        for &b in self.bytes.iter() {
            match remainder_mask {
                0b00000000 => {
                    b64bytes.push((b & 0b11111100) >> 2);
                    remainder_mask = 0b00000011;
                }
                0b00000011 => {
                    b64bytes.push((remainder << 4) | (b & 0b11110000) >> 4);
                    remainder_mask = 0b00001111;
                }
                0b00001111 => {
                    b64bytes.push((remainder << 2) | ((b & 0b11000000) >> 6));
                    remainder_mask = 0b00111111;
                }
                0b00111111 => {
                    b64bytes.push(remainder);
                    //remainder_mask = 0b00000000;
                    // The remainder already gives a full character, but we have to process b too.
                    b64bytes.push((b & 0b11111100) >> 2);
                    remainder_mask = 0b00000011;
                }
                _ => panic!("invalid remainder during base64 conversion"),
            };
            remainder = b & remainder_mask;
        }
        // Remaining bits
        if remainder_mask > 0 {
            b64bytes.push(remainder << (6 - remainder_mask.count_ones()));
        }

        // Add padding
        while b64bytes.len() % 4 != 0 {
            b64bytes.push(64);
        }

        b64bytes.iter().map(|&x| BASE64_INDEX[x as usize]).collect()
    }

    // Note the missing ref. This consumes the struct.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_text(&self) -> String {
        self.bytes.iter().map(|&x| x as char).collect()
    }

    pub fn xor(&self, other: Self) -> Self {
        use crate::xor;
        //Todo: there's an extra cloning here that's unnecessary
        Number::from_bytes(&xor(&self.bytes,&other.bytes))
    }

    pub fn hamming(&self, other: Self) -> u32 {
        use crate::hamming;
        hamming(&self.bytes,&other.bytes)
    }

}
