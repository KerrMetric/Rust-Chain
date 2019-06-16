use utils::hash;
use db;
use nodes::miner::Miner;
use nodes::node::Node;
use rand::Rng;

fn main() {
    println!("Start Rust Chain!");

    let mut node = Node::new(vec![]);
    node.create_account();

    for i in 1..=10 {
        let account = node.accounts.first().unwrap();
        let address = hash::generate(i.to_string());
        let fee = rand::thread_rng().gen_range(1, i * 10) % 100;
        let transaction = account.send_transaction(address, i * 1000, fee);
        node.transactions.push(transaction);
    }

    let account = node.accounts.first().unwrap();
    let address = &account.address;
    match db::account_db::set(account) {
        Ok(_) => println!("db set success"),
        Err(e) => println!("Nooooooo: {}", e),
    };

    println!("set account: {}", address);

    match db::account_db::get(&address) {
        Some(v) => println!("db got address: {}", v.address),
        None => println!("db got address failure"),
    };

    match db::account_db::delete(&address) {
        Ok(_) => println!("db delete success"),
        Err(_) => println!("db delete failure"),
    };

    for _ in 1..=3 {
        // let transactions = vec!(Transaction::dummy_new());
        node.mining();
    }
    node.print();
}
