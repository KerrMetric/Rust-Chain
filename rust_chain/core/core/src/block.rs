use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub(crate) height: i32,
    pub(crate) size: i64,
    pub(crate) header: Header,
}

#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) parent_hash: String,
    pub(crate) block_hash: String,
    pub(crate) nonce: i64,
    pub(crate) transactions: Vec<Transaction>,
    pub(crate) time_stamp: i64,
}
