mod encryptor;
mod decryptor;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <encrypt|decrypt> <input_path> <output_path> <key_path>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "encrypt" => encryptor::encrypt_audio(&args[2], &args[3], &args[4]),
        "decrypt" => decryptor::decrypt_audio(&args[2], &args[3], &args[4]),
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            std::process::exit(1);
        },
    }
}
