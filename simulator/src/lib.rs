use db;
use nodes::miner::Miner;
use nodes::node::Node;
use rand::Rng;

pub fn simulate() {
    let mut node = Node::new(vec![]);

    let from_account = db::account_db::get(
        &"0x5214de409e4f511b6eed7b48ec427969e1bb57f6a766c19972b43236929c56b6".to_string(),
    )
    .unwrap();
    let to_account = db::account_db::get(
        &"0x3309e3883ea9bcc8b6a97f6358d26db109d7f9f8a52a5b1ec6664d9414a904de".to_string(),
    )
    .unwrap();

    for i in 1..=100 {
        let fee = rand::thread_rng().gen_range(1, i * 10) % 100;
        match from_account.send_transaction(&to_account, i * 100, fee) {
            Some(transaction) => node.push_transaction(transaction),
            None => continue,
        };

        if node.transactions.len() >= 10 {
            node.mining();
        }
    }

    node.print();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
