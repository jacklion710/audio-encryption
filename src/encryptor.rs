use hound::{SampleFormat, WavReader, Error as HoundError};
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;
use sodiumoxide::init;

// Function to encrypt audio data
pub fn encrypt_audio(input_path: &Path, output_path: &Path, key_path: &Path, nonce_path: &Path) -> Result<(), io::Error> {
    init().expect("Failed to initialize sodiumoxide");

    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let mut key_file = File::create(key_path)?;
    key_file.write_all(&key.0)?;
    let mut nonce_file = File::create(nonce_path)?;
    nonce_file.write_all(&nonce.0)?;

    let mut reader = WavReader::open(input_path).map_err(io_error)?;
    let spec = reader.spec();

    // Check for 16-bit PCM, 32-bit PCM, or 32-bit float formats
    let mut encrypted_samples = Vec::new();
    match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Int, 16) => {
            for sample in reader.samples::<i16>() {
                let sample = sample.map_err(io_error)?;
                let sample_bytes = sample.to_ne_bytes();
                let encrypted_data = secretbox::seal(&sample_bytes, &nonce, &key);
                encrypted_samples.extend_from_slice(&encrypted_data);
            }
        },
        (SampleFormat::Float, 32) => {
            for sample in reader.samples::<f32>() {
                let sample = sample.map_err(io_error)?;
                let sample_bytes = sample.to_ne_bytes();
                let encrypted_data = secretbox::seal(&sample_bytes, &nonce, &key);
                encrypted_samples.extend_from_slice(&encrypted_data);
            }
        },
        // Add case for 32-bit integer PCM if needed
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Unsupported sample format or bits per sample")),
    }

    let mut output_file = BufWriter::new(File::create(output_path)?);
    output_file.write_all(&encrypted_samples)?;
    output_file.flush()?;

    Ok(())
}

// Helper function to convert HoundError to io::Error
fn io_error(e: HoundError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
