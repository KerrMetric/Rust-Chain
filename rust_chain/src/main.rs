extern crate core;
use core::miner;

fn main() {
    println!("Start Rust Chain!");

    let mut miner = miner::Miner::new();
    miner.pow();
}
