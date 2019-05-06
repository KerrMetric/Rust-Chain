extern crate core;
use core::miner;
use core::transaction;

fn main() {
    println!("Start Rust Chain!");

    let mut miner = miner::Miner::new();
    for _ in 1..=3 {
        let transaction = transaction::Transaction::new();
        let transactions = vec!(transaction);
        miner.pow(transactions);
    }
    miner.print();
}
