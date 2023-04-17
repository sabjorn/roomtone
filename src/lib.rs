use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub interval: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub overload_thresh: f64,
    pub reset_thresh: f64,
    pub digital_gain: f64, // gain applied to ADC input
    pub output_gain: f64, // gain applied post delay
    pub base_delay_interval: f64, // the delay time (s)
    pub channels: Vec<Channel>,
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
            digital_gain: 0.0_f64,
            output_gain: 0.0_f64,
            base_delay_interval: 0.0_f64,
            channels: vec![
                Channel {interval: 4_u8},
                Channel {interval: 12_u8},
                Channel {interval: 9_u8},],
        };

        let max_interval = config.get_max_interval();
        assert_eq!(max_interval, 12_u8);
    }
}

