mod encryptor;
mod decryptor;

use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: {} [encrypt|decrypt] <input_path> <output_path> <keys_path> <nonce_path>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_path = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);
    let keys_path = Path::new(&args[4]);
    let nonce_path = Path::new(&args[5]);

    match mode.as_str() {
        "encrypt" => encryptor::encrypt_audio(input_path, output_path, keys_path, nonce_path)?,
        "decrypt" => decryptor::decrypt_audio(input_path, output_path, keys_path, nonce_path)?,
        _ => eprintln!("Invalid mode: {}. Use 'encrypt' or 'decrypt'.", mode),
    }

    Ok(())
}