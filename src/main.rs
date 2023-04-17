use std::fs::File;
use std::io::Read;

use anyhow;
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapRb;

use roomtone::Config;
mod utils;
use utils::{to_gain, to_db, rms};

mod multitap;
use multitap::WriteHead;

#[derive(Parser, Debug)]
#[command(version, about = "CPAL feedback example", long_about = None)]
struct Opt {
    #[arg(short = 'v', long, value_name = "LIST_DEVICES", default_value_t = false)]
    list_devices: bool,

    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    input_device: String,

    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    output_device: String,

    #[arg(short, long, value_name = "DELAY_MS", default_value_t = 512)]
    latency: usize,

    #[arg(short = 'c', long = "config_file", required=true, value_name = "CONFIG_FILE")]
    config_file: Option<String>,

    /// Use the JACK host
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}

fn setup_host(_opt: &Opt) -> cpal::Host {
    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    if _opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    }

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    cpal::default_host()
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let yaml_config =  {
            let filename = opt.config_file.clone().expect("this should never happen as config_file is required");
            let mut f = File::open(filename).expect("Can't read YAML file");
    
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();

            let config: Config = serde_yaml::from_str(&s)?;
            config
        }; 
    
    let host = setup_host(&opt);

    if opt.list_devices {
        println!("input devices");
        for input_device in host.input_devices()? {
            println!("{}", input_device.name()?);
        }
        println!("output_device devices");
        for output_device in host.output_devices()? {
            println!("{}", output_device.name()?);
        }
        return Ok(())
    }

    // Find devices.
    let input_device = if opt.input_device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.input_device).unwrap_or(false))
    }
    .expect("failed to find input device");

    let output_device = if opt.output_device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.output_device).unwrap_or(false))
    }
    .expect("failed to find output device");

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    let latency_samples = opt.latency * config.channels as usize;

    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    for _ in 0..latency_samples {
        producer.push(0.0).unwrap();
    }
    
    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        producer.push_slice(data);
    };
    
    let fs = config.sample_rate.0 as usize;
    let maximum_delay = yaml_config.get_max_interval() as f64 * yaml_config.base_delay_interval * fs as f64 * 2.0_f64;
    let mut write_head = WriteHead::new(maximum_delay as usize);

    let channel_delay = (yaml_config.channels[0].interval as f64 * yaml_config.base_delay_interval * fs as f64 * 2.0_f64) - latency_samples as f64;
    let mut read_head = write_head.as_readhead(channel_delay as usize); 

    let input_gain = to_gain(yaml_config.digital_gain as f32);
    let output_gain = to_gain(yaml_config.output_gain as f32);

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let rms = to_db(rms(data));
        if rms > yaml_config.reset_thresh as f32 {
            println!("rms {} exceeded thresh {}", rms, yaml_config.reset_thresh as f32);
            write_head.clear();
        };
        
        //let peak: f32 = data.iter().map(|&x| x.abs()).max();

        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => {
                    let x = input_gain * s;
                    write_head.push(x);
                    // yaml_config.overload_thresh, peak? -- kill program?

                    // yaml_config.reset_thresh, clear
                    let x_1 = read_head.next().unwrap() * output_gain;
                    
                    x_1
                }
                None => {
                    eprintln!("input stream fell behind: try increasing latency");
                    0.0
                }
            };
        }
    };

    // Build streams.
    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
         (1000.0 /  config.sample_rate.0 as f32) * (opt.latency as f32)
    );
    input_stream.play()?;
    output_stream.play()?;

    println!("Playing");
    std::thread::park();
    drop(input_stream);
    drop(output_stream);
    println!("Done!");
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
