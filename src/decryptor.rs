use hound::{WavWriter, Error as HoundError};
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use sodiumoxide::init;

pub fn decrypt_audio(input_path: &Path, output_path: &Path, key_path: &Path, nonce_path: &Path) -> Result<(), io::Error> {
    init().expect("Failed to initialize sodiumoxide");

    let mut key_bytes = vec![0u8; secretbox::KEYBYTES];
    let mut nonce_bytes = vec![0u8; secretbox::NONCEBYTES];
    let mut encrypted_data = Vec::new();
    File::open(input_path)?.read_to_end(&mut encrypted_data)?;
    File::open(key_path)?.read_exact(&mut key_bytes)?;
    File::open(nonce_path)?.read_exact(&mut nonce_bytes)?;
    let key = secretbox::Key::from_slice(&key_bytes).unwrap();
    let nonce = secretbox::Nonce::from_slice(&nonce_bytes).unwrap();

    let decrypted_data = secretbox::open(&encrypted_data, &nonce, &key)
        .expect("Decryption failed");

    let samples = decrypted_data
        .chunks_exact(2)
        .map(|chunk| i16::from_ne_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<i16>>();

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(output_path, spec).map_err(io_error)?;

    for sample in samples {
        writer.write_sample(sample).map_err(io_error)?;
    }
    writer.finalize().map_err(io_error)?;

    Ok(())
}

fn io_error(e: HoundError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
