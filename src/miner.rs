use crate::blockchain;
use crate::network::server::Handle as ServerHandle;
use crate::blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use crate::transaction::RawTransaction;
use crate::block::{Block, Header, Content};
use crate::crypto::hash::Hashable;

use log::info;

use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::time::{self, SystemTime};

use std::thread;

enum ControlSignal {
    Start(u64), // the number controls the lambda of interval between block generation
    Exit,
}

enum OperatingState {
    Paused,
    Run(u64),
    ShutDown,
}

pub struct Context {
    /// Channel for receiving control signal
    control_chan: Receiver<ControlSignal>,
    operating_state: OperatingState,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
    start_time: Option<SystemTime>,
    total_blocks_mined: u64,
    // memory_pool: Arc<Mutex<Vec<Mempool>>>,
}

#[derive(Clone)]
pub struct Handle {
    /// Channel for sending signal to the miner thread
    control_chan: Sender<ControlSignal>,
}

pub fn new(
    server: &ServerHandle, blockchain: &Arc<Mutex<Blockchain>>
) -> (Context, Handle) {
    let (signal_chan_sender, signal_chan_receiver) = unbounded();

    let ctx = Context {
        control_chan: signal_chan_receiver,
        operating_state: OperatingState::Paused,
        server: server.clone(),
        blockchain: blockchain.clone(),
        start_time: None,
        total_blocks_mined: 0,
    };

    let handle = Handle {
        control_chan: signal_chan_sender,
    };

    (ctx, handle)
}

impl Handle {
    pub fn exit(&self) {
        self.control_chan.send(ControlSignal::Exit).unwrap();
    }

    pub fn start(&self, lambda: u64) {
        self.control_chan
            .send(ControlSignal::Start(lambda))
            .unwrap();
    }

}

impl Context {
    pub fn start(mut self) {
        thread::Builder::new()
            .name("miner".to_string())
            .spawn(move || {
                self.miner_loop();
            })
            .unwrap();
        info!("Miner initialized into paused mode");
    }

    fn handle_control_signal(&mut self, signal: ControlSignal) {
        match signal {
            ControlSignal::Exit => {
                info!("Miner shutting down");
                self.operating_state = OperatingState::ShutDown;
                // print mining stats if the miner started:
                if let Some(start_time) = self.start_time {
                    let seconds_spent = SystemTime::now().duration_since(start_time).unwrap().as_secs_f64();
                    let mining_rate = (self.total_blocks_mined as f64) / seconds_spent;
                    info!("Mined {} blocks in {} seconds, rate is {} blocks/second",
                        self.total_blocks_mined, seconds_spent, mining_rate)
                }
            }
            ControlSignal::Start(i) => {
                info!("Miner starting in continuous mode with lambda {}", i);
                self.operating_state = OperatingState::Run(i);
                // set the miner start time:
                if self.start_time == None {
                    self.start_time = Some(SystemTime::now());
                }
            }
        }
    }

    fn miner_loop(&mut self) {
        // main mining loop
        let start_time = time::SystemTime::now();
        loop {
            // check and react to control signals
            match self.operating_state {
                OperatingState::Paused => {
                    let signal = self.control_chan.recv().unwrap();
                    self.handle_control_signal(signal);
                    continue;
                }
                OperatingState::ShutDown => {
                    let signal = self.control_chan.recv().unwrap();
                    self.handle_control_signal(signal);
                    return;
                }
                _ => match self.control_chan.try_recv() {
                    Ok(signal) => {
                        self.handle_control_signal(signal);
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => panic!("Miner control channel detached"),
                },
            }
            if let OperatingState::ShutDown = self.operating_state {
                let signal = self.control_chan.recv().unwrap();
                self.handle_control_signal(signal);
                return;
            }

            // TODO: actual mining
            if let OperatingState::Run(i) = self.operating_state {
                let mut blockchain = self.blockchain.lock().unwrap();
                // Next, to build a block, you need to gather a block's fields. In a block header, the fields are gathered as follows,
                // 1. parent - use *blockchain.tip()*
                let parent = blockchain.tip();
                // 2. timestamp - use `SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()` from `std::time`. This expression is pretty self-explanatory, except `UNIX_EPOCH` refers to 1970-01-01 00:00:00 UTC, and `millis` is short for _milliseconds_.
                // You can refer [this document](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html) for more information.
                let timestamp = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
                // 3. difficulty - in real life, it is computed from parent and ancestor blocks with some adaptive rule. In this project, we use the simple rule: a static/constant difficulty. This rule just means the difficulty of this block should be the same with that of parent block. You should be able to get parent block's difficulty from blockchain.
                let parent_block = blockchain.hash_to_block.get(&parent).unwrap();
                let difficulty = parent_block.header.difficulty;
                // 4. merkle root - compute it by creating a merkle tree from the content.
                let merkle_root = parent_block.header.merkle_root;
                drop(blockchain);

                loop {
                    let mut blockchain = self.blockchain.lock().unwrap();
                    // 5. nonce - generate a random nonce (use *rand* crate) in every iteration, or increment nonce (say, increment by 1) in every iteration. P.S. Do you think there is any difference in terms of the probability of solving the puzzle?
                    let nonce = rand::random::<u32>();
        
                    // As for the block content, you can put arbitrary content, since in this step we don't have memory pool yet. You can put an empty vector, or some random transactions.
                    let transactions: Vec<RawTransaction> = vec![];
                    let header = Header {
                        parent,
                        nonce,
                        difficulty,
                        timestamp,
                        merkle_root,
                    };
                    let content = Content {
                        transactions,
                    };
                    let new_block = Block {
                        header,
                        content,
                    };
        
                    // After you have all these fields and build a block, just check whether the proof-of-work hash puzzle is satisfied by
                    // ```
                    // block.hash() <= difficulty
                    // ```
                    // Notice that the above code is conceptually the same as *H(nonce|block) < threshold* in lectures.
                    // If it is satisfied, the block is successfully generated. Congratulations! Just insert it into blockchain, and keep on mining for another block.
        
                    if new_block.hash() <= difficulty {
                        
                        blockchain.insert(&new_block);
                        info!("Block mined: {:?}", new_block);
                        self.total_blocks_mined += 1;
                        break;
                    }
                }
                if i != 0 {
                    let interval = time::Duration::from_micros(i as u64);
                    thread::sleep(interval);
                }
            }
            info!("Length of blockchain: {}", self.blockchain.lock().unwrap().hash_to_block.len());
            let end_time = time::SystemTime::now();
            let elapsed = end_time.duration_since(start_time).unwrap();
            info!("Elapsed time: {:?}", elapsed);
            info!("Mining rate: {:?}", self.blockchain.lock().unwrap().hash_to_block.len() as f64 / elapsed.as_secs_f64());
        }
    }
}
