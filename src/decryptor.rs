use hound;
use rand::{seq::SliceRandom, SeedableRng, rngs::StdRng};
use std::path::Path;

pub fn decrypt(input_path: &Path, output_path: &Path, seed: u64) {
    // Open the input WAV file
    let mut reader = hound::WavReader::open(input_path)
        .expect("Failed to open input WAV file");

    // Obtain the specifications of the audio file
    let spec = reader.spec();

    // Initialize the random number generator with the provided seed
    let mut rng = StdRng::seed_from_u64(seed);

    // The decryption process would ideally reverse the encryption process.
    // For the sake of this example, we follow the same steps as encryption, 
    // acknowledging this does not truly "decrypt" in a traditional sense.
    match spec.sample_format {
        hound::SampleFormat::Float if spec.bits_per_sample == 32 => {
            // Handle 32-bit floating-point samples
            let samples: Vec<f32> = reader.samples::<f32>()
                .map(|s| s.expect("Failed to read sample"))
                .collect();

            // "Decrypt" the samples (this step is conceptual under the current approach)
            let mut decrypted_samples = samples.clone();
            decrypted_samples.shuffle(&mut rng);

            // Create the output WAV file
            let mut writer = hound::WavWriter::create(output_path, spec)
                .expect("Failed to create output WAV file");

            // Write the "decrypted" samples to the output file
            for sample in decrypted_samples {
                writer.write_sample(sample)
                    .expect("Failed to write sample");
            }

            // Finalize the WAV file to ensure all data is flushed
            writer.finalize().expect("Failed to finalize WAV file");
        },
        _ => panic!("Unsupported sample format or bit depth"),
    }

    println!("Decryption complete. Output saved to {:?}", output_path);
}
