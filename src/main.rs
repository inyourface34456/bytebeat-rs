mod traits;

use crate::traits::TypeToFormat;
use hound::{WavSpec, WavWriter};
use std::mem::size_of;

type SampleType = i8;
const SAMPLE_RATE: u32 = 8000;
const BITS_PER_SAMPLE: u16 = (size_of::<SampleType>() * 8) as u16;
const NUM_CHANNELS: u16 = 1;
const DURATION: u64 = SAMPLE_RATE as u64 * 60;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: NUM_CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: SampleType::FORMAT,
    };

    let mut wav = WavWriter::create("sound.wav", spec)?;

    for t in 0..DURATION {
        let t = t as i64;

        let t = t*(t>>9&10|t>>11&24^t>>10&15&t>>15);

        wav.write_sample(t as SampleType)?;
    }

    Ok(())
}
