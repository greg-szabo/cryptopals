# Cryptopals challenges library in Rust

Please see [the cryptopals crypto challenges](https://cryptopals.com) for details.

Currently the first set of challenges are implemented.

## How to check the challenges
Check out the `src/challenges.rs` file for solutions to the challenges. The test assertions usually don't use the full
solution so there is some magic left for you to discover.

```shell script
cargo test
```

## How to use the library
There is a small example program that uses cryptopals as a library in `src/main.rs`.

```shell script
cargo run
```