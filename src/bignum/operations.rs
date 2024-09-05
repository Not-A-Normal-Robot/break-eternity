pub mod operations {
    use crate::bignum::{BigNum, Sign};

    impl BigNum {
        pub fn get_sign_as_num(&self) -> f64 {
            match self.sign {
                Sign::Positive => 1.0,
                Sign::Negative => -1.0
            }
        }

        pub fn normalize(&self) -> Self {
            if self.is_normalized() {
                return self.clone();
            }
            todo!()
        }

        pub fn normalize_mut(&mut self) {
            if !self.is_normalized() {
                *self = self.normalize();
            }
        }
    }
}