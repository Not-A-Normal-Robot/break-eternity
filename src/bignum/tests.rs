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

    #[test]
    fn test_equality() {
        let data = [
        //  Name                 Sign 1  Mag 1  Layer 1  Sign 2  Mag 2  Layer 2  Expected
            ("Equal",            P,      1.0,   0.0,     P,      1.0,   0.0,     true),
            ("Different sign",   P,      1.0,   0.0,     N,      1.0,   0.0,     false),
            ("Different mag",    P,      1.0,   0.0,     P,      2.0,   0.0,     false),
            ("Different layer",  P,      1.0,   0.0,     P,      1.0,   2.0,     false),
            ("Signed Zeroes",    P,      0.0,   0.0,     N,      0.0,   0.0,     true),
            ("NaNs 1/10",        P,      NaN,   0.0,     P,      0.0,   0.0,     false),
            ("NaNs 2/10",        P,      0.0,   NaN,     P,      0.0,   0.0,     false),
            ("NaNs 3/10",        N,      0.0,   0.0,     P,      NaN,   0.0,     false),
            ("NaNs 4/10",        N,      0.0,   0.0,     N,      0.0,   NaN,     false),
            ("NaNs 5/10",        P,      NaN,   0.0,     P,      NaN,   0.0,     false),
            ("NaNs 6/10",        P,      0.0,   NaN,     N,      0.0,   NaN,     false),
            ("NaNs 7/10",        P,      NaN,   0.0,     P,      0.0,   NaN,     false),
            ("NaNs 8/10",        N,      0.0,   NaN,     N,      NaN,   0.0,     false),
            ("NaNs 9/10",        N,      0.0,   NaN,     P,      0.0,   NaN,     false),
            ("NaNs 10/10",       P,      NaN,   0.0,     N,      NaN,   0.0,     false),
            ("Infinities 1/10",  P,      INF,   INF,     P,      INF,   INF,     true),
            ("Infinities 2/10",  P,      INF,   INF,     N,      INF,   INF,     false),
            ("Infinities 3/10",  P,      INF,   INF,     P,      NINF,  NINF,    false),
            ("Infinities 4/10",  P,      INF,   INF,     N,      NINF,  NINF,    false),
            ("Infinities 5/10",  P,      INF,   INF,     P,      INF,   NINF,    false),
            ("Infinities 6/10",  P,      INF,   INF,     N,      INF,   NINF,    false),
            ("Infinities 7/10",  N,      NINF,  NINF,    P,      NINF,  NINF,    false),
            ("Infinities 8/10",  N,      NINF,  NINF,    N,      NINF,  NINF,    true),
            ("Infinities 9/10",  N,      NINF,  NINF,    P,      NINF,  INF,     false),
            ("Infinities 10/10", N,      NINF,  NINF,    N,      NINF,  INF,     false)
            
        ];

        for (name, s1, m1, l1, s2, m2, l2, expected) in data.iter() {
            let num1 = BigNum { sign: *s1, magnitude: *m1, layer: *l1 };
            let num2 = BigNum { sign: *s2, magnitude: *m2, layer: *l2 };
            assert_eq!(num1 == num2, *expected, "{name}");
        }
    }
    // #endregion
}