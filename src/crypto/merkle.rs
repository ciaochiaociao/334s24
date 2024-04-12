use ring::digest;

use super::hash::{Hashable, H256};

/// helper functions
fn hash_children(left: &H256, right: &H256) -> H256 {
    // concatenate the left and right children, then hash the result
    digest::digest(&digest::SHA256, &[left.as_ref(), right.as_ref()].concat()).into()
}


fn duplicate_last_node(curr_level: &mut Vec<H256>) {
    let last_node = curr_level.last().unwrap().clone();
    curr_level.push(last_node);
}


/// A Merkle tree with binary array representation.
#[derive(Debug, Default)]
pub struct MerkleTree {
    pub array: Vec<H256>,
    level_count: usize, // how many levels the tree has
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self where T: Hashable, {
        assert!(!data.is_empty());

        // create the leaf nodes:
        let mut curr_level: Vec<H256> = Vec::new();
        for item in data {
            curr_level.push(item.hash());
        }
        let mut level_count = 1;
        let mut array: Vec<H256> = curr_level.clone();


        // create the upper levels of the tree and prepend the nodes of each level to the front of the array
        while curr_level.len() > 1 {
            // Whenever a level of the tree has odd number of nodes, duplicate the last node to make the number even:
            if curr_level.len() % 2 == 1 {
                duplicate_last_node(&mut curr_level);
            }
            assert!(curr_level.len() % 2 == 0); // make sure we now have even number of nodes.



            // Bottom-up construction of the Merkle tree
            let mut next_level: Vec<H256> = Vec::new();
            // println!("Doing the loop of the level: {}", level_count);
            // println!("curr_level length: {}", curr_level.len());
            for i in 0 .. curr_level.len() / 2 {
                // println!("node {} and {}", i * 2, i * 2 + 1);
                let left = curr_level[i * 2];
                let right = curr_level[i * 2 + 1];
                let hash = hash_children(&left, &right);
                next_level.push(hash);
            }
            
            // prepend the nodes of the current level to the front of the array
            // println!("length of the current level: {}", next_level.len());
            // println!("length of the array: {}", array.len());
            let mut array2= next_level.clone();
            array2.extend_from_slice(&array.clone());
            // println!("length of the array2: {}", array2.len());
            array = array2;

            curr_level = next_level;
            level_count += 1;
        }

        // Create a MerkleTree instance with the root node and the level count.
        MerkleTree {
            array: array,
            level_count: level_count,
        }
    }

    pub fn root(&self) -> H256 {
        println!("array: {:?}", self.array);
        println!("len(array): {}", self.array.len());
        self.array[0]
    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        let mut proof: Vec<H256> = Vec::new();

        // 2**(level_count - 1) - 1 is the index of the first node in the bottom level
        let mut i = index + 2usize.pow(self.level_count as u32 - 1) - 1;
        println!("i in the array (for proof index): {}", i);
        let mut j = 0;
        while j < self.level_count - 1 {

            // get the sibling node
            // println!("len(array): {}", self.array.len());
            println!("j: {}", j);
            if i % 2 == 0 {
                proof.push(self.array[i - 1]);
            } else {
                proof.push(self.array[i + 1]);
            }

            // move to the parent node
            i = (i - 1) / 2;
            j += 1;
        }
        proof
    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
    
    // convert the index to binary to know either the left or right child is the current node
    // the binary index is the path from the leaf to the root
    let mut binary_index: Vec<usize> = Vec::new();
    let level = (leaf_size as f64).log2().ceil() as usize + 1;
    let mut i = index + 2usize.pow(level as u32 - 1) - 1;
    while i > 0 {
        i -= 1;
        binary_index.push(i % 2);
        i /= 2;
    }

    // verify the proof by hashing the current node with the sibling node along the path up to the root
    let mut curr_hash = *datum;
    for j in 0..proof.len() {
        if binary_index[j] == 0 {
            curr_hash = hash_children(&curr_hash, &proof[j]);
        } else {
            curr_hash = hash_children(&proof[j], &curr_hash);
        }
    }

    // compare the computed hash with the root hash
    *root == curr_hash

}

#[cfg(test)]
mod tests {
    use crate::crypto::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    macro_rules! gen_merkle_tree_middle {
        () => {{
            vec![
                (hex!("0000000000000000000000000000000000000000000000000000000000000011")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000022")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000033")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000044")).into(),
            ]
        }}
    }

    macro_rules! gen_merkle_tree_large {
        () => {{
            vec![
                (hex!("0000000000000000000000000000000000000000000000000000000000000011")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000022")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000033")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000044")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000055")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000066")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000077")).into(),
                (hex!("0000000000000000000000000000000000000000000000000000000000000088")).into(),
            ]
        }};
    }
  
    #[test]
    fn root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn tree_size() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        assert_eq!(merkle_tree.array.len(), 3);
        assert_eq!(merkle_tree.level_count, 2);
    }

    #[test]
    fn tree_size2() {
        let input_data: Vec<H256> = gen_merkle_tree_middle!();
        let merkle_tree = MerkleTree::new(&input_data);
        assert_eq!(merkle_tree.array.len(), 7);
        assert_eq!(merkle_tree.level_count, 3);
    }

    #[test]
    fn whole_tree() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        assert_eq!(
            merkle_tree.array[1..],
            vec![
                (hex!("b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0")).into(),
                (hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")).into(),
            ]
        )
    }

    // #[test]
    // fn whole_tree2() {
    //     let input_data: Vec<H256> = gen_merkle_tree_middle!();
    //     let merkle_tree = MerkleTree::new(&input_data);
    //     assert_eq!(
    //         merkle_tree.array[1..],
    //         vec![
    //             (hex!("
    // }

    #[test]
    fn proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn proof_tree_large() {
        let input_data: Vec<H256> = gen_merkle_tree_large!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(5);
  
        // We accept the proof in either the top-down or bottom-up order; you should stick to either of them.
        let expected_proof_bottom_up: Vec<H256> = vec![
            (hex!("c8c37c89fcc6ee7f5e8237d2b7ed8c17640c154f8d7751c774719b2b82040c76")).into(),
            (hex!("bada70a695501195fb5ad950a5a41c02c0f9c449a918937267710a0425151b77")).into(),
            (hex!("1e28fb71415f259bd4b0b3b98d67a1240b4f3bed5923aa222c5fdbd97c8fb002")).into(),
        ];
        let expected_proof_top_down: Vec<H256> = vec![
            (hex!("1e28fb71415f259bd4b0b3b98d67a1240b4f3bed5923aa222c5fdbd97c8fb002")).into(),  
            (hex!("bada70a695501195fb5ad950a5a41c02c0f9c449a918937267710a0425151b77")).into(),
            (hex!("c8c37c89fcc6ee7f5e8237d2b7ed8c17640c154f8d7751c774719b2b82040c76")).into(),
        ];
        println!("proof: {:?}", proof);
        assert!(proof == expected_proof_bottom_up || proof == expected_proof_top_down);
    }
    
    #[test]
    fn verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }

    #[test]
    fn verifying_tree_large() {
        let input_data: Vec<H256> = gen_merkle_tree_large!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(5);
        assert!(verify(&merkle_tree.root(), &input_data[5].hash(), &proof, 5, input_data.len()));
    }
    
    #[test]
    fn verifying_tree_large_all_nodes() {
        let input_data: Vec<H256> = gen_merkle_tree_large!();
        let merkle_tree = MerkleTree::new(&input_data);
        for i in 0..input_data.len() {
            let proof = merkle_tree.proof(i);
            assert!(verify(&merkle_tree.root(), &input_data[i].hash(), &proof, i, input_data.len()));
        }
    }
}
