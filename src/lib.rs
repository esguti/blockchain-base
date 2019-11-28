//! Provides a customizable base for a blockchain implementation

#![warn(missing_docs)]

/// Version of the protocol as appearing in block headers.
pub const VERSION: u8 = 1;
/// Lenght of the Hash block.
pub const BLOCKHASHLEN : usize = 32; // 2^8 * 2^5
/// Hash block representation.
pub type BlockHash = [u8; BLOCKHASHLEN]; // to store SHA256

mod byteable;
pub use crate::byteable::Byteable;
mod hashable;
pub use crate::hashable::Hashable;
// thanks to https://github.com/GeekLaunch/blockchain-rust
mod blockchainblock;
pub use crate::blockchainblock::BlockchainBlock;
