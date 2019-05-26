use rand::Rng;
use nodes::node::Node;

use nodes::miner::Miner;
use utils::hash;

fn main() {
    println!("Start Rust Chain!");

    let mut node = Node::new(vec!());
    node.create_account();

    for i in 1..=10 {
        let account = node.accounts.first().unwrap();
        let address = hash::generate(i.to_string());
        let fee = rand::thread_rng().gen_range(1, i * 10) % 100;
        let transaction = account.send_transaction(address, i * 1000, fee);
        node.transactions.push(transaction);
    }

    for _ in 1..=3 {
        // let transactions = vec!(Transaction::dummy_new());
        node.mining();
    }
    node.print();
}
