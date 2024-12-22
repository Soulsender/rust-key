use phf::phf_map;

// create static hashmap of char to number string
// we use the phf create because the hashmap is not changing throughout the runtime of the program
static LEET_MAP: phf::Map<char, char> = phf_map! {
    'a' => '4',
    'b' => '8',
    'e' => '3',
    'g' => '6',
    'i' => '1',
    'o' => '0',
    's' => '5',
    't' => '7',
};

// I could just generate this at runtime but why waste the memory?
static LEET_MAP_REV: phf::Map<char, char> = phf_map! {
    '4' => 'a',
    '8' => 'b',
    '3' => 'e',
    '6' => 'g',
    '1' => 'i',
    '0' => 'o',
    '5' => 's',
    '7' => 't',
};


pub fn encode(text: String) -> String {
    // create empty String to push the result to
    let mut encoded_text = String::new();

    // for each char in the text string
    for character in text.to_lowercase().chars() {
        // if the char is a key in the leet_map
        match LEET_MAP.get(&character) {
            // push the value to the result
            Some(&leet_char) => {
                encoded_text.push(leet_char);
            }
            // push the original char to the result
            None => {
                encoded_text.push(character);
            }
        }
    }

    // return the encoded text
    encoded_text
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