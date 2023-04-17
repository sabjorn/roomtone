pub fn to_db(gain: f32) -> f32 {
   20.0_f32 * gain.log10()
}

pub fn to_gain(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0_f32)
}

#[cfg(test)]
mod tests {
    use crate::{to_db, to_gain};

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
}

