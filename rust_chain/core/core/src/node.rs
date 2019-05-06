use crate::account::Account;
use crate::block::Block;

pub struct Node {
    pub accounts: Vec<Account>,
    pub(crate) block_chain: Vec<Block>,
}

impl Node {
    pub fn new(block_chain: Vec<Block>) -> Self {
        Self { accounts: vec![], block_chain: block_chain }
    }

    pub fn print(&self) {
        for block in &self.block_chain {
            // FIXME: JSONで出力
            println!("{:?}", block);
        }
    }
}
