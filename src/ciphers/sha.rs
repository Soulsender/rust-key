use sha2::{Sha256, Sha512, Sha384, Sha224, Digest};

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

pub fn encode_sha224(text: String) {
    let mut hasher = Sha224::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_sha384(text: String) {
    let mut hasher = Sha384::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}