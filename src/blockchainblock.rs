use crypto_hash::{Algorithm, digest};
use std::fmt;
use super::*;

/// Structure for storing one Block of the Blockchain.
pub struct BlockchainBlock<T>{
    /// hash of the current block
    pub curr_hash: BlockHash,
    /// hash of the previous block. Is `None` for the first block
    pub prev_hash: Option<BlockHash>,
    /// encrypted data in compressed form
    pub data: T,
    /// time of block creation in seconds since 1970-01-01T00:00 UTC
    pub timestamp: u64,
    /// field used for giving variability
    pub nonce: u64,
    /// root of a sha256 hash tree where the leaves are transactions
    pub merkle_root: BlockHash,
    /// version of the protocol used to create the block
    pub version: u8,
}

/// Implementation of BlockchainBlock for a generic type `T`
impl<T> BlockchainBlock<T>
where
    T: Byteable,
{    
    /// Constructs a new `BlockchainBlock<T>`.
    ///
    /// # Examples
    ///
    /// Simple example with i32 data
    ///
    /// ```
    ///   extern crate blockchainblock;
    ///   use crate::blockchainblock::*;
    ///   
    ///   let prev  : Option<BlockHash> = Some([1; BLOCKHASHLEN]);
    ///   let merk  : Vec<Box<BlockchainBlock<i32>>> = vec![];
    ///   let nonce : u64 = 3;
    ///   let timestamp : u64 = 4;
    ///   let data : i32 = 5;
    ///   let block : BlockchainBlock<i32> = BlockchainBlock::new(prev, data, timestamp, nonce, &merk);
    ///   println!("\n{:?}\n", &block);
    ///   assert_eq!(block.curr_hash, [174, 98, 223, 59, 198, 22, 229, 2, 105, 113, 32, 226, 166, 118, 72, 94, 155, 43, 68, 112, 126, 155, 189, 147, 22, 204, 112, 35, 78, 209, 167, 78]);
    /// ```
    ///
    /// Example with String data
    ///
    /// ```
    ///   extern crate blockchainblock;
    ///   use crate::blockchainblock::*;
    ///       
    ///   let book_reviews = String::from(
    ///    "{
    ///     \"Adventures of Huckleberry Finn\": \"My favorite book.\",
    ///     \"Grimms' Fairy Tales\": \"Masterpiece.\",
    ///     \"Pride and Prejudice\": \"Very enjoyable.\",
    ///     \"The Adventures of Sherlock Holmes\": \"Eye lyked it alot.\",
    ///     }"
    ///   );
    /// 
    ///   let prev  : Option<BlockHash> = Some([1; BLOCKHASHLEN]);
    ///   let merk  : Vec<Box<BlockchainBlock<String>>> = vec![];
    ///   let nonce : u64 = 3;
    ///   let timestamp = std::time::Duration::from_secs(1524885322).as_secs();
    ///   let block : BlockchainBlock<String> = BlockchainBlock::new(prev, book_reviews, timestamp, nonce, &merk);
    ///     
    ///   println!("\n{:?}\n", &block);
    ///   assert_eq!(block.curr_hash, [168, 100, 204, 131, 252, 20, 69, 148, 230, 74, 165, 38, 154, 240, 27, 75, 141, 210, 40, 176, 124, 125, 180, 245, 86, 104, 17, 204, 215, 175, 198, 246]);
    /// ```

    /// Example with vector of blocks
    ///
    /// ```
    ///   extern crate blockchainblock;
    ///   use crate::blockchainblock::*;    
    ///   const MERKLE_SIZE : usize = 8;
    ///   let mut prev : Option<BlockHash> = None;
    ///   let mut merk  : Vec<Box<BlockchainBlock<String>>> = vec![];
    ///   let mut nonce : u64 = 0;
    ///   for idx in 0..MERKLE_SIZE {
    ///       let book_reviews = idx.to_string();
    ///       nonce += 1;
    ///       let timestamp = std::time::Duration::from_secs(1524885322+nonce)
    ///           .as_secs();
    ///       let block = Box::new(BlockchainBlock::new(prev, book_reviews, timestamp, nonce, &merk));
    ///       prev = Some(block.curr_hash);
    ///       merk.push(block);
    ///       println!("\n{:?}\n", merk);
    ///   }
    ///   let block = &merk[MERKLE_SIZE-1];
    ///   println!("\n{:?}\n", block);
    ///   assert_eq!(block.merkle_root, [215, 93, 242, 152, 17, 122, 84, 209, 39, 180, 167, 206, 155, 213, 206, 153, 113, 204, 119, 246, 251, 237, 192, 2, 148, 31, 149, 32, 3, 88, 18, 106]);
    /// ```
    
    pub fn new(prev_hash: Option<BlockHash>, data: T, timestamp: u64, nonce: u64, merkle_root: &Vec<Box<BlockchainBlock<T>>>) -> BlockchainBlock<T> {
        let mut block = BlockchainBlock {
            prev_hash,
            data,
            timestamp,
            merkle_root : [ 0; BLOCKHASHLEN],
            nonce,
            version : VERSION,
            curr_hash : [ 0; BLOCKHASHLEN]
        };
        block.calculate_hash();
        if merkle_root.len() > 0 { block.calculate_merkle_root(&merkle_root[..]); }
        else{ block.merkle_root = block.curr_hash; } // In the first node, the merkle root is the hash of the node
        block
    }

    fn calculate_merkle_root<'a> (&'a mut self, blocks: &'a [Box<BlockchainBlock<T>>]) -> &'a BlockHash{
        const DOUBLE_BLOCK_LEN : usize = BLOCKHASHLEN * 2;
        
        let size = blocks.len();
        if size == 1 {
            return &(blocks[0].curr_hash);
        }else{
            let (left, right) = blocks.split_at(size/2);
            let mut bytes: [u8; DOUBLE_BLOCK_LEN] = [0; DOUBLE_BLOCK_LEN];;
            bytes[..BLOCKHASHLEN].clone_from_slice(self.calculate_merkle_root(&left));
            bytes[BLOCKHASHLEN..].clone_from_slice(self.calculate_merkle_root(&right));

            let digest = digest(Algorithm::SHA256, &bytes);
            &self.merkle_root.copy_from_slice(&digest);
            return &self.merkle_root;
        }
    }

    
}

impl<T: fmt::Debug> fmt::Debug for BlockchainBlock<T>{
    fn fmt (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_struct("Block")
            .field("Current Hash", &self.curr_hash)
            .field("Previous Hash", &self.prev_hash)
            .field("Data", &self.data)
            .field("Timestamp", &self.timestamp)
            .field("Timestamp", &self.timestamp)
            .field("Nonce", &self.nonce)
            .field("Merkleroot", &self.merkle_root)
            .field("Version", &self.version)
            .finish()
    }    
}


impl<T> Hashable for BlockchainBlock<T>
where
    T: Byteable,
{
    fn calculate_hash (&mut self){
        let prev_hash_bytes = &self.prev_hash;
        let data_bytes = &self.data.bytes();
        let timestamp_bytes = &self.timestamp.to_le_bytes();
        let nonce_bytes = &self.nonce.to_le_bytes();        
        let merkle_root_bytes = &self.merkle_root;
        let version_bytes = &self.version.to_le_bytes();
        let size =
            match prev_hash_bytes { Some(prev_h) => prev_h.len(), None => 0 } +
            data_bytes.len() +
            timestamp_bytes.len() +
            nonce_bytes.len() +
            merkle_root_bytes.len() +
            version_bytes.len();
        let mut bytes : Vec<u8> = Vec::with_capacity(size);

        match prev_hash_bytes {
            Some(prev_h) => {
                for idj in 0..prev_h.len(){
                    bytes.push(prev_h[idj]);
                }
            },
            None => ()
        }
        for idj in 0..data_bytes.len(){
            bytes.push(data_bytes[idj]);
        }
        for idj in 0..timestamp_bytes.len(){
            bytes.push(timestamp_bytes[idj]);
        }
        for idj in 0..nonce_bytes.len(){
            bytes.push(nonce_bytes[idj]);
        }
        for idj in 0..merkle_root_bytes.len(){
            bytes.push(merkle_root_bytes[idj]);
        }
        for idj in 0..version_bytes.len(){
            bytes.push(version_bytes[idj]);
        }

        let digest = digest(Algorithm::SHA256, &bytes);
        &self.curr_hash.copy_from_slice(&digest);
    }
}
