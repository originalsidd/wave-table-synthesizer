pub mod wave_table_oscillator {
	use core::time::Duration;
	use rodio::{source::Source};

	pub struct Oscillator {
		sample_rate: u32,
		wave_table: Vec<f32>,
		index: f32,
		index_step: f32,
	}
	
	impl Oscillator {
		pub fn new(sample_rate: u32, wave_table: &Vec<f32>) -> Oscillator {
			Oscillator {
				sample_rate: sample_rate,
				wave_table: wave_table.to_vec(),
				index: 0.0,
				index_step: 0.0,
			}
		}
		
		pub fn set_frequency(&mut self, frequency: f32) {
			self.index_step = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
		}
		
		fn get_sample(&mut self) -> f32 {
			let sample: f32 = self.lerp();
			self.index += self.index_step;
			self.index %= self.wave_table.len() as f32;
			sample
		}
		
		fn lerp(&self) -> f32 {
			let round_index: usize = self.index as usize;
			let next_index: usize = (round_index + 1) % self.wave_table.len();
			
			let next_index_weight: f32 = self.index - round_index as f32;
			let round_index_weight: f32 = 1.0 - next_index_weight;

			round_index_weight * self.wave_table[round_index] + next_index_weight * self.wave_table[next_index]
		}
	}

	impl Iterator for Oscillator {
		type Item = f32;
		
		fn next(&mut self) -> Option<f32> {
			Some(self.get_sample())
		}
	}

	impl Source for Oscillator {
		fn channels(&self) -> u16 {
			1
		}
		
		fn sample_rate(&self) -> u32 {
			self.sample_rate
		}
		
		fn current_frame_len(&self) -> Option<usize> {
			None
		}
		
		fn total_duration(&self) -> Option<Duration> {
			None
		}
	}
}