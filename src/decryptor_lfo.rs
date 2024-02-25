use hound;

#[allow(dead_code)]
pub fn decrypt_audio_with_lfo(input_path: &str, output_path: &str, lfo_path: &str) {
    // Opening the encrypted WAV reader and immediately consuming it to get samples.
    let spec = hound::WavReader::open(input_path)
        .expect("Failed to open encrypted WAV reader")
        .spec();

    let encrypted_samples: Vec<i16> = hound::WavReader::open(input_path)
        .expect("Failed to open encrypted WAV reader again")
        .into_samples::<i16>()
        .map(|s| s.unwrap())
        .collect();

    // Opening the LFO WAV reader and consuming it to get LFO samples.
    let lfo_samples: Vec<i16> = hound::WavReader::open(lfo_path)
        .expect("Failed to open LFO WAV reader")
        .into_samples::<i16>()
        .map(|s| s.unwrap())
        .collect();

    let mut decrypted_samples = Vec::new();

    // Decrypting by applying the inverse modulation process.
    for (encrypted_sample, lfo_sample) in encrypted_samples.iter().zip(lfo_samples.iter()) {
        let encrypted_sample_f32 = *encrypted_sample as f32 / i16::MAX as f32;
        let lfo_sample_f32 = *lfo_sample as f32 / i16::MAX as f32;

        // Applying the inverse of the original modulation formula.
        let decrypted_sample_f32 = encrypted_sample_f32 / (1.0 + lfo_sample_f32);

        // Scale and convert back to i16
        let decrypted_sample_i16 = (decrypted_sample_f32 * i16::MAX as f32) as i16;
        decrypted_samples.push(decrypted_sample_i16);
    }

    // Writing the decrypted samples to the output WAV file.
    let mut writer = hound::WavWriter::create(output_path, spec)
        .expect("Failed to create WAV writer for decrypted audio");
    for sample in decrypted_samples {
        writer.write_sample(sample).expect("Failed to write decrypted sample");
    }
    writer.finalize().expect("Failed to finalize decrypted WAV file");
}
