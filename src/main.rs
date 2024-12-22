use clap::Parser;

// import functions from the submodules
mod ciphers;

// struct of all the cli arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // cipher to use
    #[arg(short, long, required = false, help = "Specify the cipher to encode/decode with\nRun the program with no arguments to get a list of ciphers")]
    cipher: Option<String>,

    // text to encode/decode
    #[arg(short, long, required = false, help = "The string to operate on")]
    text: Option<String>,

    // encode flag
    #[clap(short, long, default_value_t = false, conflicts_with("decode"), requires = "cipher", help = "Encode text")]
    encode: bool,

    // decode flag
    #[clap(short, long, default_value_t = false, conflicts_with("encode"), requires = "cipher", help = "Decode text")]
    decode: bool,
}

fn main() {
    let args = Cli::parse();

    match (&args.cipher, &args.text) {
        (Some(cipher_value), Some(text_value)) => {
            if args.encode {
                encode(cipher_value, text_value.to_string());
            } else if args.decode {
                decode(cipher_value, text_value.to_string());
            } else {
                eprintln!("You didn't specify an action. Please choose encode or decode.");
            }
        }
        (Some(_), None) => { /* Handled by clap */ }
        (None, _) => list_ciphers(),
    }
}

// encode function
fn encode(method: &str, text: String) {
    match method {
        // encoding
        "base64" => println!("{}", ciphers::base64::encode(text)),
        "hex" => ciphers::hex::encode(text),
        "leet" => println!("{}", ciphers::leet::encode(text)),

        // hashing algorithms
        "md5" => ciphers::md::encode_md5(text),
        "md4" => ciphers::md::encode_md4(text),
        "md2" => ciphers::md::encode_md2(text),
        "sha1" => println!("{}", ciphers::sha1::encode(text)),
        "sha224" => ciphers::sha::encode_sha224(text),
        "sha256" => ciphers::sha::encode_sha256(text),
        "sha384" => ciphers::sha::encode_sha384(text),
        "sha512" => ciphers::sha::encode_sha512(text),
        _ => println!("Error: invalid encoding type")
    }
}

// decode function
fn decode(method: &str, text: String) {
    match method {
        "base64" => println!("{}", ciphers::base64::decode(&text)),
        "hex" => println!("{}", ciphers::hex::decode(&text)),
        "leet" => println!("{}", ciphers::leet::decode(&text)),
        _ => println!("Error: invalid decoding type")
    }
}

fn list_ciphers() {
    println!("
    
    Print help with --help or -h

    AVAILABLE CIPHERS:
    base64  - Traditional base64
    leet    - 1337SP34K
    hex     - Traditional hex code

    AVAILABLE HASHING ALGORITHMS
    sha1
    sha224
    sha256
    sha384
    sha512
    ")
}

