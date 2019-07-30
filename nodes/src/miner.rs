use chrono::Local;

use crate::node::Node;
use core::block::Block;
use core::block::Header;

use core::transaction::Transaction;
use utils::hash;

pub trait Miner {
    fn mining(&mut self);
}

impl Miner for Node {
    fn mining(&mut self) {
        let (target_height, parent_hash) = match self.block_chain.last() {
            Some(parent_block) => (
                parent_block.height + 1,
                parent_block.header.block_hash.to_string(),
            ),
            None => (0, "0x0000000000000000000000000000000000000000".to_string()),
        };

        let mut transactions = std::mem::replace(&mut self.transactions, vec![]);

        let merkle_tree = make_merkle_tree(&transactions);
        let result = pow(&parent_hash, &merkle_tree.first().unwrap());

        transactions
            .iter_mut()
            .for_each(|transaction| transaction.pending = false);

        let block = create_block(target_height, &parent_hash, result, transactions);
        self.block_chain.push(block);
    }
}

fn make_merkle_tree(transactions: &Vec<Transaction>) -> Vec<String> {
    if transactions.len() == 0 {
        return vec![];
    }

    let mut hash_list: Vec<String> = transactions
        .iter()
        .map(|t| t.hash.clone())
        .collect::<Vec<String>>();

    loop {
        let len = hash_list.len();
        if (len & len - 1) == 0 {
            break;
        } else {
            let last = hash_list.last().unwrap().clone();
            hash_list.push(last);
        }
    }

    hash_list.reverse();

    let mut index = 0;
    while index < hash_list.len() - 1 {
        let hash = hash::generate(format!("{}{}", hash_list[index], hash_list[index + 1]));
        hash_list.push(hash);
        index += 2;
    }

    hash_list.reverse();
    hash_list
}

fn pow(parent_hash: &String, merkle_root: &String) -> (String, i64, i64) {
    let target = "01000100111111111111111111111111111111111111111111111111111111".to_string();
    let mut hash = "11111111111111111111111111111111111111111111111111111111111111".to_string();
    let mut nonce = 0_i64;
    let mut time_stamp = Local::now().timestamp();

    while hash > target {
        nonce += 1;
        time_stamp = Local::now().timestamp();
        hash = calc(parent_hash, merkle_root, nonce, time_stamp);
    }

    (format!("0x{}", hash), nonce, time_stamp)
}

fn calc(parent_hash: &String, merkle_root: &String, nonce: i64, time_stamp: i64) -> String {
    hash::sha256(format!(
        "{}{}{}{}",
        parent_hash,
        merkle_root,
        nonce.to_string(),
        time_stamp.to_string()
    ))
}

fn create_block(
    target_height: i32,
    parent_hash: &String,
    pow_result: (String, i64, i64),
    transactions: Vec<Transaction>,
) -> Block {
    let header = Header {
        parent_hash: parent_hash.to_string(),
        block_hash: pow_result.0,
        nonce: pow_result.1,
        transactions: transactions,
        time_stamp: pow_result.2,
    };
    // TODO: Block Sizeの計算
    Block {
        height: target_height,
        size: 0,
        header: header,
    }
}
