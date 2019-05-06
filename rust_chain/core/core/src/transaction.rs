extern crate rand;
extern crate crypto;

use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug)]
pub struct Transaction {
    pub to_address: String,
    pub from_address: String,
    pub value: i64,
}

impl Transaction {
    pub fn new() -> Transaction {
        let (to, from) = (rand::thread_rng().gen_range(1, 101), rand::thread_rng().gen_range(1, 101));
        let (to_data, from_data) = (format!("{}", to), format!("{}", from));
        let mut to_hasher = Sha256::new();
        let mut from_hasher = Sha256::new();
        to_hasher.input_str(&to_data);
        from_hasher.input_str(&from_data);

        Transaction { to_address: format!("0x{}", to_hasher.result_str()),
                    from_address: format!("0x{}", from_hasher.result_str()),
                    value: rand::thread_rng().gen_range(100, 10001) }
    }
}