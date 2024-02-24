use hound;
use serde_json;
use std::fs::File;

pub fn decrypt_audio(input_path: &str, output_path: &str, key_path: &str) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV reader");
    let spec = reader.spec();
    let samples: Vec<i16> = reader.into_samples::<i16>().map(|s| s.unwrap()).collect();

    let key_file = File::open(key_path).expect("Failed to open key file");
    let key_samples: Vec<i16> = serde_json::from_reader(key_file).expect("Failed to read key file");

    let mut decrypted_samples = Vec::new();

    for (sample, key_sample) in samples.iter().zip(key_samples.iter()) {
        let decrypted_sample = sample.wrapping_sub(*key_sample);
        decrypted_samples.push(decrypted_sample);
    }

    let mut writer = hound::WavWriter::create(output_path, spec).expect("Failed to create WAV writer");
    for sample in decrypted_samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");
}
