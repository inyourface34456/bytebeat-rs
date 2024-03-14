mod traits;

use crate::traits::TypeToFormat;
use hound::{WavSpec, WavWriter};
use std::mem::size_of;
// use rand::prelude::*;
use js_sandbox::{Script, AnyError};

type SampleType = f32;
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
        let mut t = t as f32;
        // let mut rng = thread_rng();

        let js_code = "function triple(t) { return t<2?(a=0,b=0,c=0):(a=.999*a+.001*random(),b<0?(b=.7*random(),c=random()):b-=1/44100,abs(256*a*(5*sin(t/5E4)+10)%256-128)+255*(t/300*(10*c+200)&2)*b**(random()/5+4))";
        // let mut script = Script::from_string(js_code)?;
        //t = script.call("triple", &t)?;


        wav.write_sample(t as SampleType)?;
    }

    Ok(())
}

// fn eval(code: &str, t: &SampleType) -> Result<SampleType, AnyError> {
//     Script::from_string(format!("function num(t) {{return {}}}", code).as_ref())?.call("num", t)
// }