extern crate rand;
extern crate crypto;

use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug)]
pub struct Transaction {
    pub hash: String,
    pub to_address: String,
    pub from_address: String,
    pub value: i64,
}

impl Transaction {
    pub fn new() -> Transaction {
        let (to, from) = (rand::thread_rng().gen_range(1, 101), rand::thread_rng().gen_range(1, 101));
        let to_address = Transaction::to_hash(format!("{}", to));
        let from_address = Transaction::to_hash(format!("{}", from));
        let value = rand::thread_rng().gen_range(100, 10001);
        let hash = Transaction::to_hash(format!("{}{}{}", to_address, from_address, value));

        Transaction { hash: hash,
                    to_address: to_address,
                    from_address: from_address,
                    value: value }
    }

    fn to_hash(target: String) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&target);
        format!("0x{}", hasher.result_str())
    }
}
