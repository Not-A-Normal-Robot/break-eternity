pub mod bignum {
    pub mod converters;
    pub mod comparisons;
    pub mod operations;
    pub mod constants;
    pub mod misc;

    mod tests;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Sign { Positive, Negative }

    #[derive(Clone, Debug)]
    pub struct BigNum {
        pub sign: Sign,
        pub magnitude: f64,
        pub layer: f64
    }
}