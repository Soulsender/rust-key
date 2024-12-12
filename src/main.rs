use clap::{Parser, ValueHint};

// import functions from the submodules
mod ciphers;

// struct of all the cli arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // cipher to use
    #[arg(short, long, default_missing_value = "help", help = "Specify the cipher to encode/decode with")]
    cipher: String,

    // text to encode/decode
    #[arg(short, long, help = "The string to operate on")]
    text: String,

    // encode flag
    #[clap(short, long, default_value_t = false, conflicts_with("decode"), requires = "cipher", help = "Encode text")]
    encode: bool,

    // decode flag
    #[clap(short, long, default_value_t = false, conflicts_with("encode"), requires = "cipher", help = "Decode text")]
    decode: bool,
}

fn main() {
    let args = Cli::parse();

    // if encode flag is specified
    if args.encode {
        encode(&args.cipher, args.text);
    }
    // if decode flag is specified
    else if args.decode {
        decode(&args.cipher, args.text);
    }
    else {
        println!("You didn't specify an action. What do you want me to do?");
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
