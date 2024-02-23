# Audio Encryption Project

This project demonstrates a simple audio processing experiment that encrypts and decrypts WAV audio files using sample scrambling based on a pseudo-random sequence generated from a seed. The project is structured to allow encryption and decryption through command-line arguments, providing a flexible way to process audio files.

## Project Structure

- `src/`: Contains the Rust source files.
  - `main.rs`: The entry point of the application.
  - `encryptor.rs`: Module for encrypting audio files.
  - `decryptor.rs`: Module for decrypting audio files.
- `audio/`: Directory for raw, encrypted, and decrypted audio files.
  - `raw/`: Contains original audio files.
  - `encrypted/`: Contains audio files after encryption.
  - `decrypted/`: Contains audio files after decryption.
- `keys/`: Directory containing the key and nonce used for encryption and decryption.
- `Cargo.toml`: Rust's package manifest file.

## Setup

Ensure you have Rust and Cargo installed on your system. You can download them from [https://rustup.rs/](https://rustup.rs/).

Clone the repository to your local machine:

```bash
git clone https://github.com/jacklion710/audio-encryption.git
cd audio-encryption
```

## Usage

The application supports two modes of operation: `encrypt` and `decrypt`. You need to specify the mode, input file path, output file path, and the path to the keys file containing the seed.

## Encrypting an Audio File
```bash
cargo run encrypt audio/raw/sine-c.wav audio/encrypted/sine-c-encr.wav keys/sin-c-key.txt keys/sin-c-nonce.txt
```

This command will read `sine-c.wav` from the `audio/raw` directory, encrypt it using a key and nonce then saves the key in `keys/sin-c-key.txt` and the nonce in `keys/sin-c-nonce.txt`, and save the encrypted file as `sine-c-encr.wav` in the `audio/encrypted` directory.

# Decrypting an Audio File

```bash
cargo run decrypt audio/encrypted/sin-c-encr.wav audio/decrypted/sin-c-decr.wav keys/sin-c-key.txt keys/sin-c-nonce.txt
```

This command will read `sin-c-encr.wav` from the `audio/encrypted` directory, decrypt it using the seed provided in `keys/sin-c-key.txt` and `keys/sin-c-nonce.txt`, and save the decrypted file as `sin-c-decr.wav` in the `audio/decrypted` directory.

## Note on Security

This project is a demonstration of audio processing and not intended for secure encryption purposes. The encryption and decryption are based on audio sample scrambling and should not be used for sensitive information.