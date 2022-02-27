use std::env;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Channel {
    index: u8,
    output: u8,
    interval: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    overload_thresh: f64,
    reset_thresh: f64,
    adc_input: u16,
    digital_gain: f64,
    output_gain: f64,
    max_delay: u64,
    base_delay_min: f64,
    base_delay_max: f64,
    base_delay_interval: f64,
    enable_random_interval: bool,
    fullscreen: bool,
    channels: Vec<Channel>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let args: Vec<_> = env::args().collect();
   
    let mut f = File::open(&args[1]).expect("Can't read YAML file");
    
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let config: Config = serde_yaml::from_str(&s)?;
    println!("{:?}", config);

    Ok(())
}

