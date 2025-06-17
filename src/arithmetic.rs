use crate::bigfloat::{BigFloat, Exponent};
use std::ops::{Add, Sub, Mul, Div};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_basic() {
        let a = BigFloat::new(1.5, 2);  // 150
        let b = BigFloat::new(2.5, 2);  // 250
        let result = a + b;  // 400
        assert!((result.mantissa() - 4.0).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_add_different_exponents() {
        let a = BigFloat::new(1.0, 2);  // 100
        let b = BigFloat::new(1.0, 0);  // 1
        let result = a + b;  // 101
        assert!((result.mantissa() - 1.01).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_add_zero() {
        let a = BigFloat::new(1.5, 2);
        let b = BigFloat::from_f64(0.0);
        let result = a + b;
        assert_eq!(result.mantissa(), 1.5);
        assert_eq!(result.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_sub_basic() {
        let a = BigFloat::new(3.0, 2);  // 300
        let b = BigFloat::new(1.0, 2);  // 100
        let result = a - b;  // 200
        assert!((result.mantissa() - 2.0).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_mul_basic() {
        let a = BigFloat::new(2.0, 2);  // 200
        let b = BigFloat::new(3.0, 1);  // 30
        let result = a * b;  // 6000
        assert!((result.mantissa() - 6.0).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(3));
    }

    #[test]
    fn test_mul_zero() {
        let a = BigFloat::new(2.0, 2);
        let b = BigFloat::from_f64(0.0);
        let result = a * b;
        assert_eq!(result.mantissa(), 0.0);
    }

    #[test]
    fn test_div_basic() {
        let a = BigFloat::new(6.0, 3);  // 6000
        let b = BigFloat::new(2.0, 1);  // 20
        let result = a / b;  // 300
        assert!((result.mantissa() - 3.0).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_div_by_zero() {
        let a = BigFloat::new(1.0, 0);
        let b = BigFloat::from_f64(0.0);
        let result = a / b;
        assert_eq!(result.mantissa(), f64::INFINITY);
    }

    #[test]
    fn test_very_large_numbers() {
        let a = BigFloat::new(1.0, 100);  // 1e100
        let b = BigFloat::new(1.0, 200);  // 1e200
        let result = a * b;  // 1e300
        assert!((result.mantissa() - 1.0).abs() < 1e-10);
        assert_eq!(result.exponent(), &Exponent::Long(300));
    }
}

impl BigFloat {
    pub fn is_zero(&self) -> bool {
        self.mantissa == 0.0
    }

    pub fn is_finite(&self) -> bool {
        self.mantissa.is_finite()
    }

    fn exponent_as_i64(&self) -> Option<i64> {
        match &self.exponent {
            Exponent::Long(exp) => Some(*exp),
            Exponent::BigFloat(_) => None,
        }
    }

    pub fn compare_exponents(&self, other: &BigFloat) -> std::cmp::Ordering {
        match (&self.exponent, &other.exponent) {
            (Exponent::Long(a), Exponent::Long(b)) => a.cmp(b),
            (Exponent::Long(_), Exponent::BigFloat(_)) => std::cmp::Ordering::Less,
            (Exponent::BigFloat(_), Exponent::Long(_)) => std::cmp::Ordering::Greater,
            (Exponent::BigFloat(_), Exponent::BigFloat(_)) => {
                // TODO: Implement BigFloat comparison
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl Add for BigFloat {
    type Output = BigFloat;

    fn add(self, other: BigFloat) -> BigFloat {
        if self.is_zero() {
            return other;
        }
        if other.is_zero() {
            return self;
        }

        if !self.is_finite() || !other.is_finite() {
            return BigFloat::from_f64(self.mantissa + other.mantissa);
        }

        let exp_cmp = self.compare_exponents(&other);
        
        match exp_cmp {
            std::cmp::Ordering::Equal => {
                let new_mantissa = self.mantissa + other.mantissa;
                if let Some(exp) = self.exponent_as_i64() {
                    BigFloat::new(new_mantissa, exp)
                } else {
                    // Handle BigFloat exponent case
                    BigFloat {
                        mantissa: new_mantissa,
                        exponent: self.exponent.clone(),
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                // self has larger exponent
                if let (Some(self_exp), Some(other_exp)) = (self.exponent_as_i64(), other.exponent_as_i64()) {
                    let exp_diff = self_exp - other_exp;
                    if exp_diff > 15 {
                        // Other number is too small to affect the result
                        return self;
                    }
                    let scale_factor = 10.0_f64.powi(exp_diff as i32);
                    let scaled_other = other.mantissa / scale_factor;
                    let new_mantissa = self.mantissa + scaled_other;
                    BigFloat::new(new_mantissa, self_exp)
                } else {
                    // Handle BigFloat case
                    self
                }
            }
            std::cmp::Ordering::Less => {
                // other has larger exponent
                other.add(self)
            }
        }
    }
}

impl Sub for BigFloat {
    type Output = BigFloat;

    fn sub(self, other: BigFloat) -> BigFloat {
        let neg_other = BigFloat {
            mantissa: -other.mantissa,
            exponent: other.exponent,
        };
        self.add(neg_other)
    }
}

impl Mul for BigFloat {
    type Output = BigFloat;

    fn mul(self, other: BigFloat) -> BigFloat {
        if self.is_zero() || other.is_zero() {
            return BigFloat::from_f64(0.0);
        }

        if !self.is_finite() || !other.is_finite() {
            let result = self.mantissa * other.mantissa;
            return BigFloat::from_f64(result);
        }

        let new_mantissa = self.mantissa * other.mantissa;
        
        match (&self.exponent, &other.exponent) {
            (Exponent::Long(exp1), Exponent::Long(exp2)) => {
                if let Some(new_exp) = exp1.checked_add(*exp2) {
                    BigFloat::new(new_mantissa, new_exp)
                } else {
                    // Exponent overflow, convert to BigFloat exponent
                    let big_exp1 = BigFloat::new(*exp1 as f64, 0);
                    let big_exp2 = BigFloat::new(*exp2 as f64, 0);
                    let result_exp = big_exp1.add(big_exp2);
                    BigFloat {
                        mantissa: new_mantissa,
                        exponent: Exponent::BigFloat(Box::new(result_exp)),
                    }
                }
            }
            _ => {
                // TODO: Handle BigFloat exponent cases
                BigFloat::new(new_mantissa, 0)
            }
        }
    }
}

impl Div for BigFloat {
    type Output = BigFloat;

    fn div(self, other: BigFloat) -> BigFloat {
        if other.is_zero() {
            return BigFloat::from_f64(f64::INFINITY);
        }

        if self.is_zero() {
            return BigFloat::from_f64(0.0);
        }

        if !self.is_finite() || !other.is_finite() {
            let result = self.mantissa / other.mantissa;
            return BigFloat::from_f64(result);
        }

        let new_mantissa = self.mantissa / other.mantissa;
        
        match (&self.exponent, &other.exponent) {
            (Exponent::Long(exp1), Exponent::Long(exp2)) => {
                if let Some(new_exp) = exp1.checked_sub(*exp2) {
                    BigFloat::new(new_mantissa, new_exp)
                } else {
                    // Handle overflow case
                    let big_exp1 = BigFloat::new(*exp1 as f64, 0);
                    let big_exp2 = BigFloat::new(*exp2 as f64, 0);
                    let result_exp = big_exp1.sub(big_exp2);
                    BigFloat {
                        mantissa: new_mantissa,
                        exponent: Exponent::BigFloat(Box::new(result_exp)),
                    }
                }
            }
            _ => {
                // TODO: Handle BigFloat exponent cases
                BigFloat::new(new_mantissa, 0)
            }
        }
    }
}