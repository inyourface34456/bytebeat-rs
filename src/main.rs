use hound::{WavSpec, WavWriter, SampleFormat};
use std::fs::File;
use std::env::args;
use std::io::Read;
use jstime_core as jstime;
// use rand::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    jstime::init(None);
    let mut scope = jstime::JSTime::new(
        jstime::Options::default()
    );

    let mut source = String::new();
    let mut file = File::open(args().into_iter().collect::<Vec<String>>()[1].clone()).expect("could not open file");
    file.read_to_string(&mut source).expect("could not read file");

    let _ = match scope.run_script(source.as_str(), "main") {
        Ok(_) => {},
        Err(err) => panic!("{err}"),
    };

    let discription: Vec<String> = match scope.run_script("d();", "main") {
        Ok(desc) => {
            desc.split(',').map(|x| x.to_string()).collect()
        },
        Err(err) => panic!("{err}"),
    };

    let sample_rate = match discription[0].parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("sample rate is NaN"),
    };

    let channels = match discription[1].parse::<u16>() {
        Ok(num) => num,
        Err(_) => panic!("number of channels is is NaN"),
    };

    let duration = match discription[2].parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("duration is is NaN"),
    };

    let kind = match discription[3].parse::<u32>() {
        Ok(num) => {
            if num>2 {
                panic!("kind out of range");
            }
            num
        },
        Err(_) => panic!("kind is is NaN"),
    };

    let sample_format = if kind==2 {SampleFormat::Float} else {SampleFormat::Int};
    let bits_per_sample = if kind==2 {32} else {8};

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
        sample_format,
    };
    let mut wav = WavWriter::create("sound.wav", spec)?;
    let duration = sample_rate*duration*(channels as u32);

    match kind {
        0 => {
            for t in 0..duration {
                let t = scope.run_script(format!("t({t})").as_str(), "main")
                .expect("ruhroh something went wrong").parse::<f32>().unwrap();
        
                //println!("{t}");
                
                wav.write_sample(t as i8)?;
            }
        },
        1 => {
            for t in 0..duration {
                let t = scope.run_script(format!("t({t})").as_str(), "main")
                .expect("ruhroh something went wrong").parse::<f32>().unwrap() as u8;
        
                //println!("{t}");
                
                wav.write_sample(t as i8)?;
            }
        },
        2 => {
            for t in 0..duration {
                let t = scope.run_script(format!("t({t})").as_str(), "main")
                .expect("ruhroh something went wrong").parse::<f32>().unwrap();
        
                //println!("{t}");
                
                wav.write_sample(t as f32)?;
            }
        },
        _ => unreachable!()
    }

    Ok(())
}