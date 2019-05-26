use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn sha256(target: String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&target);
    hasher.result_str()
}

pub fn generate(target: String) -> String {
    format!("0x{}", sha256(target))
}
