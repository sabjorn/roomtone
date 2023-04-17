use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Channel {
    index: u8, // can just be vector index, not needed
    output: u8,
    interval: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    overload_thresh: f64,
    reset_thresh: f64,
    adc_input: u16, // sets input channel -- currently not used
    pub digital_gain: f64, // gain applied to ADC input
    pub output_gain: f64, // gain applied post delay
    max_delay: u64, // set maximum expected delay time -- can be just 2 x largest 
    base_delay_min: f64, // set minimum for random interval range -- not used
    base_delay_max: f64, // set maximum for random interval range -- not used
    base_delay_interval: f64, // the delay time (s)
    enable_random_interval: bool, // enables random -- not used
    fullscreen: bool, // fullscreen for max/msp -- not used
    channels: Vec<Channel>,
}

impl Config {
    pub fn get_max_interval(&self) -> u8 {
        let channels = &self.channels;
        let max = channels.iter().max_by_key(|&c| c.interval);
        match max {
            Some(channel) => { channel.interval },
            None => {0} 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_max_interval_returns_max_interval_from_channels_returns_correct_value() {
        let config = Config {
            overload_thresh: 0.0_f64,
            reset_thresh: 0.0_f64,
            adc_input: 0_u16,
            digital_gain: 0.0_f64,
            output_gain: 0.0_f64,
            max_delay: 0_u64,
            base_delay_min: 0.0_f64,
            base_delay_max: 0.0_f64,
            base_delay_interval: 0.0_f64,
            enable_random_interval: false,
            fullscreen: false,
            channels: vec![
                Channel {index: 0_u8, output: 0_u8, interval: 4_u8},
                Channel {index: 0_u8, output: 0_u8, interval: 12_u8},
                Channel {index: 0_u8, output: 0_u8, interval: 9_u8},],
        };

        let max_interval = config.get_max_interval();
        assert_eq!(max_interval, 12_u8);
    }
}

