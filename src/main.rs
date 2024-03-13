use hound::{WavWriter, WavSpec};
use std::mem::size_of;
// use std::f32::consts::PI;

type SampleType = i8;
const SAMPLE_RATE: u32 = 8000;
const BITS_PER_SAMPLE: u16 = (size_of::<SampleType>() as u16)*8;
const NUM_CHANNELS: u16 = 1;
const DURATION: u64 = SAMPLE_RATE as u64*10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: NUM_CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav = WavWriter::create("sound.wav", spec)?;
    
    for t in 0..DURATION {
        let mut t = t%256;
        // 10*(t>>6|t|t>>(t>>16))+(7&t>>11)

        // t = t.sin();
        if (127&t*(7&t>>10))<(245&t*(2+(5&t>>14))) {
            t = !t>>2;
        } else {
            t = 0
        }

        wav.write_sample(t as SampleType)?;
    }

    Ok(())
}
