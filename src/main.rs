use std::io;
use core::time::Duration;
use rodio::{OutputStream, source::Source};

mod wave_table_oscillator;
use crate::wave_table_oscillator::wave_table_oscillator::Oscillator;

fn main() {
    let size_wave_table: usize = 64;
    let mut wave_table_sin: Vec<f32> = Vec::with_capacity(size_wave_table);
    let mut wave_table_pulse: Vec<f32> = Vec::with_capacity(size_wave_table);
    let mut wave_table_saw: Vec<f32> = Vec::with_capacity(size_wave_table);
    let mut wave_table_supersaw: Vec<f32> = Vec::with_capacity(size_wave_table);

    // Sine Wave
    for i in 0..size_wave_table {
        let time = i as f32 / size_wave_table as f32;
        let value = 2.0 * std::f32::consts::PI * time;
        wave_table_sin.push(value.sin());
    }

    // Pulse Wave
    for i in 0..size_wave_table {
        if i < size_wave_table / 2 {
            wave_table_pulse.push(-1.0);
        } else {
            wave_table_pulse.push(1.0);
        }
    }

    // Saw Wave
    for i in 0..size_wave_table {
        let value = (2.0 * i as f32 / size_wave_table as f32) - 1.0;
        wave_table_saw.push(value);
    }

    // SuperSaw!
    let freq = 2048.0;
    let spread = 64;
    let detune = 0.02;
    for i in 0..size_wave_table {
        let mut value = 0.0;
        for j in 0..spread {
            let detune_freq = (j as f32 - spread as f32 / 2.0) * detune * freq;
            let index = (i as f32 + detune_freq) % size_wave_table as f32;
            let weight = (1.0 - (index - i as f32).abs() / freq).max(0.0);
            value += weight * (index - i as f32).signum();
        }
        wave_table_supersaw.push(value / spread as f32);
    }

    println!("Welcome to WaveTable Synthesizer!");
    loop {
        println!("Pick a sound!");
        println!("1. Sine");
        println!("2. Pulse/Square");
        println!("3. Triangle/Sawtooth");
        println!("4. SuperSaw");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let option = input.trim().parse().unwrap();

        let wave_table: &Vec<f32>;
        match option {
            1..=4 => {
                wave_table = match option {
                    1 => &wave_table_sin,
                    2 => &wave_table_pulse,
                    3 => &wave_table_saw,
                    4 => &wave_table_supersaw,
                    _ => unreachable!(),
                };
                println!("Sound Playing! (2s)")
            }
            5 => break,
            _ => {
                println!("Invalid option!");
                continue
            }
        }
        let mut oscillator = Oscillator::new(44100, wave_table);
        oscillator.set_frequency(440.0);
        
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        
        let _result = stream_handle.play_raw(oscillator.convert_samples());
        
        std::thread::sleep(Duration::from_secs(2));
    }
}