use rand::Rng;
use utils;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: i64,
}

impl Transaction {
    pub fn dummy_new() -> Transaction {
        let (to, from) = (rand::thread_rng().gen_range(1, 101), rand::thread_rng().gen_range(1, 101));
        let to_address = utils::hash::generate(format!("{}", to));
        let from_address = utils::hash::generate(format!("{}", from));
        let value = rand::thread_rng().gen_range(100, 10001);
        let hash = utils::hash::generate(format!("{}{}{}", to_address, from_address, value));

        Transaction { hash: hash,
                    to_address: to_address,
                    from_address: from_address,
                    value: value }
    }

    pub fn new(from_address: String, to_address: String, value: i64) -> Transaction {
        let hash = utils::hash::generate(format!("{}{}{}", to_address, from_address, value));
        Transaction{hash: hash,
                    from_address: from_address,
                    to_address: to_address,
                    value: value}
    }
}
