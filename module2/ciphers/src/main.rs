/*

To run homophone cipher:

cargo run --  --message "Off to the bunker. Every person for themselves" --ahomophone --shift 0

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

To Guess: 

cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess 

*/

use cipher_makers::{decrypt, encrypt, homophonic_cipher};
use clap::Parser;

mod decoder_ring;

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

    /// Guess the shift for the message
    #[arg(short, long)]
    guess: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    //statistical information about the message
    #[arg(short, long)]
    printstats: bool,

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
    } else if args.guess {
        if args.printstats {
            decoder_ring::print_stats_analysis(&args.message);
        }
        let (depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift(&args.message, 26);
        println!(
            "Best shift: {} (out of {}), score: {}",
            best_shift, depth, max_score
        );
        println!("Decrypted message: {}", decrypted); 
    } else {
        println!("Please specify mode: --ahomophone, --encrypt, --decrypt, or --guess");
    }
 }