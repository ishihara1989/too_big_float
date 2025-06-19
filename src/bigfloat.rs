#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BigFloat {
    pub mantissa: f64,
    pub exponent: u128,
}

impl BigFloat {
    pub fn new(mantissa: f64, exponent: u128) -> Self {
        let mut bf = BigFloat {
            mantissa,
            exponent,
        };
        bf.normalize();
        bf
    }

    pub fn new_from_i64_exponent(mantissa: f64, exponent: i64) -> Self {
        if exponent < 0 {
            // For negative exponents, we represent as mantissa with exponent 0
            let scaled_mantissa = mantissa * 10.0_f64.powi(exponent as i32);
            BigFloat {
                mantissa: scaled_mantissa,
                exponent: 0,
            }
        } else {
            Self::new(mantissa, exponent as u128)
        }
    }


    pub fn from_f64(value: f64) -> Self {
        if value == 0.0 {
            return BigFloat {
                mantissa: 0.0,
                exponent: 0,
            };
        }
        
        if !value.is_finite() {
            return BigFloat {
                mantissa: value,
                exponent: 0,
            };
        }

        let abs_value = value.abs();
        if abs_value < 1.0 {
            // For small numbers, keep exponent 0 and allow mantissa < 1
            BigFloat {
                mantissa: value,
                exponent: 0,
            }
        } else {
            // For large numbers, normalize to mantissa >= 1 and < 10
            let log10_value = abs_value.log10();
            let exponent = log10_value.floor() as u128;
            let mantissa = abs_value / 10.0_f64.powi(exponent as i32);
            
            BigFloat {
                mantissa: if value.is_sign_negative() { -mantissa } else { mantissa },
                exponent,
            }
        }
    }

    fn normalize(&mut self) {
        if self.mantissa == 0.0 || !self.mantissa.is_finite() {
            return;
        }

        let abs_mantissa = self.mantissa.abs();
        
        if self.exponent == 0 {
            // When exponent is 0, allow mantissa to be < 1 for fractional numbers
            if abs_mantissa >= 10.0 {
                // Only normalize if mantissa >= 10
                let log_mantissa = abs_mantissa.log10().floor();
                let adjustment = log_mantissa as u128;
                self.mantissa /= 10.0_f64.powi(adjustment as i32);
                self.exponent = adjustment;
            }
            // Do nothing if mantissa < 1 when exponent is 0
        } else {
            // For non-zero exponents, maintain standard normalization (1 <= |mantissa| < 10)
            if abs_mantissa >= 10.0 {
                let log_mantissa = abs_mantissa.log10().floor();
                let adjustment = log_mantissa as u128;
                self.mantissa /= 10.0_f64.powi(adjustment as i32);
                self.exponent += adjustment;
            } else if abs_mantissa < 1.0 {
                // Move to exponent 0 and allow fractional mantissa
                let scale_factor = 10.0_f64.powi(self.exponent as i32);
                self.mantissa *= scale_factor;
                self.exponent = 0;
            }
        }
    }


    pub fn mantissa(&self) -> f64 {
        self.mantissa
    }

    pub fn exponent(&self) -> u128 {
        self.exponent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_basic() {
        let bf = BigFloat::new(1.5, 3);
        assert_eq!(bf.mantissa(), 1.5);
        assert_eq!(bf.exponent(), 3);
    }

    #[test]
    fn test_new_normalization() {
        let bf = BigFloat::new(15.0, 2);
        assert!((bf.mantissa() - 1.5).abs() < 1e-10);
        assert_eq!(bf.exponent(), 3);
    }

    #[test]
    fn test_new_small_normalization() {
        let bf = BigFloat::new(0.15, 2);
        // With new logic: if exponent != 0 and mantissa < 1, move to exponent 0
        // 0.15 * 10^2 = 15, so it should be stored as mantissa 15 with exponent 0
        assert!((bf.mantissa() - 15.0).abs() < 1e-10);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_from_f64_zero() {
        let bf = BigFloat::from_f64(0.0);
        assert_eq!(bf.mantissa(), 0.0);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_from_f64_positive() {
        let bf = BigFloat::from_f64(123.45);
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), 2);
    }

    #[test]
    fn test_from_f64_negative() {
        let bf = BigFloat::from_f64(-123.45);
        assert!((bf.mantissa() + 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), 2);
    }

    #[test]
    fn test_from_f64_small() {
        let bf = BigFloat::from_f64(0.00123);
        // Small numbers keep exponent 0 and allow mantissa < 1
        assert!((bf.mantissa() - 0.00123).abs() < 1e-10);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_from_f64_infinity() {
        let bf = BigFloat::from_f64(f64::INFINITY);
        assert_eq!(bf.mantissa(), f64::INFINITY);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_from_f64_neg_infinity() {
        let bf = BigFloat::from_f64(f64::NEG_INFINITY);
        assert_eq!(bf.mantissa(), f64::NEG_INFINITY);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_from_f64_nan() {
        let bf = BigFloat::from_f64(f64::NAN);
        assert!(bf.mantissa().is_nan());
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_small_fraction() {
        let bf = BigFloat::new(0.123, 0);
        assert_eq!(bf.mantissa(), 0.123);
        assert_eq!(bf.exponent(), 0);
    }

    #[test]
    fn test_large_number_normalization() {
        let bf = BigFloat::new(1234.5, 0);
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), 3);
    }
}