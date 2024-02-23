use hound;
use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};
use std::path::Path;

pub fn encrypt(input_path: &Path, output_path: &Path, seed: u64) {
    // Open the input WAV file
    let mut reader = hound::WavReader::open(input_path)
        .expect("Failed to open input WAV file");

    // Obtain the specifications of the audio file
    let spec = reader.spec();

    // Initialize the random number generator with the provided seed
    let mut rng = StdRng::seed_from_u64(seed);

    // Read samples and scramble them
    match spec.sample_format {
        hound::SampleFormat::Float if spec.bits_per_sample == 32 => {
            // Handle 32-bit floating-point samples
            let samples: Vec<f32> = reader.samples::<f32>()
                .map(|s| s.expect("Failed to read sample"))
                .collect();

            // Scramble the samples
            let mut scrambled_samples = samples.clone();
            scrambled_samples.shuffle(&mut rng);

            // Create the output WAV file
            let mut writer = hound::WavWriter::create(output_path, spec)
                .expect("Failed to create output WAV file");

            // Write the scrambled samples to the output file
            for sample in scrambled_samples {
                writer.write_sample(sample)
                    .expect("Failed to write sample");
            }

            // Finalize the WAV file to ensure all data is flushed
            writer.finalize().expect("Failed to finalize WAV file");
        },
        _ => panic!("Unsupported sample format or bit depth"),
    }

    println!("Encryption complete. Output saved to {:?}", output_path);
}
