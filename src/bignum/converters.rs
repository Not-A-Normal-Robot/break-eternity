pub mod converters {
    use std::ops::Rem;

    use crate::bignum::{BigNum, Sign};

    impl BigNum {
        pub fn from_components(sign: Sign, magnitude: f64, layer: f64) -> BigNum {
            BigNum { sign, magnitude, layer }
        }

        pub fn from_f64(value: f64) -> BigNum {
            let sign = if value < 0.0 { Sign::Negative } else { Sign::Positive };
            let magnitude = value.abs();
            BigNum { sign, magnitude, layer: 0.0 }
        }

        pub fn from_exponent(exponent: f64) -> BigNum {
            let magnitude = exponent.log10();
            BigNum { sign: Sign::Positive, magnitude, layer: 1.0 }
        }

        /// Creates a BigNum from the height of a 10^10^... power tower.
        /// In other words, creates a BigNum instance that represents the value of 10 tetrated by `tower_height`.
        pub fn from_power_tower(tower_height: f64) -> BigNum {
            let layer = tower_height.floor();

            let remainder = tower_height.rem(1.0).abs();
            let magnitude_exp = std::f64::MAX.log10() * remainder;
            let magnitude = 10_f64.powf(magnitude_exp);
            
            BigNum {
                sign: Sign::Positive,
                magnitude,
                layer
            }
        }

        pub fn to_f64(&self) -> f64 {
            if self.layer.is_nan() || self.magnitude.is_nan() || self.layer < 0.0 {
                return std::f64::NAN;
            }

            if self.layer >= 4.0 {
                if self.sign == Sign::Positive {
                    return std::f64::INFINITY;
                } else {
                    return std::f64::NEG_INFINITY;
                }
            }

            let mut result = self.magnitude;

            for _ in 0..(self.layer as i8) {
                result = 10_f64.powf(result);
            }

            result *= self.get_sign_as_num();

            return result;
        }
    }
}