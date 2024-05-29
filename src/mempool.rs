use log::warn;

use crate::{address::H160, blockchain::State, transaction::SignedTransaction as Transaction};
use std::collections::HashMap;
use crate::crypto::hash::{H256, Hashable};

/// Store all the received valid transactions which have not been included in the blockchain yet.
pub struct Mempool {
    // TODO Optional: you may use other data structures if you wish.
    pub hash_to_transaction: HashMap<H256, Transaction>,
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

    pub fn get_valid_transactions(&self, state: &State) -> Vec<Transaction> {
        
        let mut valid_transactions = Vec::new();
        for transaction in self.hash_to_transaction.values() {
            if self.is_valid(state.clone(), transaction.clone()) {
                valid_transactions.push(transaction.clone());
            }
        }
        valid_transactions
    }

    pub fn get_invalid_transactions(&self, state: &State) -> Vec<Transaction> {
        let mut invalid_transactions = Vec::new();
        for transaction in self.hash_to_transaction.values() {
            if !self.is_valid(state.clone(), transaction.clone()) {
                invalid_transactions.push(transaction.clone());
            }
        }
        invalid_transactions
    }

    pub fn is_valid(&self, state: State, transaction: Transaction) -> bool {
        // validity check
        if !transaction.verify_signature() {
            warn!("Invalid transaction detected in : Failed to verify signature");
            return false
        }

        if transaction.raw.from_addr != H160::from_pubkey(&transaction.pub_key) {
            warn!("Invalid transaction detected: Failed to match from_addr: {:?} with address of pub_key: {:?}", transaction.raw.from_addr, H160::from_pubkey(&transaction.pub_key));
            return false
        }

        let sender = H160::from_pubkey(&transaction.pub_key);
        // get the sender's nonce and balance
        let (sender_account_nonce, sender_account_balance) = state.get(&sender).unwrap();
        // check if the nonce (sender) is correct
        if sender_account_nonce + 1 != transaction.raw.nonce {  // TODO: sender_account_nonce is always 0, the nonce of sender account at the tip of the blockchain => tip never changes?
            // warn!("Invalid transaction detected: tx's nonce: {:?} should be 1 more than sender_account_nonce: {:?}", transaction.raw.nonce, sender_account_nonce);
            return false
        }
        // check if the balance is enough
        if sender_account_balance < &transaction.raw.value {
            warn!("Invalid transaction detected: Sender account doesn't have enough balance with value: {:?} and sender_account_balance: {:?}", transaction.raw.value, sender_account_balance);
            return false
        }
        return true
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