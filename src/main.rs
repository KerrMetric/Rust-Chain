use nodes::node::Node;
use accounts::account::Account;

use nodes::miner::Miner;
use utils::hash;

fn main() {
    println!("Start Rust Chain!");

    let mut node = Node::new(vec!());
    let account = Account::new();
    node.accounts.push(account);

    for i in 1..=10 {
        let transaction = node.accounts.first().unwrap().send_transaction(hash::generate(i.to_string()), i * 1000);
        node.transactions.push(transaction);
    }

    for _ in 1..=3 {
        // let transactions = vec!(Transaction::dummy_new());
        node.mining();
    }
    node.print();
}
