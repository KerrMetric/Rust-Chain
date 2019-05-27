use chrono::Local;

use core::transaction::Transaction;
use core::block::Block;
use core::block::Header;
use utils::hash;
use crate::node::Node;

pub trait Miner {
    fn mining(&mut self);
}

impl Miner for Node {
    fn mining(&mut self) {
        let (target_height, parent_hash) = match self.block_chain.last() {
            Some(parent_block) => (parent_block.height + 1, parent_block.header.block_hash.to_string()),
            None => (0, "0x0000000000000000000000000000000000000000".to_string()),
        };
        self.transactions.sort_by_key(|t| t.fee);
        let mut transactions: Vec<Transaction> = vec!();
        for _ in 1..=3 {
            let transaction = self.transactions.pop().unwrap();
            transactions.push(transaction);
        }
        let merkle_root = hash::generate(format!("{}{}{}", transactions[0].hash, transactions[1].hash, transactions[2].hash));
        let result = pow(&parent_hash, &merkle_root);
        for transaction in transactions.iter_mut() {
            transaction.pending = false;
        }
        let block = create_block(target_height, &parent_hash, result, transactions);
        self.block_chain.push(block);
    }
}

fn pow(parent_hash: &String, merkle_root: &String) -> (String, i64, i64) {
    let target = "01000100111111111111111111111111111111111111111111111111111111".to_string();
    let mut hash = "11111111111111111111111111111111111111111111111111111111111111".to_string();
    let mut nonce: i64 = 0;
    let mut time_stamp = Local::now().timestamp();

    while hash > target {
        nonce += 1;
        time_stamp = Local::now().timestamp();
        hash = calc(parent_hash, merkle_root, nonce, time_stamp);
    }

    (format!("0x{}", hash), nonce, time_stamp)
}

fn calc(parent_hash: &String, merkle_root: &String, nonce: i64, time_stamp: i64) -> String {
    hash::sha256(format!("{}{}{}{}", parent_hash, merkle_root, nonce.to_string(), time_stamp.to_string()))
}

fn create_block(target_height: i32,
                parent_hash: &String,
                pow_result: (String, i64, i64),
                transactions: Vec<Transaction>) -> Block {
    let header = Header { parent_hash: parent_hash.to_string(),
                block_hash: pow_result.0,
                nonce: pow_result.1,
                transactions: transactions,
                time_stamp: pow_result.2,
                };
    // TODO: Block Sizeの計算
    Block { height: target_height, size: 0, header: header, }
}
