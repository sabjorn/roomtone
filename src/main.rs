use std::env;
use std::fs::File;
use std::io::Read;
use roomtone::Config;

fn main() -> Result<(), serde_yaml::Error> {
    let args: Vec<_> = env::args().collect();
   
    let mut f = File::open(&args[1]).expect("Can't read YAML file");
    
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let config: Config = serde_yaml::from_str(&s)?;
    println!("{:?}", config);

    Ok(())
}

