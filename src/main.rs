use cpal::traits::{DeviceTrait, HostTrait, StreamTrait}; // Import StreamTrait to access play
use cpal::{default_host, StreamConfig};
use hound::{WavWriter, WavSpec};

fn main() {
    // Get the default host and input device
    let host = default_host();
    let device = host.default_input_device().expect("No input device available");

    // Get the default input configuration
    let input_config = device.default_input_config().expect("Failed to get default input config");

    // Create a WAV writer with the correct specifications
    let spec = WavSpec {
        channels: input_config.channels(),
        sample_rate: input_config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec).expect("Failed to create WAV writer");

    // Create a stream to capture audio
    let config: StreamConfig = input_config.into();
    let stream = device.build_input_stream(
        &config,
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            for &sample in data {
                writer.write_sample(sample).expect("Failed to write sample");
            }
        },
        |err| eprintln!("Error occurred on stream: {:?}", err),
        None // Provide an option for duration
    ).expect("Failed to create input stream");

    // Start the stream
    stream.play().expect("Failed to play stream");

    println!("Recording... Press Enter to stop.");
    let _ = std::io::stdin().read_line(&mut String::new());

    // The stream will stop when the function exits
    println!("Recording stopped.");
}
