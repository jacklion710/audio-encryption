use hound;
use rand::prelude::*;
use serde_json;
use std::fs::File;

pub fn encrypt_audio(input_path: &str, output_path: &str, key_path: &str) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV reader");
    
    let spec = reader.spec();
    let samples: Vec<i16> = reader.into_samples::<i16>().map(|s| s.unwrap()).collect();

    let mut rng = thread_rng();
    let mut encrypted_samples = Vec::new();
    let mut key_samples = Vec::new();

    for sample in samples {
        let key_sample: i16 = rng.gen_range(-32768..=32767);
        let encrypted_sample = sample.wrapping_add(key_sample);
        encrypted_samples.push(encrypted_sample);
        key_samples.push(key_sample);
    }

    let mut writer = hound::WavWriter::create(output_path, spec).expect("Failed to create WAV writer");
    for sample in encrypted_samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");

    let key_file = File::create(key_path).expect("Failed to create key file");
    serde_json::to_writer(&key_file, &key_samples).expect("Failed to write key file");
}
