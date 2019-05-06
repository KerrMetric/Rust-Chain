#[derive(Debug)]
pub(crate) struct Block {
    pub(crate) height: i32,
    pub(crate) size: i64,
    pub(crate) header: Header,
}

#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) parent_hash: String,
    pub(crate) block_hash: String,
    pub(crate) nonce: i64,
    pub(crate) time_stamp: i64,
}
