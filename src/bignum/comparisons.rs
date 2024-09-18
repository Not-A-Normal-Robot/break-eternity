const F64_MAX_10_EXP: f64 = 308.254715559916743;

pub mod comparisons {
    use std::{cmp::Ordering, ops::Rem};

    use crate::bignum::{BigNum, Sign};

    use super::F64_MAX_10_EXP;

    impl BigNum {
        pub fn is_nan(&self) -> bool {
            self.layer.is_nan() ||
            self.magnitude.is_nan() ||
            self.layer < 0.0
        }

        pub fn is_finite(&self) -> bool {
            self.layer.is_finite() &&
            self.magnitude.is_finite()
        }

        pub fn is_infinite(&self) -> bool {
            self.layer.is_infinite() ||
            self.magnitude.is_infinite()
        }

        pub fn is_positive(&self) -> bool {
            self.sign == Sign::Positive
        }

        pub fn is_negative(&self) -> bool {
            self.sign == Sign::Negative
        }

        pub fn is_zero(&self) -> bool {
            self.magnitude == 0.0 &&
            self.layer == 0.0
        }

        pub fn is_normalized(&self) -> bool {
            // normally, layer is a whole number
            self.layer.rem(1.0) == 0.0 &&
            (
                // if layer is higher than it needs to be then it's not normalized
                // too high of a layer can lead to less precision
                self.layer == 0.0 ||
                self.magnitude >= F64_MAX_10_EXP
            )
        }
    }

    impl PartialEq for BigNum {
        fn eq(&self, other: &Self) -> bool {
            if self.is_nan() || other.is_nan() {
                return false;
            }

            if self.is_zero() && other.is_zero() {
                return true;
            }

            let normalized_self = self.normalize();
            let normalized_other = other.normalize();

            normalized_self.sign == normalized_other.sign &&
            normalized_self.magnitude == normalized_other.magnitude &&
            normalized_self.layer == normalized_other.layer
        }
    }

    impl PartialOrd for BigNum {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.is_nan() || other.is_nan() {
                return None;
            }

            if self.is_zero() && other.is_zero() {
                return Some(std::cmp::Ordering::Equal);
            }
            
            if self.sign != other.sign {
                if self.is_positive() {
                    return Some(std::cmp::Ordering::Greater);
                } else {
                    return Some(std::cmp::Ordering::Less);
                }
            }

            let sign = self.sign;

            let normalized_self = self.normalize();
            let normalized_other = other.normalize();

            let layer_ordering = normalized_self.layer.partial_cmp(&normalized_other.layer);

            match layer_ordering {
                Some(Ordering::Greater) => {
                    if(sign == Sign::Positive) {
                        return Some(Ordering::Greater);
                    } else {
                        return Some(Ordering::Less);
                    }
                },
                Some(Ordering::Less) => {
                    if(sign == Sign::Positive) {
                        return Some(Ordering::Less);
                    } else {
                        return Some(Ordering::Greater);
                    }
                },
                None => {
                    return None
                },

                Some(Ordering::Equal) => () // continue
            };

            let mag_ordering = normalized_self.magnitude.partial_cmp(&normalized_other.magnitude);

            return match mag_ordering {
                Some(Ordering::Greater) => {
                    if(sign == Sign::Positive) {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    }
                },
                Some(Ordering::Less) => {
                    if(sign == Sign::Positive) {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                },
                Some(Ordering::Equal) => {
                    Some(Ordering::Equal)
                },
                None => {
                    None
                }
            };
        }
    }
}