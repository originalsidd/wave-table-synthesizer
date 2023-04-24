use core::time::Duration;
use rodio::{OutputStream, source::Source};

mod wave_table_oscillator;
use crate::wave_table_oscillator::wave_table_oscillator::Oscillator;

fn main() {
    let size_wave_table: usize = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(size_wave_table);

    for i in 0..size_wave_table {
        wave_table.push((2.0 * std::f32::consts::PI * i as f32 / size_wave_table as f32).sin());
        println!("Wave table {}", wave_table[i]);
    }

    let mut oscillator = Oscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(2));
}
