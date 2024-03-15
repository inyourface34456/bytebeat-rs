mod traits;

use crate::traits::TypeToFormat;
use hound::{WavSpec, WavWriter};
use std::mem::size_of;
// use rand::prelude::*;

type SampleType = i8;
const SAMPLE_RATE: u32 = 8000;
const BITS_PER_SAMPLE: u16 = (size_of::<SampleType>() * 8) as u16;
const NUM_CHANNELS: u16 = 1;
const DURATION: u64 = SAMPLE_RATE as u64 * 60*NUM_CHANNELS as u64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: NUM_CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: SampleType::FORMAT,
    };
    let mut wav = WavWriter::create("sound.wav", spec)?;
    let mut channel: u64 = 0;

    for t in 0..DURATION {
        // let mut t = t as f32;
        // if channel > NUM_CHANNELS as u64 {
        //     channel = 0;
        //     t = t.sin();
        // } else {
        //     channel += 1;
        //     t = t.cos();
        // }

        let t = t>>5|(t>>2)*(t>>5);
        wav.write_sample(t as SampleType)?;
    }

    Ok(())
}