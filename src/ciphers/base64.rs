use rbase64;

pub fn encode(text: String) -> String {
    // attempt encoding on the String as an array of bytes
    rbase64::encode(text.as_bytes())        
}
    
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