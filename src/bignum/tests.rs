mod tests {
    use crate::bignum::{
        BigNum,
        Sign::Positive as P,
        Sign::Negative as N
    };
    use std::f64::{
        NAN as NaN,
        INFINITY as INF,
        NEG_INFINITY as NINF
    };

    // #region Comparison tests
    #[test]
    fn test_is_nan() {
        let data = [
        //  Name                Sign  Mag  Layer  Expected
            ("NaN Layer",       P,    1.0, NaN,   true),
            ("Negative Layer",  P,    1.0, -1.0,  true),
            ("NaN Mag",         P,    NaN, 1.0,   true),
            ("Positive Layer",  P,    1.0, 2.0,   false)
        ];

        for (name, sign, mag, layer, expected) in data.iter() {
            let num = BigNum { sign: *sign, magnitude: *mag, layer: *layer };
            assert_eq!(num.is_nan(), *expected, "{name}");
        }
    }

    #[test]
    fn test_is_finite() {
        // A BigNum is only infinite if either the layer or magnitude is infinite.

        let data = [
        //  Name                Sign  Mag  Layer  Expected
            ("Finite layer",    P,    1.0, 1.0,   true),
            ("Infinite layer",  P,    1.0, INF,   false),
            ("Finite mag",      P,    1.0, 1.0,   true),
            ("Infinite mag",    P,    INF, 1.0,   false)
        ];

        for (name, sign, mag, layer, expected) in data.iter() {
            let num = BigNum { sign: *sign, magnitude: *mag, layer: *layer };
            assert_eq!(num.is_finite(), *expected, "{name}");
        }
    }
    // #endregion
}