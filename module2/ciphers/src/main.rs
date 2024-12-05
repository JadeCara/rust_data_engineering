/*

To run homophone cipher:

cargo run --  --message "Off to the bunker. Every person for themselves" --ahomophone --shift 0

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/

use cipher_makers::{decrypt, encrypt, homophonic_cipher};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Encrypt the message using a homophonic cipher
    #[arg(short, long)]
    ahomophone: bool,

    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}
 
 fn main() {

    let args = Args::parse();
    if args.ahomophone {
        println!("Encrypting the plaintext with homophonic cipher: {}", &args.message);
        homophonic_cipher(&args.message);
    } else if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else {
        println!("Please specify either --encrypt or --decrypt");
    }
 }