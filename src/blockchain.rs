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

    #[test]
    fn mp1_insert_chain() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let mut block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
        for _ in 0..50 {
            let h = block.hash();
            block = generate_random_block(&h);
            blockchain.insert(&block);
            assert_eq!(blockchain.tip(), block.hash());
        }
    }

    #[test]
    fn mp1_insert_3_fork_and_back() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block_1 = generate_random_block(&genesis_hash);
        blockchain.insert(&block_1);
        assert_eq!(blockchain.tip(), block_1.hash());
        let block_2 = generate_random_block(&block_1.hash());
        blockchain.insert(&block_2);
        assert_eq!(blockchain.tip(), block_2.hash());
        let block_3 = generate_random_block(&block_2.hash());
        blockchain.insert(&block_3);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_1 = generate_random_block(&block_2.hash());
        blockchain.insert(&fork_block_1);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_2 = generate_random_block(&fork_block_1.hash());
        blockchain.insert(&fork_block_2);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_4 = generate_random_block(&block_3.hash());
        blockchain.insert(&block_4);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_5 = generate_random_block(&block_4.hash());
        blockchain.insert(&block_5);
        assert_eq!(blockchain.tip(), block_5.hash());
    }

}
