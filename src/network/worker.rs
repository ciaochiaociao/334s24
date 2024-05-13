use super::message::Message;
use super::peer;
use crate::network::server::Handle as ServerHandle;
use crate::blockchain::{Blockchain, BlockOrigin};
use crate::block::Block;
use crate::crypto::hash::Hashable;
use std::sync::{Arc, Mutex};
use crossbeam::channel;
use log::{debug, warn};

use std::thread;

#[derive(Clone)]
pub struct Context {
    msg_chan: channel::Receiver<(Vec<u8>, peer::Handle)>,
    num_worker: usize,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
}

pub fn new(
    num_worker: usize,
    msg_src: channel::Receiver<(Vec<u8>, peer::Handle)>,
    server: &ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Context {
    Context {
        msg_chan: msg_src,
        num_worker: num_worker,
        server: server.clone(),
        blockchain: blockchain,
    }
}

impl Context {
    pub fn start(self) {
        let num_worker = self.num_worker;
        for i in 0..num_worker {
            let cloned = self.clone();
            thread::spawn(move || {
                cloned.worker_loop();
                warn!("Worker thread {} exited", i);
            });
        }
    }


    fn handle_orphans(&self, block: Block) {
        // 3.3. Orphan block handler
        // Check if the newly processed block is a parent to any blocks in the orphan buffer.                         
        // If that is the case, remove the blocks from the orphan buffer and insert them one by one. 
        // Repeat this process until no more orphan blocks can be processed.
        // Note that inserting a block can make other orphan blocks, i.e. its children, ready to be inserted consequently. This step should be repeated until no more orphan blocks can be processed (e.g., using a recursive function, or a loop or whatever).
        

        let found_orphans = self.blockchain.lock().unwrap().get_orphans(&block.hash());  // get the "found" orphans of this parent block
        self.blockchain.lock().unwrap().remove_orphans(&block.hash());
        // if !found_orphans.is_empty() {
        //     println!("Found Orphans: {:?}", found_orphans);
        // }
        for orphan in found_orphans {
            // assert that the parent block is already in the blockchain
            assert!(self.blockchain.lock().unwrap().contains_block(&orphan.header.parent));
            self.blockchain.lock().unwrap().insert(&orphan);
            self.handle_orphans(orphan);  // this orphan might also be a parent to some orphans, so we need to check recursively
        }
    }

    fn worker_loop(&self) {
        loop {
            let msg = self.msg_chan.recv().unwrap();
            let (msg, peer) = msg;
            let msg: Message = bincode::deserialize(&msg).unwrap();
            match msg {
                Message::Ping(nonce) => {
                    debug!("Ping: {}", nonce);
                    peer.write(Message::Pong(nonce.to_string()));
                }
                Message::Pong(nonce) => {
                    debug!("Pong: {}", nonce);
                }
                // 1. NewBlockHashes(Vec\<H256\>), similar to *inv* in lectures
                // 2. GetBlocks(Vec\<H256\>), similar to *getdata* in lectures
                // 3. Blocks(Vec\<Block\>), similar to *block* in lectures
                Message::NewBlockHashes(hashes) => {
                    // Received **NewBlockHashes** message from other peers.
                    // This message will either originate from the miner when it successfully mines a block or be received from another peer relaying the blocks.

                    // Upon receiving **NewBlockHashes**, if the hashes are not already in blockchain, you need to ask for them by sending **GetBlocks**.
                    
                    debug!("Message::NewBlockHashes: {:?}", hashes);
                    let mut new_hashes = Vec::new();
                    for hash in hashes {
                        if !self.blockchain.lock().unwrap().contains_block(&hash) {
                            new_hashes.push(hash);
                        }
                    }
                    if !new_hashes.is_empty() {
                        self.server.broadcast(Message::GetBlocks(new_hashes.clone()));
                    }
                }
                Message::GetBlocks(hashes) => {
                    // Received **GetBlocks** message from other peers.
                    // Upon receiving **GetBlocks**, if the hashes are in blockchain, you can get these blocks and send them by **Blocks** message.

                    debug!("Message::GetBlocks: {:?}", hashes);
                    let blocks = self
                        .blockchain
                        .lock()
                        .unwrap()
                        .get_blocks(&hashes);

                    if !blocks.is_empty() {
                        // println!("Get Blocks: {:?}", blocks);
                        self.server.broadcast(Message::Blocks(blocks));
                    }
                }
                Message::Blocks(blocks) => {
                    // Received blocks from other peers.
                    //- Check if each block is already in the blockchain. If so, skip that block; otherwise, check if that block is valid before inserting it into blockchain. We will discuss the validity checks in the following subsections.
                    //- Finally, you need to broadcast **NewBlockHashes** message when receiving new blocks in **Blocks** message. **NewBlockHashes** message should contain hashes of blocks newly received and accepted.
                    debug!("Message::Blocks");
                    // print out the time when the new block hashes are received
                    let now = std::time::SystemTime::now();
                    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                    // println!("Received new block hashes at: {:?}", since_the_epoch);
                    // print out the delay between the time when the new block hashes are received and the time when the new block is created
                    let delay = since_the_epoch.as_millis() - blocks[0].clone().header.timestamp;
                    // println!("Block creation time (milliseconds): {:?}", blocks[0].clone().header.timestamp);
                    // println!("Received new block hashes at: {:?}", since_the_epoch.as_millis());
                    // println!("Delay (milliseconds): {:?}", delay);
                    let mut new_hashes = Vec::new();
                    for block in blocks {
                        if self.blockchain.lock().unwrap().contains_block(&block.hash()) {
                            continue;
                        }
                        // check if the block is valid before inserting it into blockchain
                        // 3.1. PoW validity check
                        // - `block.hash() <= difficulty`. (Note that difficulty is a misnomer here since a higher 'difficulty' here means that the block is easier to mine).
                        // - Difficulty in the block header is consistent with your view. We have a fixed mining difficulty for this project, thus, this would just involve checking if the difficulty equals the genesis block's difficulty.
                        
                        // If the check fails, it indicates that the block is corrupted or dishonest. You should ignore the block instead of adding it to your blockchain.
                        if block.hash() > Block::genesis().header.difficulty {  // failed PoW check
                            warn!("Invalid block detected: {:?}", block);
                            continue;
                        }

                        // propagate valid blocks (even for orphan blocks, we need to propagate them to other peers, so that we could ask the other peers to find the parent block)
                        new_hashes.push(block.hash().clone());

                        // 3.2. Parent block existence check
                        // - Check if the block's parent exists in your local copy of your blockchain, if the parent exists, insert the block into your blockchain.
                        // - If this check fails, you need to add the block in an 'orphan buffer'. The buffer stores the blocks whose parent is not seen yet. Also, you need to send **GetBlocks** message, containing this parent hash.

                        let parent_hash = block.header.parent;
                        if self.blockchain.lock().unwrap().contains_block(&parent_hash) {
                            self.blockchain.lock().unwrap().insert(&block);  // insert the block into your blockchain
                            // 3.3. Orphan block handler: this block might be a parent to some orphans
                            self.handle_orphans(block.clone());
                        } else {
                            warn!("Orphan block detected: {:?}", block);
                            self.blockchain.lock().unwrap().insert_orphan(parent_hash, block.clone());
                            self.server.broadcast(Message::GetBlocks(vec![parent_hash]));  // to look for this orphan's parent; maybe it is in other peers
                        }
                        
                        // set the delay time for each block
                        let origin_received = BlockOrigin::Received { delay_ms: delay };
                        self.blockchain.lock().unwrap().hash_to_origin.insert(block.hash(), origin_received);
                    }
                    if !new_hashes.is_empty() {
                        self.server.broadcast(Message::NewBlockHashes(new_hashes.clone()));  // propagate the new block hashes to other peers
                    }
                }

            }
        }
    }
}
