use crate::bigfloat::{BigFloat, Exponent};
use std::cmp::Ordering;

impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Handle NaN cases
        if self.mantissa.is_nan() || other.mantissa.is_nan() {
            return None;
        }

        // Handle infinity cases
        if self.mantissa.is_infinite() || other.mantissa.is_infinite() {
            return self.mantissa.partial_cmp(&other.mantissa);
        }

        // Handle zero cases
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        if self.is_zero() {
            return Some(if other.mantissa > 0.0 { Ordering::Less } else { Ordering::Greater });
        }
        if other.is_zero() {
            return Some(if self.mantissa > 0.0 { Ordering::Greater } else { Ordering::Less });
        }

        // Compare signs first
        let self_sign = self.mantissa.is_sign_positive();
        let other_sign = other.mantissa.is_sign_positive();
        
        match (self_sign, other_sign) {
            (true, false) => return Some(Ordering::Greater),
            (false, true) => return Some(Ordering::Less),
            _ => {} // Same sign, continue with magnitude comparison
        }

        // Compare exponents
        let exp_cmp = self.compare_exponents(other);
        
        match exp_cmp {
            Ordering::Equal => {
                // Same exponent, compare mantissas
                if self_sign {
                    self.mantissa.partial_cmp(&other.mantissa)
                } else {
                    other.mantissa.partial_cmp(&self.mantissa) // Flip for negative numbers
                }
            }
            Ordering::Greater => {
                // self has larger exponent
                if self_sign {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            Ordering::Less => {
                // other has larger exponent
                if self_sign {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Eq for BigFloat {}

impl Ord for BigFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl BigFloat {
    pub fn abs(&self) -> BigFloat {
        if self.mantissa < 0.0 {
            BigFloat {
                mantissa: -self.mantissa,
                exponent: self.exponent.clone(),
            }
        } else {
            self.clone()
        }
    }

    pub fn signum(&self) -> BigFloat {
        if self.mantissa > 0.0 {
            BigFloat::from_f64(1.0)
        } else if self.mantissa < 0.0 {
            BigFloat::from_f64(-1.0)
        } else {
            BigFloat::from_f64(0.0)
        }
    }

    pub fn is_sign_positive(&self) -> bool {
        self.mantissa.is_sign_positive()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.mantissa.is_sign_negative()
    }

    pub fn min(self, other: BigFloat) -> BigFloat {
        if self <= other { self } else { other }
    }

    pub fn max(self, other: BigFloat) -> BigFloat {
        if self >= other { self } else { other }
    }
}

// Implement Default
impl Default for BigFloat {
    fn default() -> Self {
        BigFloat::from_f64(0.0)
    }
}

// Add assignment operators
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

impl AddAssign for BigFloat {
    fn add_assign(&mut self, other: BigFloat) {
        *self = self.clone() + other;
    }
}

impl SubAssign for BigFloat {
    fn sub_assign(&mut self, other: BigFloat) {
        *self = self.clone() - other;
    }
}

impl MulAssign for BigFloat {
    fn mul_assign(&mut self, other: BigFloat) {
        *self = self.clone() * other;
    }
}

impl DivAssign for BigFloat {
    fn div_assign(&mut self, other: BigFloat) {
        *self = self.clone() / other;
    }
}

// Implement Neg
use std::ops::Neg;

impl Neg for BigFloat {
    type Output = BigFloat;

    fn neg(self) -> BigFloat {
        BigFloat {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_ord_basic() {
        let a = BigFloat::new(1.0, 2); // 100
        let b = BigFloat::new(2.0, 1); // 20
        assert!(a > b);
        assert!(b < a);
    }

    #[test]
    fn test_partial_ord_same_exponent() {
        let a = BigFloat::new(1.5, 2);
        let b = BigFloat::new(2.5, 2);
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_partial_ord_negative() {
        let a = BigFloat::new(-1.0, 2); // -100
        let b = BigFloat::new(1.0, 1);  // 10
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_partial_ord_zero() {
        let zero = BigFloat::from_f64(0.0);
        let positive = BigFloat::new(1.0, 0);
        let negative = BigFloat::new(-1.0, 0);
        
        assert!(zero < positive);
        assert!(zero > negative);
        assert_eq!(zero, BigFloat::from_f64(0.0));
    }

    #[test]
    fn test_abs() {
        let positive = BigFloat::new(1.5, 2);
        let negative = BigFloat::new(-1.5, 2);
        
        assert_eq!(positive.abs(), positive);
        assert_eq!(negative.abs(), positive);
    }

    #[test]
    fn test_signum() {
        let positive = BigFloat::new(1.5, 2);
        let negative = BigFloat::new(-1.5, 2);
        let zero = BigFloat::from_f64(0.0);
        
        assert_eq!(positive.signum(), BigFloat::from_f64(1.0));
        assert_eq!(negative.signum(), BigFloat::from_f64(-1.0));
        assert_eq!(zero.signum(), BigFloat::from_f64(0.0));
    }

    #[test]
    fn test_min_max() {
        let a = BigFloat::new(1.0, 2);
        let b = BigFloat::new(2.0, 1);
        
        assert_eq!(a.clone().min(b.clone()), b);
        assert_eq!(a.clone().max(b.clone()), a);
    }

    #[test]
    fn test_assignment_operators() {
        let mut a = BigFloat::new(1.0, 1);
        let b = BigFloat::new(2.0, 1);
        
        a += b.clone();
        assert_eq!(a, BigFloat::new(3.0, 1));
        
        a -= b.clone();
        assert_eq!(a, BigFloat::new(1.0, 1));
        
        a *= b.clone();
        assert_eq!(a, BigFloat::new(2.0, 1));
        
        a /= b;
        assert_eq!(a, BigFloat::new(1.0, 1));
    }

    #[test]
    fn test_neg() {
        let positive = BigFloat::new(1.5, 2);
        let negative = -positive.clone();
        
        assert_eq!(negative.mantissa(), -1.5);
        assert_eq!(negative.exponent(), positive.exponent());
    }

    #[test]
    fn test_default() {
        let default_bf = BigFloat::default();
        assert_eq!(default_bf, BigFloat::from_f64(0.0));
    }
}