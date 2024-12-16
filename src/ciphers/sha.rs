use sha2::{Sha256, Sha512, Digest};

pub fn encode_sha256(text: String) {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_sha512(text: String) {
    let mut hasher = Sha512::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}