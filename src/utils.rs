pub fn to_db(gain: f32) -> f32 {
   20.0_f32 * gain.log10()
}

pub fn to_gain(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0_f32)
}

pub fn rms(buffer: &[f32]) -> f32 {
    let mean_squared: f32 = buffer.iter().map(|&x| x.powf(2.0)).sum();
    let mean = mean_squared / buffer.len() as f32;
    mean.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_db_produces_correct_value() {
        assert_eq!(to_db(1.0), 0.0);
        assert_eq!(to_db(0.5), -6.0206003);
    }

    #[test]
    pub fn to_gain_produces_correct_value() {
        assert_eq!(to_gain(0.0), 1.0);
        assert_eq!(to_gain(-6.0), 0.5011872);
    }

    #[test]
    pub fn to_db_and_to_gain_are_reciprocal() {
        let db = -6.0;
        let gain = to_gain(db);
        
        assert_eq!(to_db(gain), db);

        let gain = 2.0;
        let db = to_db(gain);

        assert_eq!(to_gain(db), gain);
    }

    #[test]
    pub fn rms_calculates_correctly() {
        let vec = vec![-1.0, 0.0, 1.0, 0.0];
        let rms = rms(&vec);

        assert_eq!(rms, 0.5_f32.sqrt());
    }
}

