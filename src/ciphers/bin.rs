pub fn encode(text: String) -> String {
    let mut encoded_text = String::new();
    // convert String to byte slice
    let new_text = text.as_bytes();
    // for every byte, convert it to bits
    for i in new_text {
        let x = format!("{i:08b}");
        encoded_text.push_str(&x);
    }

    encoded_text
}

