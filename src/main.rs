mod encryptor;
mod decryptor;

use std::env;
use std::fs;
use std::path::Path;
use rand::{Rng, thread_rng};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} [encrypt|decrypt] <input_path> <output_path> <keys_path>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_path = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);
    let keys_path = Path::new(&args[4]);

    match mode.as_str() {
        "encrypt" => {
            // Generate a new seed for encryption
            let seed: u64 = thread_rng().gen();
            // Save the seed to the specified keys path
            fs::write(keys_path, seed.to_string()).expect("Failed to write seed to keys file");

            encryptor::encrypt(input_path, output_path, seed);
        },
        "decrypt" => {
            // Read the seed from the keys path
            let seed = fs::read_to_string(keys_path)
                .expect("Failed to read seed from keys file")
                .trim()
                .parse::<u64>()
                .expect("Failed to parse seed as u64");

            decryptor::decrypt(input_path, output_path, seed);
        },
        _ => eprintln!("Invalid mode: {}. Use 'encrypt' or 'decrypt'.", mode),
    }
}
