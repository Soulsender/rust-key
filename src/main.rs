use clap::Parser;

// import functions from the submodules
mod ciphers;

// struct of all the cli arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // text to encode/decode
    #[arg(short, long, required = true, help = "The string to operate on")]
    text: String,

    // cipher to use
    #[arg(short, long, required = true, help = "Specify the cipher to encode/decode with")]
    cipher: String,

    // encode flag (optional)
    #[clap(short, long, default_value_t = false, required = false, help = "Encode text (optional)")]
    encode: bool,

    // decode flag (optional)
    #[clap(short, long, default_value_t = false, required = false, help = "Decode text (optional)")]
    decode: bool
}


fn main() {
    let args = Cli::parse();
    
    if args.encode {
        encode(&args.cipher, args.text);
    }
    else if args.decode {
        decode(&args.cipher, args.text);
    }    
}

fn encode(method: &str, text: String) {
    match method {
        "base64" => println!("{}", ciphers::base64::encode(text)),
        _ => println!("[!] Error: invalid encoding type")
    }
}

fn decode(method: &str, text: String) {
    match method {
        "base64" => println!("{}", ciphers::base64::decode(&text)),
        _ => println!("[!] Error: invalid decoding type")
    }
}
