use std::collections::HashMap;

const fn build_leet_dictionary() -> [(char, &'static str); 8] {
    [
        ('a', "4"),
        ('b', "8"),
        ('e', "3"),
        ('g', "6"),
        ('i', "1"),
        ('o', "0"),
        ('s', "5"),
        ('t', "7"),
    ]
}

const LEET_DICTIONARY: [(char, &'static str); 8] = build_leet_dictionary();

// pub fn encode(text: String) -> String {
//     // attempt encoding on the String as an array of bytes
//     for i in text {
//         return i;
//     }        
// }
    
pub fn decode(text: &str) -> String {
    // decode returns Result enum
    match rbase64::decode(text) {
        // if successful, turn the result bytes into a UTF-8 String -  if this doesn't work, return an error
        // attempt to unwrap, and if it fails return an error - disregard what caused it
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| "[!] Invalid UTF-8".to_string()),
        // disregard reason for error
        Err(_) => "[!] Decoding failed".to_string(),
    }
}