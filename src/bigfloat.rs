#[derive(Debug, Clone, PartialEq)]
pub struct BigFloat {
    pub mantissa: f64,
    pub exponent: Exponent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exponent {
    Long(i64),
    BigFloat(Box<BigFloat>),
}

impl BigFloat {
    pub fn new(mantissa: f64, exponent: i64) -> Self {
        let mut bf = BigFloat {
            mantissa,
            exponent: Exponent::Long(exponent),
        };
        bf.normalize();
        bf
    }

    pub fn from_f64(value: f64) -> Self {
        if value == 0.0 {
            return BigFloat {
                mantissa: 0.0,
                exponent: Exponent::Long(0),
            };
        }
        
        if !value.is_finite() {
            return BigFloat {
                mantissa: value,
                exponent: Exponent::Long(0),
            };
        }

        let abs_value = value.abs();
        let log10_value = abs_value.log10();
        let exponent = log10_value.floor() as i64;
        let mantissa = abs_value / 10.0_f64.powi(exponent as i32);
        
        BigFloat {
            mantissa: if value.is_sign_negative() { -mantissa } else { mantissa },
            exponent: Exponent::Long(exponent),
        }
    }

    fn normalize(&mut self) {
        if self.mantissa == 0.0 || !self.mantissa.is_finite() {
            return;
        }

        let abs_mantissa = self.mantissa.abs();
        
        if abs_mantissa >= 10.0 {
            let log_mantissa = abs_mantissa.log10().floor();
            let adjustment = log_mantissa as i64;
            self.mantissa /= 10.0_f64.powi(adjustment as i32);
            self.adjust_exponent(adjustment);
        } else if abs_mantissa < 1.0 {
            let log_mantissa = (-abs_mantissa.log10()).ceil();
            let adjustment = -(log_mantissa as i64);
            self.mantissa /= 10.0_f64.powi(adjustment as i32);
            self.adjust_exponent(adjustment);
        }
    }

    fn adjust_exponent(&mut self, adjustment: i64) {
        match &mut self.exponent {
            Exponent::Long(exp) => {
                if let Some(new_exp) = exp.checked_add(adjustment) {
                    *exp = new_exp;
                } else {
                    let big_exp = BigFloat::new(*exp as f64, 0);
                    let _big_adj = BigFloat::new(adjustment as f64, 0);
                    // TODO: Implement addition for BigFloat
                    self.exponent = Exponent::BigFloat(Box::new(big_exp));
                }
            }
            Exponent::BigFloat(_) => {
                // TODO: Implement BigFloat addition for exponent adjustment
            }
        }
    }

    pub fn mantissa(&self) -> f64 {
        self.mantissa
    }

    pub fn exponent(&self) -> &Exponent {
        &self.exponent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_basic() {
        let bf = BigFloat::new(1.5, 3);
        assert_eq!(bf.mantissa(), 1.5);
        assert_eq!(bf.exponent(), &Exponent::Long(3));
    }

    #[test]
    fn test_new_normalization() {
        let bf = BigFloat::new(15.0, 2);
        assert!((bf.mantissa() - 1.5).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(3));
    }

    #[test]
    fn test_new_small_normalization() {
        let bf = BigFloat::new(0.15, 2);
        assert!((bf.mantissa() - 1.5).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(1));
    }

    #[test]
    fn test_from_f64_zero() {
        let bf = BigFloat::from_f64(0.0);
        assert_eq!(bf.mantissa(), 0.0);
        assert_eq!(bf.exponent(), &Exponent::Long(0));
    }

    #[test]
    fn test_from_f64_positive() {
        let bf = BigFloat::from_f64(123.45);
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_from_f64_negative() {
        let bf = BigFloat::from_f64(-123.45);
        assert!((bf.mantissa() + 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_from_f64_small() {
        let bf = BigFloat::from_f64(0.00123);
        assert!((bf.mantissa() - 1.23).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(-3));
    }

    #[test]
    fn test_from_f64_infinity() {
        let bf = BigFloat::from_f64(f64::INFINITY);
        assert_eq!(bf.mantissa(), f64::INFINITY);
        assert_eq!(bf.exponent(), &Exponent::Long(0));
    }

    #[test]
    fn test_from_f64_neg_infinity() {
        let bf = BigFloat::from_f64(f64::NEG_INFINITY);
        assert_eq!(bf.mantissa(), f64::NEG_INFINITY);
        assert_eq!(bf.exponent(), &Exponent::Long(0));
    }

    #[test]
    fn test_from_f64_nan() {
        let bf = BigFloat::from_f64(f64::NAN);
        assert!(bf.mantissa().is_nan());
        assert_eq!(bf.exponent(), &Exponent::Long(0));
    }
}