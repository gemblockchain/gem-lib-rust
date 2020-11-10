# gem-lib-rust

![crates.io](https://img.shields.io/crates/v/gemblockchain.svg)
![docs.rs](https://docs.rs/gemblockchain/badge.svg)

Library to work with [Gem blockchain](https://github.com/gemblockchain)

# Usage
```rust
use base58::*;
use std::time::{SystemTime, UNIX_EPOCH};
use gemblockchain::account::{PrivateKeyAccount, TESTNET};
use gemblockchain::seed::*;
use gemblockchain::transaction::*;

fn main() {
    let phrase = generate_phrase();
    let account = PrivateKeyAccount::from_seed(&phrase);
    println!("My TESTNET address: {}", account.public_key().to_address(TESTNET).to_string());

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
    let tx = Transaction::new_alias(&account.public_key(), "rhino", TESTNET, 100000, ts);
    println!("ID is {}", tx.id().to_string());
    let ptx = account.sign_transaction(tx);
    println!("Proofs are {:?}", ptx.proofs.iter().map(|p| p.to_base58()).collect::<Vec<String>>());
}
```
