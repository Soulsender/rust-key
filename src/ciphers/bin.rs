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

pub fn decode(text: String) -> String {
    let mut temp_string = String::new();

    for (i, value) in text.chars().enumerate() {
        if is_divisible_by_8((i).try_into().unwrap()) {
            temp_string.push(value);
            temp_string.push(' ');
        } else {
            temp_string.push(value);
        }
    }
    // let decoded_text: Vec<&str> = temp_string.split(' ').collect();
    
    String::from_utf8(decode_binary(&temp_string)).expect("Error Decoding")
}

// I didn't write this because fuck that
fn decode_binary(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2).unwrap_or(0))
        .collect()
}

fn is_divisible_by_8(value: i32) -> bool {
    (value + 1) % 8 == 0
}