use accounts::account::Account;
use core::transaction::Transaction;
use core::block::Block;

pub struct Node {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub(crate) block_chain: Vec<Block>,
}

impl Node {
    pub fn new(block_chain: Vec<Block>) -> Self {
        Self { accounts: vec![], transactions: vec![], block_chain: block_chain }
    }

    pub fn create_account(&mut self) {
        let account = Account::new();
        self.accounts.push(account);
    }

    pub fn print(&self) {
        for block in &self.block_chain {
            // FIXME: JSONで出力
            println!("{:?}", block);
        }
    }
}
