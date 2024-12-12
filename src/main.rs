use clap::Parser;

// import functions from the submodules
mod ciphers;

// struct of all the cli arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // text to encode/decode
    #[arg(short, long, required = false, alias = "input", help = "The string to operate on")]
    text: String,

    // cipher to use
    #[arg(short, long, required = false, help = "Specify the cipher to encode/decode with")]
    cipher: String,

    // encode flag (optional)
    #[clap(short, long, default_value_t = false, required = false, requires = "cipher", help = "Encode text")]
    encode: bool,

    // decode flag (optional)
    #[clap(short, long, default_value_t = false, required = false, requires = "cipher", help = "Decode text")]
    decode: bool,

    // list ciphers (optional)
    #[arg(long,  default_value_t = false, required = false, help = "List ciphers")]
    list: bool,
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
    else if args.list {
        list_ciphers();
    }
    else {
        println!("You didn't specify an action. Tf you want me to do?");
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
