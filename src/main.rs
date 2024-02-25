mod encryptor;
mod decryptor;
mod encryptor_lfo;
mod decryptor_lfo;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <command> <input_path> <output_path> <key_path>", args[0]);
        eprintln!("Commands:");
        eprintln!("  encrypt <input_path> <output_path> <key_path>");
        eprintln!("  decrypt <input_path> <output_path> <key_path>");
        eprintln!("  encrypt_lfo <input_path> <output_path> <key_path>");
        eprintln!("  decrypt_lfo <input_path> <output_path> <key_path>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "encrypt" | "decrypt" | "encrypt_lfo" | "decrypt_lfo" => {
            if args.len() != 5 {
                eprintln!("Usage: {} {} <input_path> <output_path> <key_path>", args[0], args[1]);
                std::process::exit(1);
            }
            // Call the appropriate function based on the command
            match args[1].as_str() {
                "encrypt" => encryptor::encrypt_audio(&args[2], &args[3], &args[4]),
                "decrypt" => decryptor::decrypt_audio(&args[2], &args[3], &args[4]),
                "encrypt_lfo" => encryptor_lfo::encrypt_audio_with_lfo(&args[2], &args[3], &args[4]),
                "decrypt_lfo" => decryptor_lfo::decrypt_audio_with_lfo(&args[2], &args[3], &args[4]),
                _ => unreachable!(), // We have already checked for valid commands
            }
        },
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            std::process::exit(1);
        },
    }
}
