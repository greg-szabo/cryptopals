use crate::aes;
use crate::caesar;
use crate::vigenere;
use crate::Number;

// https://cryptopals.com/sets/1/challenges/1
#[test]
fn challenge_1() {
    let hexstr = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(Number::from_hexstr(hexstr).to_b64string(), result);
}

// https://cryptopals.com/sets/1/challenges/2
#[test]
fn challenge_2() {
    let hexstr1 = "1c0111001f010100061a024b53535009181c";
    let hexstr2 = "686974207468652062756c6c277320657965";
    let result = "746865206b696420646f6e277420706c6179";
    assert_eq!(
        Number::from_hexstr(hexstr1)
            .xor(Number::from_hexstr(hexstr2))
            .to_string(),
        result
    );
}

// https://cryptopals.com/sets/1/challenges/3
#[test]
fn challenge_3() {
    let number =
        Number::from_hexstr("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let found_encryption_key = crate::caesar::solve(&number.to_vec()).0;
    let should_result = "Cooking MC's like a pound of bacon";
    assert_eq!(found_encryption_key, 88);
    assert_eq!(
        number
            .xor(Number::from_bytes(&vec![found_encryption_key]))
            .to_text(),
        should_result
    );
}

// https://cryptopals.com/sets/1/challenges/4
#[test]
fn challenge_4() {
    // Read file
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };
    let file = File::open("4.txt").expect("could not read 4.txt");
    let reader = BufReader::new(file);

    let mut found_encoded = String::new();
    let mut found_encryption_key = 0;
    let mut min_std_dev = std::f64::MAX;

    // Read each line
    for line in reader.lines() {

        let encoded_line = line.expect("could not read line in 4.txt");
        let encoded_number = Number::from_hexstr(encoded_line.as_str());

        // Solve it and compare to previously solved lines. The one closest to the English language occurrence frequencies will win.
        let (encryption_key,std_dev) = caesar::solve(&encoded_number.to_vec());
        if min_std_dev > std_dev {
            min_std_dev = std_dev;
            found_encryption_key = encryption_key;
            found_encoded = encoded_line;
        }

    }

    let decoded = Number::from_hexstr(found_encoded.as_str()).xor(Number::from_bytes(&vec![found_encryption_key]));

    assert_eq!(
        found_encoded,
        "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f"
    );
    assert_eq!(
        decoded.to_text(),
        "Now that the party is jumping\n"
    );
    assert_eq!(
        found_encryption_key,
        53 //'5'
    );
}

// https://cryptopals.com/sets/1/challenges/5
#[test]
fn challenge_5() {
    let input = Number::from_str(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    );
    let encryption_key = Number::from_str("ICE");
    let result = input.xor(encryption_key).to_string();

    assert_eq!(
        result,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}

// https://cryptopals.com/sets/1/challenges/6
#[test]
fn challenge_6() {
    // Create Number
    let encoded = Number::from_b64file("6.txt");

    assert_eq!(
        encoded.to_b64string().starts_with("HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2"),
        true
    );

    // Check key size
    let key_size = vigenere::key_size(&encoded.to_vec()).unwrap();
    assert_eq!(
        key_size,
        29
    );

    // Solve Vigenere cipher
    let vigenere_key = vigenere::solve(&encoded.to_vec()).unwrap();
    let result = encoded.xor(Number::from_bytes(&vigenere_key));

    assert_eq!(
        result.to_text().starts_with("I'm back and I'm ringin' the bell"),
        true
    );
}

// https://cryptopals.com/sets/1/challenges/7
#[test]
fn challenge_7() {
    // Create Number
    let encoded = Number::from_b64file("7.txt");
    let result = aes::decrypt(&encoded.to_vec(),&"YELLOW SUBMARINE".as_bytes().to_vec(), aes::Mode::ECB).unwrap();
    let result_text = Number::from_bytes(&result).to_text();
    println!("{:?}",result);
    assert_eq!(
        result_text.starts_with("I'm back and I'm ringin' the bell"),
        true
    )
}

// https://cryptopals.com/sets/1/challenges/8
#[test]
fn challenge_8() {
    // Read file
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };
    let file = File::open("8.txt").expect("could not read 8.txt");
    let reader = BufReader::new(file);
    let mut result = false;
    let mut result_line = Number::new();

    // Read each line
    for line in reader.lines() {

        let encoded_line = line.expect("could not read line in 8.txt");
        let encoded_number = Number::from_hexstr(encoded_line.as_str());

        result = aes::detect(&encoded_number.to_vec(), aes::Mode::ECB);
        if result {
            result_line = encoded_number;
            break;
        }

    }

    assert_eq!(
        result,
        true
    );

    assert_eq!(
        result_line.to_string(),
        "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a"
    );
}
