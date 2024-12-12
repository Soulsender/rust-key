use ciphers::base64;
use clap::{Parser};

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

    // this whole stupid thing is because clap refuses to take a String optionally, so it has to be an enum
    match &args.cipher {
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
                    // error handled by clap
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
        "base64" => println!("{}", ciphers::base64::encode(text)),
        _ => println!("Error: invalid encoding type")
    }
}

// decode function
fn decode(method: &str, text: String) {
    match method {
        "base64" => println!("{}", ciphers::base64::decode(&text)),
        _ => println!("Error: invalid decoding type")
    }
}

fn list_ciphers() {
    println!("
    AVAILABLE CIPHERS:
    
    base64
    ")
}

