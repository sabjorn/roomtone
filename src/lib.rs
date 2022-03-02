use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Channel {
    index: u8,
    output: u8,
    interval: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
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

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        let result = 2 + 2;
//        assert_eq!(result, 4);
//    }
//}
