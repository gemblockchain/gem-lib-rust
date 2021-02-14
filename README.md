# gem-lib-rust

![crates.io](https://img.shields.io/crates/v/gemblockchain.svg)
![docs.rs](https://docs.rs/gemblockchain/badge.svg)

Library to work with Gem blockchain

# Usage
```rust
use gemblockchain::GemAddress;

let gem = GemAddress::generate(None);
println!("Address: {}", gem.address);
println!("Mnemonic phrase: {}", gem.mnemonic_phrase);
println!("Mini secret key: {}", gem.mini_secret_key_to_string());
println!("Public key: {}", gem.public_key_to_string());
```
