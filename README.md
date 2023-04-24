# Implemenation of Wave Table Synthesis algorihtm in Rust

I tried implementing the wave table synthesizer in rust with the help of some awesome Sound Engineer nerds😉.

In a nutshell, we are generating music digitally unlike via classical instruments. But how do we actually do them?

THIS is one of the techniques and here i implemented it in Rust. The reason i chose Rust programming language is that audio processing is somewhat a computationally intensive work, considering calling a function for each phase of numerous waveforms (Here just one) is kind of a heavy duty. Thus Rust fits the description pretty well when compared to other languages.

In summary, the algorithm states that wavetable is just an array in memory which stores a fragment of a waveform. In this code sample, a Sine wave generator is constructed using which the wavetable is initialized. After this, the algorithm of wave table synthesis follows.

In brief, an oscillator is constructed with various parts to construct the working model of wavetable synthesis technique. Then it is used to produce a Sine wave for about 2 seconds (The duration of the main thread is cut after 2 seconds rather than giving it as an argument to the Oscillator).

I plan to expand on this topic stretching to a full on audio-processing application implementing different paradigms like web assembly and machine learning later on in this rust run :)

### Follow the steps to run locally

1. Git clone the project in your local machine.
2. cd into the git directory and open the terminal.
3. Run `cargo run` command in the terminal.
