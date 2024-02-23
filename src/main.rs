mod encryptor;
mod decryptor;

use std::env;
use std::path::Path;

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

    // Example: Read the seed from keys_path, for simplicity assume it's directly the seed
    let seed = std::fs::read_to_string(keys_path)
        .expect("Failed to read seed from keys file")
        .trim()
        .parse::<u64>()
        .expect("Failed to parse seed as u64");

    match mode.as_str() {
        "encrypt" => encryptor::encrypt(input_path, output_path, seed),
        "decrypt" => decryptor::decrypt(input_path, output_path, seed),
        _ => eprintln!("Invalid mode: {}. Use 'encrypt' or 'decrypt'.", mode),
    }
}
