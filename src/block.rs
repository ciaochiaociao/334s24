use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};
use crate::transaction::RawTransaction;

/// The block header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub parent: H256,
    pub nonce: u32,
    pub difficulty: H256,
    pub timestamp: u128,
    pub merkle_root: H256,
}

/// Transactions contained in a block
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub transactions: Vec<RawTransaction>,
}

/// A block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub content: Content,
}

/// Returns the default difficulty, which is a big-endian 32-byte integer.
/// - Note: a valid block must satisfy that `block.hash() <= difficulty`.
///   In other words, the _smaller_ the `difficulty`, the harder it actually is to mine a block!
fn default_difficulty() -> [u8; 32] {
    // TODO: it's up to you to determine an appropriate difficulty.
    // For example, after executing the code below, `difficulty` represents the number 256^31.
    //
    // let mut difficulty = [0u8; 32];
    // difficulty[0] = 1;
    // difficulty
    let mut difficulty = [0xFF; 32];
    let leading_zeros = 2;
    for i in 0..leading_zeros {
        difficulty[i] = 0;
    }
    // hash < 0000010000000000000000000000000000000000000000000000000000000000
    //  e.g., 000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
    // 6 leading zeros in the hexidecimal representation
    // 3 leading zeros in the byte representation
    difficulty
}

impl Hashable for Block {
    /// Hash the block header using SHA256.
    fn hash(&self) -> H256 {
        let bytes = bincode::serialize(&self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &bytes).into()
    }
}

impl Block {
    /// Construct the (totally deterministic) genesis block
    pub fn genesis() -> Block {
        let transactions: Vec<RawTransaction> = vec![];
        let header = Header {
            parent: Default::default(),
            nonce: 0,
            difficulty: default_difficulty().into(),
            timestamp: 0,
            merkle_root: Default::default(),
        };
        let content = Content { transactions };
        Block { header, content }
    }
}

#[cfg(any(test, test_utilities))]
pub mod test {
    use super::*;
    use crate::crypto::hash::H256;
    use crate::crypto::merkle::MerkleTree;

    pub fn generate_random_block(parent: &H256) -> Block {
        let transactions: Vec<RawTransaction> = vec![Default::default()];
        let root = MerkleTree::new(&transactions).root();
        let header = Header {
            parent: *parent,
            nonce: rand::random(),
            difficulty: default_difficulty().into(),
            timestamp: rand::random(),
            merkle_root: root,
        };
        let content = Content { transactions };
        Block { header, content }
    }
}
