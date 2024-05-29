use log::{debug, warn};
use rand::Rng;
use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};
use crate::address::H160;
use crate::crypto::hash::{H256, Hashable};

use crate::crypto::key_pair;
use crate::network::server::Handle as ServerHandle;
use crate::transaction::{RawTransaction, SignedTransaction as Transaction};
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use crate::mempool::Mempool;
use crate::network::message::Message;
use crate::blockchain::{Blockchain};

pub struct TransactionGenerator {
    server: ServerHandle,
    mempool: Arc<Mutex<Mempool>>,
    blockchain: Arc<Mutex<Blockchain>>,
    controlled_keypair: Ed25519KeyPair,
}

impl TransactionGenerator {
    pub fn new(
        server: &ServerHandle,
        mempool: &Arc<Mutex<Mempool>>,
        blockchain: &Arc<Mutex<Blockchain>>,
        controlled_keypair: Ed25519KeyPair
    ) -> TransactionGenerator {
        TransactionGenerator {
            server: server.clone(),
            mempool: Arc::clone(mempool),
            blockchain: Arc::clone(blockchain),
            controlled_keypair,
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            self.generation_loop();
            log::warn!("Transaction Generator exited");
        });
    }

    /// Generate random transactions and send them to the server
    fn generation_loop(&self) {
        debug!("Transaction generator loop started");
        const INTERVAL_MILLISECONDS: u64 = 3000; // how quickly to generate transactions

        let sender = self.controlled_keypair.public_key().as_ref();

        let mut rng = rand::thread_rng();

        loop {
            let sender_nonce = self.blockchain.lock().unwrap().state().get(&H160::from_pubkey(sender)).unwrap().0;
            debug!("Transaction generator started for sender {:?} with nonce {:?} with blockchain tip height {:?}", H160::from_pubkey(sender), sender_nonce, self.blockchain.lock().unwrap().length_of_longest_chain());
            // sleep for some time:
            let interval = time::Duration::from_millis(INTERVAL_MILLISECONDS);
            thread::sleep(interval);

            // To demonstrate transaction is working well with the client, you need to add transactions to your running client. The transactions can be a simple payment in the account-based model. You are free to choose the sender and recipient.
            // In order to do that, you need to write a transaction generator. One recommended way to do that is to create a new thread, and generate a transaction periodically (we have provided a template in `src/transaction_generator.rs`; be sure to initialize it in `main.rs`). You may use other methods too, like writing an API in *src/api/* and call this API externally.
            // When a transaction is generated, add the transactions to mempool and broadcast the hash to the network.
            // **Note**: We do not ask you to implement transaction fees and mining rewards and the corresponding coinbase transaction for this project.
            // 1. generate some random transactions:
            let mut transactions = vec![];

            
            // simulate double spending tx's: 3 tx's with the same nonce
            for _ in 0..3 {
                let value: u64 = rng.gen_range(1, 1000);
                let transaction = Transaction::from_raw(
                    RawTransaction {
                        from_addr: H160::from_pubkey(sender),
                        to_addr: H160::from_pubkey(&key_pair::random().public_key().as_ref()),
                        // positive value
                        value,
                        nonce: sender_nonce + 1,
                    },
                    &self.controlled_keypair,
                );
                // validity check
                if !transaction.verify_signature() {
                    warn!("Invalid transaction detected in : Failed to verify signature");
                    continue;
                }

                if transaction.raw.from_addr != H160::from_pubkey(&transaction.pub_key) {
                    warn!("Invalid transaction detected: Failed to match from_addr: {:?} with address of pub_key: {:?}", transaction.raw.from_addr, H160::from_pubkey(&transaction.pub_key));
                    continue;
                }

                let sender = H160::from_pubkey(&transaction.pub_key);
                let block_hash = self.blockchain.lock().unwrap().tip();  // tip of the blockchain
                // get the sender's nonce and balance
                let blockchain = self.blockchain.lock().unwrap();
                let (sender_account_nonce, sender_account_balance) = blockchain.hash_to_state.get(&block_hash).unwrap().get(&sender).unwrap();
                // check if the nonce (sender) is correct
                if sender_account_nonce + 1 != transaction.raw.nonce {  // TODO: sender_account_nonce is always 0
                    warn!("Generated An Invalid transaction detected: tx's nonce: {:?} should be 1 more than sender_account_nonce: {:?}", transaction.raw.nonce, sender_account_nonce);
                    continue;
                }
                // check if the balance is enough
                if sender_account_balance < &transaction.raw.value {
                    warn!("Invalid transaction detected: Sender account doesn't have enough balance with value: {:?} and sender_account_balance: {:?}", transaction.raw.value, sender_account_balance);
                    continue;
                }

                debug!("Transaction generator added a valid transaction to mempool with nonce: {:?} of account: {:?} with balance: {:?} and wired amount: {:?}", transaction.raw.nonce, sender, sender_account_balance, transaction.raw.value);

                transactions.push(transaction);
            }
            debug!("Transaction generator added {} valid transactions to mempool", transactions.len());

            
            // 2. add these transactions to the mempool:
            let mut mempool = self.mempool.lock().unwrap();
            for transaction in &transactions {
                mempool.insert(transaction.clone());
            }
            debug!("# of transactions in mempool: {}", mempool.len());
            // the nonces of each account in the mempool
            // for transaction in mempool.hash_to_transaction.values() {
            //     debug!("nonce of account: {:?} in mempool: {:?}", transaction.raw.from_addr, transaction.raw.nonce);
            // }
            // debug!("# of valid transactions in mempool: {}", mempool.get_valid_transactions(&self.blockchain.lock().unwrap().state()).len());  // deadlock here?

            
            // 3. broadcast them using `self.server.broadcast(Message::NewTransactionHashes(...))`:
            if transactions.len() > 0 {
                let hashes: Vec<H256> = transactions.clone().iter().map(|tx| tx.hash()).collect();
                self.server.broadcast(Message::NewTransactionHashes(hashes));
            }
        }
    }
}
