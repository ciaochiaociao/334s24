use core::hash;
use std::collections::HashMap;

use crate::block::Block;
use crate::crypto::hash::{Hashable, H256};

pub struct Blockchain {
    tip: H256,
    // track the length of each block by using HashMap
    hash_to_length: HashMap<H256, u64>,
    // use a **HashMap** in standard crate to store blocks
    hash_to_block: HashMap<H256, Block>,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        let genesis = Block::genesis();
        let genesis_hash = genesis.hash();
        let mut hash_to_block = HashMap::new();
        // track the length of each block
        let mut hash_to_length = HashMap::new();
        hash_to_length.insert(genesis_hash, 0);
        hash_to_block.insert(genesis_hash, genesis);
        Blockchain {
            tip: genesis_hash,
            hash_to_length: hash_to_length,
            hash_to_block: hash_to_block,
        }
    }

    /// Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        let block_hash = block.hash();
        self.hash_to_block.insert(block_hash, block.clone());
        let parent_hash = block.header.parent;
        let length = self.hash_to_length.get(&parent_hash).unwrap() + 1;
        self.hash_to_length.insert(block_hash, length);
        if length > *self.hash_to_length.get(&self.tip).unwrap() {
            self.tip = block_hash;
        }
        
    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        self.tip
    }

    /// Get the last block's hash of the longest chain
    #[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        // init with a vector of Blocks
        let mut blocks: Vec<Block> = vec![];
        let mut current_hash = self.tip;
        // loop until the genesis block
        while current_hash != Block::genesis().hash() {
            let block = self.hash_to_block.get(&current_hash).unwrap();
            blocks.push(block.clone());
            current_hash = block.header.parent;
        }
        blocks.iter().map(|block| block.hash()).collect()
    }
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;
    use crate::crypto::hash::Hashable;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());

    }
}
