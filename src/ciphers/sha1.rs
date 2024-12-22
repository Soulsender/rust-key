use std::{fs, path::Path};
use std::ffi::OsStr;
use sha1_checked::{Digest, Sha1};
use hex::encode as hex_encode;

pub fn encode(text: String) -> String {
    // attempt encoding on the String as an array of bytes
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    hex_encode(hasher.finalize())
}
  
pub fn decode(text: &str, wordlist_path: &str) -> String {
    let path = Path::new(wordlist_path);
    let contents = fs::read_to_string(path);

    contents.unwrap()
}