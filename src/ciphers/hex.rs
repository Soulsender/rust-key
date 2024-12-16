use hex;

pub fn encode(text: String) {
    println!("{}", hex::encode(text))
}

pub fn decode(text: &str) -> String {
    match hex::decode(text.as_bytes()) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8 text to decode. Are you sure this is base64?".to_string()),
        Err(_) => "Decoding failed".to_string(),
    }
}