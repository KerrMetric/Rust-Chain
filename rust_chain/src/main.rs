extern crate crypto;
use chrono::Local;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

struct Block {
    height: i32,
    size: i64,
    header: Header,
}

struct Header {
    parent_hash: String,
    block_hash: String,
    nonce: i32,
    time_stamp: i64,
}

struct Miner {
    block_chain: Vec<Block>,
}

impl Miner {
    fn pow(&self) {
        let target = "00001000111111111111111111111111111111111111111111111111111111";
        let mut hash = "";
        let mut nonce: i64 = 0;
        let time_stamp = Local::now().timestamp();
    }

    fn calc(&self, nonce: i64, time_stamp: i64) -> String {
        let parent_hash = &self.block_chain.last().unwrap().header.parent_hash;
        let raw_data = format!("{}{}{}", parent_hash, nonce.to_string(), time_stamp.to_string());
        let mut hasher = Sha256::new();
        hasher.input_str(&raw_data);
        hasher.result_str()
    }
}

fn main() {
    println!("Start Rust Chain!");

    let header = Header { parent_hash: "0x0000000000000000000000000000000000000000".to_string(),
                          // TODO: PoWで求める
                        block_hash: "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3".to_string(),
                        nonce: 0,
                        time_stamp: Local::now().timestamp(),
                        };

    let genesis_block = Block { height: 0, size: 0, header: header, };

    let miner = Miner { block_chain: vec![genesis_block] };
}
