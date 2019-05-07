use rand::Rng;
use utils;

pub struct Account {
    pub address: String,
    pub balance: i64,
    pub miner: bool,
}

impl Account {
    fn new() -> Self {
        Self { address: utils::hash::generate(format!("{}", rand::thread_rng().gen_range(1, 10001))),
            balance: 0,
            miner: false
        }
    }
}
