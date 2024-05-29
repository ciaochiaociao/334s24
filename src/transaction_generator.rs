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
        const INTERVAL_MILLISECONDS: u64 = 3000; // how quickly to generate transactions

        let sender = self.controlled_keypair.public_key().as_ref();
        let sender_nonce = self.blockchain.lock().unwrap().state().get(&H160::from_pubkey(sender)).unwrap().0;

        let mut nonce = sender_nonce;
        let mut rng = rand::thread_rng();

        loop {
            // sleep for some time:
            let interval = time::Duration::from_millis(INTERVAL_MILLISECONDS);
            thread::sleep(interval);

            // To demonstrate transaction is working well with the client, you need to add transactions to your running client. The transactions can be a simple payment in the account-based model. You are free to choose the sender and recipient.
            // In order to do that, you need to write a transaction generator. One recommended way to do that is to create a new thread, and generate a transaction periodically (we have provided a template in `src/transaction_generator.rs`; be sure to initialize it in `main.rs`). You may use other methods too, like writing an API in *src/api/* and call this API externally.
            // When a transaction is generated, add the transactions to mempool and broadcast the hash to the network.
            // **Note**: We do not ask you to implement transaction fees and mining rewards and the corresponding coinbase transaction for this project.
            // 1. generate some random transactions:
            let mut txs = vec![];

            
            for _ in 0..10 {
                nonce += 1;
                let value = rng.gen_range(1, 1000);
                let tx = Transaction::from_raw(
                    RawTransaction {
                        from_addr: H160::from_pubkey(sender),
                        to_addr: H160::from_pubkey(&key_pair::random().public_key().as_ref()),
                        // positive value
                        value,
                        nonce,
                    },
                    &self.controlled_keypair,
                );
                txs.push(tx);
            }

            
            // 2. add these transactions to the mempool:
            let mut mempool = self.mempool.lock().unwrap();
            for tx in &txs {
                mempool.insert(tx.clone());
            }
            
            // 3. broadcast them using `self.server.broadcast(Message::NewTransactionHashes(...))`:
            let hashes: Vec<H256> = txs.clone().iter().map(|tx| tx.hash()).collect();
            self.server.broadcast(Message::NewTransactionHashes(hashes));
        }
    }
}
