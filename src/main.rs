extern crate cryptopals;

use cryptopals::Number;

fn main() {
    let n = Number::from_hexstr("4141402041414020414140");
    println!("Example number: {}", n);
    println!("Example number base64 encoded: {}", n.to_b64string());
    println!("Read src/challenges.rs for the cryptopal challenges.");
    println!("Run 'cargo test' to run all cryptopal challenges.");
}
