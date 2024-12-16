use md5::compute;
use md4::{Md4, Digest};
use md2::Md2;

pub fn encode_md5(text: String) {
    println!("{:x}", compute(text))
}

pub fn encode_md4(text: String) {
    let mut hasher = Md4::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}

pub fn encode_md2(text: String) {
    let mut hasher = Md2::new();
    hasher.update(text.as_bytes());
    println!("{:x}", hasher.finalize())
}
