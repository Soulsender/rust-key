use std::env;

// import functions from the submodules
mod ciphers;

fn main() {
    // get command line arguments as a String Vector
    let args: Vec<String> = env::args().collect();
    // set the operation String to the second arg
    let text = &args[2];

    if &args[1] == "encode" {
        println!("{}", ciphers::base64::encode(text.to_string()));
    } else if &args[1] == "decode" {
        println!("{}", ciphers::base64::decode(text));
    }
}
