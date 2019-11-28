use crypto_hash::{Algorithm, digest};
use std::fmt;
use super::*;

/// Structure for storing one Block of the Blockchain.
pub struct BlockchainBlock<'a, T>{
    /// hash of the current block
    pub curr_hash: BlockHash,
    /// hash of the previous block. Is `None` for the first block
    pub prev_hash: Option<BlockHash>,
    /// encrypted data in compressed form
    pub data: &'a[T],
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
impl<'a, T> BlockchainBlock<'a, T>
where
    T: Byteable + Clone
{    
    /// Constructs a new `BlockchainBlock<T>`.
    ///
    /// # Examples
    ///
    /// Simple example with i32
    ///
    /// ```
    ///   extern crate blockchainblock;
    ///   use crate::blockchainblock::*;
    ///   
    ///   let prev  : Option<BlockHash> = None;
    ///   let nonce : u64 = 3;
    ///   let timestamp : u64 = 4;
    ///   let data : [i32; 1] = [5];
    ///   let block : BlockchainBlock<i32> = BlockchainBlock::new(prev, &data, timestamp, nonce);
    ///   println!("\n{:?}\n", &block);
    ///   assert_eq!(block.curr_hash, [23, 105, 91, 179, 190, 192, 178, 189, 198, 134, 87, 143, 214, 135, 93, 17, 50, 143, 192, 3, 254, 144, 115, 123, 42, 223, 197, 199, 181, 113, 224, 123]);
    /// ```
    ///
    /// Example with array of Strings
    ///
    /// ```
    /// extern crate blockchainblock;
    /// use crate::blockchainblock::*;
    ///     
    /// let book_reviews = [
    ///  String::from(
    ///   "{
    ///    \"Adventures of Huckleberry Finn\": \"My favorite book.\",
    ///    \"Grimms' Fairy Tales\": \"Masterpiece.\",
    ///    \"Pride and Prejudice\": \"Very enjoyable.\",
    ///    \"The Adventures of Sherlock Holmes\": \"Eye lyked it alot.\",
    ///    }"),
    ///  String::from(
    ///   "{
    ///    \"Adventures of Huckleberry Finn\": \"My favorite book.\",
    ///    \"Grimms' Fairy Tales\": \"Masterpiece.\",
    ///    \"Pride and Prejudice\": \"Very enjoyable.\",
    ///    \"The Adventures of Sherlock Holmes\": \"Eye lyked it alot.\",
    ///    }")
    /// ];
     
    /// let prev  : Option<BlockHash> = Some([1; BLOCKHASHLEN]);
    /// let nonce : u64 = 3;
    /// let timestamp = std::time::Duration::from_secs(1524885322).as_secs();
    /// let block : BlockchainBlock<String> = BlockchainBlock::new(prev, &book_reviews, timestamp, nonce);
    ///   
    /// println!("\n{:?}\n", &block);
    /// assert_eq!(block.curr_hash, [220, 149, 236, 219, 173, 29, 131, 71, 35, 245, 97, 228, 58, 247, 45, 86, 197, 104, 26, 236, 232, 98, 144, 4, 220, 210, 177, 17, 235, 113, 214, 18]);
    /// ```
   
    pub fn new(prev_hash: Option<BlockHash>, data: &[T], timestamp: u64, nonce: u64) -> BlockchainBlock<T> {
        let mut block = BlockchainBlock {
            prev_hash,
            data,
            timestamp, 
            merkle_root : [ 0; BLOCKHASHLEN],
            nonce,
            version : VERSION,
            curr_hash : [ 0; BLOCKHASHLEN]
        };
        if data.len() > 0 { block.merkle_root = block.calculate_merkle_root(data); }
        block.calculate_hash();        
        block
    }

    /// Check data is inside the block calculating the new merkle root
    ///
    /// # Examples
    ///    
    /// Example checking String is inside the Block
    ///
    /// ```
    /// extern crate blockchainblock;
    /// use crate::blockchainblock::*;    
    /// let string_check = String::from(
    ///     "{\"The Adventures of Sherlock Holmes\",\"Grimms' Fairy Tales\"}"
    /// );
    /// let book_reviews = [
    ///     String::from(
    ///         "{\"Adventures of Huckleberry Finn\",\"Grimms' Fairy Tales\"}"
    ///     ),
    ///     String::from(
    ///         "{\"Eloquent JavaScript, Second Edition\",\"Learning JavaScript Design Patterns\"}"
    ///     ),
    ///     string_check.clone(),
    ///     String::from(
    ///         "{\"Speaking JavaScript\",\"Programming JavaScript Applications\"}"
    ///     ),
    /// ];
    /// 
    /// let prev  : Option<BlockHash> = None;
    /// let nonce : u64 = 1;
    /// let timestamp = std::time::Duration::from_secs(1524885322).as_secs();
    /// let block : BlockchainBlock<String> = BlockchainBlock::new(prev, &book_reviews, timestamp, nonce);
    ///    
    /// println!("\n{:?}\n", &block);
    /// assert_eq!(block.check_value_inblock(&string_check,2), true);
    /// ```

    pub fn check_value_inblock(&self, data: &T, position: usize) -> bool{
        if position >= self.data.len(){ return false; }
        let mut temp = self.data.to_vec();
        temp[position] = data.clone();
        if self.calculate_merkle_root(&temp[..]) == self.merkle_root { return true; }
        else{ return false; }
    }
    
    fn calculate_merkle_hash<'b>(&self, block_left: &'b BlockHash, block_right: &'b BlockHash) -> BlockHash{
        const DOUBLE_BLOCK_LEN : usize = BLOCKHASHLEN * 2;
        let mut bytes: [u8; DOUBLE_BLOCK_LEN] = [0; DOUBLE_BLOCK_LEN];
        
        bytes[..BLOCKHASHLEN].clone_from_slice(block_left);
        bytes[BLOCKHASHLEN..].clone_from_slice(block_right);
        let digest = digest(Algorithm::SHA256, &bytes);
        let mut result: BlockHash = [0; BLOCKHASHLEN];
        result.copy_from_slice(&digest);
        return result;
    }

    fn calculate_merkle_root (&self, blocks: &[T]) -> BlockHash{
        
        let size = blocks.len();
        match size {
            1 | 2 => {
                let mut bytes : Vec<u8> = Vec::new();
                if size == 1 {
                    bytes.append(&mut blocks[0].bytes());
                    bytes.append(&mut blocks[0].bytes());
                } else {
                    bytes.append(&mut blocks[0].bytes());
                    bytes.append(&mut blocks[1].bytes());
                }
                let digest = digest(Algorithm::SHA256, &bytes);
                let mut result: BlockHash = [0; BLOCKHASHLEN];
                result.copy_from_slice(&digest);
                return result;
            },
            _ => {
                let (left, right) = blocks.split_at(size/2);
                return self.calculate_merkle_hash(
                    &self.calculate_merkle_root(&left),
                    &self.calculate_merkle_root(&right));
            },
        }
    }
    
}

impl<'a, T: fmt::Debug> fmt::Debug for BlockchainBlock<'a, T>{
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


impl<'a, T> Hashable for BlockchainBlock<'a, T>
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

// #[test]
// fn test() {
  
// }

