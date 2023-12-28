//The provided code is a Rust implementation of a service called `EvmRecorderService` that listens for incoming EVM (Ethereum Virtual Machine) blocks, writes the block headers and transactions to a blockstore, and can be gracefully stopped using an exit signal.

//Let's break down the code and its functionality:

// 1. The `EvmRecorderService` struct contains a single field `thread_hdl` of type `JoinHandle<()>`, which represents a handle to the spawned thread responsible for processing EVM blocks.

//2. The `new` function is an associated function of `EvmRecorderService` used to create a new instance of the service. It takes three parameters:
//- `evm_recorder_receiver`: A receiver channel for receiving EVM blocks.
//- `blockstore`: An atomic reference-counted pointer to a `Blockstore` instance, which is used to store EVM block headers and transactions.
//- `exit`: An atomic boolean flag used to signal the service to gracefully exit.

//3. Inside the `new` function, a new thread is spawned using `Builder::new().spawn(move || {...})`. The closure passed to `spawn` contains the main loop for processing incoming EVM blocks. The loop continues until the `exit` flag is set to true, at which point it breaks out of the loop and the thread terminates.

//4. The `write_evm_record` function is a private helper function used to write EVM block headers and transactions to the blockstore. It takes two parameters:
//- `evm_records_receiver`: A reference to the EVM block receiver channel.
//- `blockstore`: An atomic reference-counted pointer to the `Blockstore` instance.

//5. Inside the `write_evm_record` function, it attempts to receive an EVM block from the receiver channel with a timeout of 1 second using `recv_timeout`. If a block is received, it extracts the block header and transactions, and writes them to the blockstore using methods like `write_evm_block_header` and `write_evm_transaction`.

//6. The `join` function is used to wait for the spawned thread to finish and retrieve the result. It returns a `thread::Result<()>`, indicating whether the thread has completed successfully or with an error.

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use nexis_ledger::blockstore::Blockstore;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, Builder, JoinHandle},
    time::Duration,
};

use evm_state::Block;

pub type EvmRecorderReceiver = Receiver<Block>;
pub type EvmRecorderSender = Sender<Block>;

pub struct EvmRecorderService {
    thread_hdl: JoinHandle<()>,
}

impl EvmRecorderService {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        evm_recorder_receiver: EvmRecorderReceiver,
        blockstore: Arc<Blockstore>,
        exit: &Arc<AtomicBool>,
    ) -> Self {
        let exit = exit.clone();
        let thread_hdl = Builder::new()
            .name("evm-block-writer".to_string())
            .spawn(move || loop {
                if exit.load(Ordering::Relaxed) {
                    break;
                }
                if let Err(RecvTimeoutError::Disconnected) =
                    Self::write_evm_record(&evm_recorder_receiver, &blockstore)
                {
                    break;
                }
            })
            .unwrap();
        Self { thread_hdl }
    }

    fn write_evm_record(
        evm_records_receiver: &EvmRecorderReceiver,
        blockstore: &Arc<Blockstore>,
    ) -> Result<(), RecvTimeoutError> {
        let block = evm_records_receiver.recv_timeout(Duration::from_secs(1))?;
        let block_header = block.header;
        debug!("Writing evm block num = {}", block_header.block_number);
        blockstore
            .write_evm_block_header(&block_header)
            .expect("Expected database write to succed");
        for (hash, tx) in block.transactions {
            blockstore
                .write_evm_transaction(
                    block_header.block_number,
                    block_header.native_chain_slot,
                    hash,
                    tx,
                )
                .expect("Expected database write to succed");
        }
        Ok(())
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
