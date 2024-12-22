use sha2::{Sha256, Sha512, Sha384, Sha224, Digest};
use sha1_checked::Sha1;
use hex::encode as hex_encode;


pub fn encode_sha1(text: String) -> String {
    // attempt encoding on the String as an array of bytes
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    hex_encode(hasher.finalize())
}

pub fn encode_sha224(text: String) {
    let mut hasher = Sha224::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_sha256(text: String) {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_sha384(text: String) {
    let mut hasher = Sha384::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_sha512(text: String) {
    let mut hasher = Sha512::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}