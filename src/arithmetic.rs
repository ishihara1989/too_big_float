use crate::bigfloat::BigFloat;
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
        assert_eq!(result.exponent(), 2);
    }

    #[test]
    fn test_add_different_exponents() {
        let a = BigFloat::new(1.0, 2);  // 100
        let b = BigFloat::new(1.0, 0);  // 1
        let result = a + b;  // 101
        assert!((result.mantissa() - 1.01).abs() < 1e-10);
        assert_eq!(result.exponent(), 2);
    }

    #[test]
    fn test_add_zero() {
        let a = BigFloat::new(1.5, 2);
        let b = BigFloat::from_f64(0.0);
        let result = a + b;
        assert_eq!(result.mantissa(), 1.5);
        assert_eq!(result.exponent(), 2);
    }

    #[test]
    fn test_sub_basic() {
        let a = BigFloat::new(3.0, 2);  // 300
        let b = BigFloat::new(1.0, 2);  // 100
        let result = a - b;  // 200
        assert!((result.mantissa() - 2.0).abs() < 1e-10);
        assert_eq!(result.exponent(), 2);
    }

    #[test]
    fn test_mul_basic() {
        let a = BigFloat::new(2.0, 2);  // 200
        let b = BigFloat::new(3.0, 1);  // 30
        let result = a * b;  // 6000
        assert!((result.mantissa() - 6.0).abs() < 1e-10);
        assert_eq!(result.exponent(), (3));
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
        assert_eq!(result.exponent(), 2);
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
        assert_eq!(result.exponent(), (300));
    }
}

impl BigFloat {
    pub fn is_zero(&self) -> bool {
        self.mantissa == 0.0
    }

    pub fn is_finite(&self) -> bool {
        self.mantissa.is_finite()
    }

    pub fn compare_exponents(&self, other: &BigFloat) -> std::cmp::Ordering {
        self.exponent.cmp(&other.exponent)
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
                BigFloat::new(new_mantissa, self.exponent)
            }
            std::cmp::Ordering::Greater => {
                // self has larger exponent
                let exp_diff = self.exponent - other.exponent;
                if exp_diff > 15 {
                    // Other number is too small to affect the result
                    return self;
                }
                let scale_factor = 10.0_f64.powi(exp_diff as i32);
                let scaled_other = other.mantissa / scale_factor;
                let new_mantissa = self.mantissa + scaled_other;
                BigFloat::new(new_mantissa, self.exponent)
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
        let new_exp = self.exponent + other.exponent;
        
        BigFloat::new(new_mantissa, new_exp)
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
        
        // Handle underflow case where other.exponent > self.exponent
        if other.exponent > self.exponent {
            // Result would have negative exponent, so we scale mantissa and use exponent 0
            let exp_diff = other.exponent - self.exponent;
            let scaled_mantissa = new_mantissa / 10.0_f64.powi(exp_diff as i32);
            BigFloat::new(scaled_mantissa, 0)
        } else {
            let new_exp = self.exponent - other.exponent;
            BigFloat::new(new_mantissa, new_exp)
        }
    }
}