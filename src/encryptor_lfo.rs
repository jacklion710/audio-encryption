use hound::{WavWriter, WavSpec, SampleFormat};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::f32::consts::PI;

// Define the Waveform enum
#[derive(Serialize, Deserialize, Debug)]
enum Waveform {
    Sine,
    Square,
    Triangle,
    Ramp,
    Noise,
}

// Define the LFO struct
#[derive(Debug)]struct LFO {
    waveform: Waveform,
    frequency: f32,
    phase: f32,
    amplitude: f32,
    offset: f32,
}

// Implement methods for LFO
impl LFO {
    fn new(waveform: Waveform, frequency: f32, phase: f32, amplitude: f32, offset: f32) -> Self {
        Self {
            waveform,
            frequency,
            phase,
            amplitude,
            offset,
        }
    }

    fn sample(&self, time: f32) -> f32 {
        match self.waveform {
            Waveform::Sine => self.amplitude * (2.0 * PI * self.frequency * time + self.phase).sin() + self.offset,
            Waveform::Square => {
                if (2.0 * PI * self.frequency * time + self.phase).sin() >= 0.0 {
                    self.amplitude + self.offset
                } else {
                    -self.amplitude + self.offset
                }
            },
            Waveform::Triangle => {
                self.amplitude * (2.0 * PI * self.frequency * time + self.phase).sin().asin() / (PI / 2.0) + self.offset
            },
            Waveform::Ramp => {
                self.amplitude * (((2.0 * PI * self.frequency * time + self.phase) / (2.0 * PI)).fract()) + self.offset
            },
            Waveform::Noise => {
                let mut rng = rand::thread_rng();
                self.amplitude * (rng.gen_range(-1.0..=1.0)) + self.offset
            },
        }
    }    
}

// Utility function to randomize LFO parameters
fn randomize_lfo_parameters() -> LFO {
    let mut rng = rand::thread_rng();

    let waveform = match rng.gen_range(0..=4) {
        0 => Waveform::Sine,
        1 => Waveform::Square,
        2 => Waveform::Triangle,
        3 => Waveform::Ramp,
        _ => Waveform::Noise,
    };

    let frequency = rng.gen_range(0.1..=20.0);
    let phase = rng.gen_range(0.0..=2.0 * PI);
    let amplitude = rng.gen_range(0.1..=0.5);
    let offset = 0.0;

    LFO::new(waveform, frequency, phase, amplitude, offset)
}

fn clip_sample_value(value: f32) -> f32 {
    value.max(-1.0).min(1.0)
}

#[allow(dead_code)]
pub fn encrypt_audio_with_lfo(input_path: &str, output_path: &str, lfo_path: &str) {
    let reader = hound::WavReader::open(input_path).expect("Failed to open WAV reader");
    let spec = reader.spec();
    let samples: Vec<i16> = reader.into_samples::<i16>().map(|s| s.unwrap()).collect();

    let sample_rate = spec.sample_rate as f32;
    let mut lfo = randomize_lfo_parameters(); // Initial LFO parameters
    println!("Initial LFO Parameters: {:?}", lfo);

    let mut encrypted_samples = Vec::new();
    let mut lfo_samples = Vec::new();
    let mut time = 0.0;
    let update_interval = sample_rate; // For example, update LFO parameters every second

    for (index, sample) in samples.iter().enumerate() {
        let sample_f32 = *sample as f32 / i16::MAX as f32; // Convert the sample to f32
    
        if index as f32 % update_interval == 0.0 {
            // Randomize LFO parameters every second
            lfo = randomize_lfo_parameters();
            println!("Updated LFO Parameters: {:?}", lfo);
        }
    
        let lfo_sample = lfo.sample(time);
        lfo_samples.push(lfo_sample);
    
        // Apply a more pronounced amplitude modulation effect
        // This example simply multiplies the audio sample by the LFO sample, but consider adjusting for more noticeable effects
        let modulated_sample = sample_f32 * lfo_sample; // Adjust this line as needed for your desired effect
        encrypted_samples.push((clip_sample_value(modulated_sample) * i16::MAX as f32) as i16);
    
        time += 1.0 / sample_rate;
    }
    
    // Ensure the encrypted_samples are not all zeros
    // println!("Some encrypted samples: {:?}", &encrypted_samples[..5]); // Debug: Print first few encrypted samples

    // Adjustments to ensure encrypted audio is audible and modulation key matches audio length
    let mut writer = hound::WavWriter::create(output_path, spec).expect("Failed to create WAV writer");
    for sample in encrypted_samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");

    // Save the LFO modulation sequence, ensuring it matches the audio file's length
    save_lfo_modulation_to_file(&lfo_samples, sample_rate as u32, lfo_path);
}

/// Saves the LFO modulation data to a file.
fn save_lfo_modulation_to_file(lfo_samples: &[f32], sample_rate: u32, file_path: &str) {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(file_path, spec).expect("Failed to create LFO WAV writer");
    for &sample in lfo_samples {
        let sample_int = ((clip_sample_value(sample)) * i16::MAX as f32) as i16; // Clip and scale
        writer.write_sample(sample_int).expect("Failed to write LFO sample");
    }
    writer.finalize().expect("Failed to finalize LFO WAV file");
}
