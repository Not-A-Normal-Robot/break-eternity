pub mod comparisons {
    use std::ops::Rem;

    use crate::bignum::{BigNum, Sign};

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
            self.layer.rem(1.0) == 0.0
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

            let normalizedSelf = self.normalize();
            let normalizedOther = other.normalize();

            normalizedSelf.sign == normalizedOther.sign &&
            normalizedSelf.magnitude == normalizedOther.magnitude &&
            normalizedSelf.layer == normalizedOther.layer
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

            todo!()
        }
    }
}