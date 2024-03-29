# `blockchain-base`

[![Crates.io](https://img.shields.io/crates/v/blockchain-base?style=plastic)](https://crates.io/crates/blockchain-base)

`blockchain-base` provides a customizable base for a blockchain implementation.

The purpose of this crate is to build your own Blockchain with as few dependencies as
possible. This means that when possible, the library uses the standard Rust structures and functions.

It is in the process of being part of [blockchain-satellite](https://github.com/sergiomtzlosa/blockchain-satellite) project for implementing an original idea of a blockchain over a Low-Orbit satellite. The idea is described on the paper:

[Blockchain and radio communications over suborbital spaceflights: Watchtowers and Mystics](https://arxiv.org/abs/1910.04835)


## Usage

Add `blockchain-base` to your project's `Cargo.toml`. For more details, consult the
[Cargo guide](http://doc.crates.io/guide.html#adding-dependencies).

Example:

```rust
extern crate blockchainblock;
use crate::blockchainblock::*;

let prev  : Option<BlockHash> = None;
let nonce : u64 = 3;
let timestamp : u64 = 4;
let data : [i32; 1] = [5];
let block : BlockchainBlock<i32> = BlockchainBlock::new(prev, &data, timestamp, nonce);
println!("\n{:?}\n", &block);

```

For more examples, consult the [documentation](https://docs.rs/blockchain-base/0.1.0/blockchainblock/).

## [Release Notes](https://github.com/esguti/blockchain-base/blob/master/NEWS.md)

## Acknowledgements

This crate was inspired by [blockchain-rust](https://github.com/GeekLaunch/blockchain-rust).

## Legal

`blockchain-base` is copyrighted under the terms of the ![AGPL-3.0](https://img.shields.io/github/license/esguti/blockchain-base?style=plastic) license. See [LICENSE](LICENSE.md) for details.
