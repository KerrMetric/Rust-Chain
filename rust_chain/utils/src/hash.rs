extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn generate(target: String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&target);
    format!("0x{}", hasher.result_str())
}
