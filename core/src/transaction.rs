use utils;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: i64,
    pub fee: i64,
    pub pending: bool,
}

impl Transaction {
    pub fn new(from_address: String, to_address: String, value: i64, fee: i64) -> Transaction {
        let hash = utils::hash::generate(format!("{}{}{}", to_address, from_address, value));
        Transaction{hash: hash,
                    from_address: from_address,
                    to_address: to_address,
                    value: value,
                    fee: fee,
                    pending: true}
    }
}
