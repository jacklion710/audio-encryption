use hound;
use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let input_path = "audio/raw/sine-c.wav";
    let output_path = "audio/encrypted/sine-c-encr.wav";
    let seed = 123456789; // This acts as the "encryption key"

    // Read the input WAV file
    let mut reader = hound::WavReader::open(input_path).expect("Failed to open WAV file");
    let spec = reader.spec();

    let mut rng = StdRng::seed_from_u64(seed);

    match spec.sample_format {
        hound::SampleFormat::Float => {
            if spec.bits_per_sample == 32 {
                // Handle 32-bit floating-point samples
                let samples: Vec<f32> = reader.samples::<f32>()
                    .map(|s| s.expect("Failed to read sample"))
                    .collect();

                let mut scrambled_samples = samples.clone();
                scrambled_samples.shuffle(&mut rng);

                let mut writer = hound::WavWriter::create(output_path, spec)
                    .expect("Failed to create output WAV file");
                for sample in scrambled_samples {
                    writer.write_sample(sample).expect("Failed to write sample");
                }

                writer.finalize().expect("Failed to finalize WAV file");
                println!("Scrambling complete. Output saved to {}", output_path);
            } else {
                println!("Unsupported bits per sample for float format.");
            }
        },
        hound::SampleFormat::Int => {
            // Your existing handling for integer samples goes here...
            println!("This example now also supports 32-bit floating-point samples.");
        }
    }
}
