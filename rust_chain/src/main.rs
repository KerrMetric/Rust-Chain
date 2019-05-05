struct Block {
    height: i32,
    size: i64,
    header: Header,
}

struct Header {
    parent_hash: String,
    block_hash: String,
    merkle_root: String,
    logs_bloom: String,
    state_root: String,
    nonce: i32,
    time_stamp: i64,
}

fn main() {
    println!("Hello, world!");
}
