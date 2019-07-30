use db;
use nodes::miner::Miner;
use nodes::node::Node;
use rand::Rng;

pub fn simulate(from: &String, to: &String) {
    let from_account = db::account_db::get(from).expect("invalid address. no stored in db");
    let to_account = db::account_db::get(to).expect("invalid address. no stored in db");

    let mut node = Node::new(vec![]);
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
