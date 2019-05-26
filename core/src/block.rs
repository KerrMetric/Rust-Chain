use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub height: i32,
    pub size: i64,
    pub header: Header,
}

#[derive(Debug)]
pub struct Header {
    pub parent_hash: String,
    pub block_hash: String,
    pub nonce: i64,
    pub transactions: Vec<Transaction>,
    pub time_stamp: i64,
}
