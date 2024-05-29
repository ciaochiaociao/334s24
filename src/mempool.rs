use crate::transaction::SignedTransaction as Transaction;
use std::collections::HashMap;
use crate::crypto::hash::{H256, Hashable};

/// Store all the received valid transactions which have not been included in the blockchain yet.
pub struct Mempool {
    // TODO Optional: you may use other data structures if you wish.
    hash_to_transaction: HashMap<H256, Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            hash_to_transaction: HashMap::new(),
        }
    }

    /// Get a transaction from the mempool by hash (or `None` if it does not exist)
    pub fn get_transaction(&self, hash: &H256) -> Option<&Transaction> {
        self.hash_to_transaction.get(hash)
    }

    /// Insert a transaction into the mempool
    pub fn insert(&mut self, transaction: Transaction) {
        // (Make sure you have implemented the `Hashable` trait for `SignedTransaction`, or there will be an error):
        let hash = transaction.hash();
        self.hash_to_transaction.insert(hash, transaction);
    }

    /// Remove a random transaction from the mempool and return it (or `None` if it is empty)
    pub fn pop(&mut self) -> Option<Transaction> {
        let hash = self.hash_to_transaction.keys().next().cloned();
        if let Some(hash) = hash {
            self.hash_to_transaction.remove(&hash)
        } else {
            None
        }
    }
        
    // Contain a transaction by hash
    pub fn contains_transaction(&self, hash: &H256) -> bool {
        self.hash_to_transaction.contains_key(hash)
    }

    // Get transactions
    pub fn get_transactions(&self, hashes: &[H256]) -> Vec<Transaction> {
        hashes.iter().filter_map(|hash| self.get_transaction(hash).cloned()).collect()
    }

    // Get a number of transactions from the mempool
    pub fn get_n_transactions(&self, number: usize) -> Vec<Transaction> {
        self.hash_to_transaction.values().cloned().take(number).collect()
    }

    // Remove transactions from the mempool
    pub fn remove_transactions(&mut self, hashes: &[H256]) {
        for hash in hashes {
            self.hash_to_transaction.remove(hash);
        }
    }

    // Get the number of transactions in the mempool
    pub fn len(&self) -> usize {
        self.hash_to_transaction.len()
    }

}