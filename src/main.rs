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

    // this whole stupid thing is because clap refuses to take a String optionally, so it has to be an enum (which makes sense for rust, but i'm still gonna complain about it as a python programmer)
    match &args.cipher {
        // any cipher value is taken - the error handling is in the encode() and decode() functions
        Some(ref cipher_value) => {
            match &args.text {
                Some(ref text_value) => {
                    if args.encode {
                        encode(cipher_value, text_value.to_string());
                    }
                    // if decode flag is specified
                    else if args.decode {
                        decode(cipher_value, text_value.to_string());
                    }
                    else {
                        println!("You didn't specify an action. What do you want me to do?");
                    }
                }
                None => {
<<<<<<< HEAD
                    println!("No text provided");
=======
                    // error handled by clap
>>>>>>> d169d277d234157a0619bca958fff89c590dbbce
                }
            }
        }
        None => {
            // if no arguments are found, or if just text is used, print the list of ciphers
            list_ciphers();
        }
    }    
}

// encode function
fn encode(method: &str, text: String) {
    match method {
        // encoding
        "base64" => println!("{}", ciphers::base64::encode(text)),
<<<<<<< HEAD
        "hex" => ciphers::hex::encode(text),

        // hashing algorithms
        "md5" => ciphers::md::encode_md5(text),
        "md4" => ciphers::md::encode_md4(text),
        "md2" => ciphers::md::encode_md2(text),
        "sha224" => ciphers::sha::encode_sha224(text),
        "sha256" => ciphers::sha::encode_sha256(text),
        "sha384" => ciphers::sha::encode_sha384(text),
        "sha512" => ciphers::sha::encode_sha512(text),
=======
        "leet" => println!("{}", ciphers::leet::encode(text)),
>>>>>>> d169d277d234157a0619bca958fff89c590dbbce
        _ => println!("Error: invalid encoding type")
    }
}

// decode function
fn decode(method: &str, text: String) {
    match method {
        "base64" => println!("{}", ciphers::base64::decode(&text)),
<<<<<<< HEAD
        "hex" => println!("{}", ciphers::hex::decode(&text)),
=======
        "leet" => println!("{}", ciphers::leet::decode(&text)),
>>>>>>> d169d277d234157a0619bca958fff89c590dbbce
        _ => println!("Error: invalid decoding type")
    }
}

fn list_ciphers() {
<<<<<<< HEAD
    println!("AVAILABLE CIPHERS:
    
    base64
    
    AVAILABLE HASHING ALGORITHMS:
    md5
    md4
    md2
    sha224
    sha256
    sha384
    sha512")
=======
    println!("
    Print help with --help or -h

    AVAILABLE CIPHERS:
    base64  - Traditional base64
    leet    - 1337SP34K
    ")
>>>>>>> d169d277d234157a0619bca958fff89c590dbbce
}

