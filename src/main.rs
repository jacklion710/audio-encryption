use hound;
use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let input_path = "/Users/jacobleone/Desktop/sine-c.wav";
    let output_path = "/Users/jacobleone/Desktop/sine-c-encr.wav";
    let seed = 123456789; // This acts as the "encryption key"

    // Read the input WAV file
    let mut reader = hound::WavReader::open(input_path).expect("Failed to open WAV file");
    let spec = reader.spec();

    // Check the sample format and handle accordingly
    match spec.sample_format {
        hound::SampleFormat::Float => {
            // Handle floating point samples (for advanced use cases)
            println!("Floating point format not directly supported in this example.");
        },
        hound::SampleFormat::Int => {
            if spec.bits_per_sample == 16 {
                // Your existing code assumes 16-bit samples, which is handled here
                let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

                // Scramble the samples
                let mut rng = StdRng::seed_from_u64(seed);
                let mut scrambled_samples = samples.clone();
                scrambled_samples.shuffle(&mut rng);

                // Write the scrambled samples to the output WAV file
                let mut writer = hound::WavWriter::create(output_path, spec).expect("Failed to create output WAV file");
                for sample in scrambled_samples {
                    writer.write_sample(sample).expect("Failed to write sample");
                }

                writer.finalize().expect("Failed to finalize WAV file");
                println!("Scrambling complete. Output saved to {}", output_path);
            } else {
                println!("This example only supports 16-bit integer samples.");
            }
        }
    }
}
