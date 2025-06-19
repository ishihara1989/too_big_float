use crate::bigfloat::BigFloat;

impl BigFloat {
    pub fn ln(&self) -> BigFloat {
        if self.is_zero() {
            return BigFloat::from_f64(f64::NEG_INFINITY);
        }
        
        if !self.is_finite() {
            return BigFloat::from_f64(self.mantissa.ln());
        }

        if self.mantissa < 0.0 {
            return BigFloat::from_f64(f64::NAN);
        }

        let mantissa_ln = self.mantissa.ln();
        
        // ln(mantissa * 10^exp) = ln(mantissa) + exp * ln(10)
        let exp_term = (self.exponent as f64) * 10.0_f64.ln();
        BigFloat::from_f64(mantissa_ln + exp_term)
    }

    pub fn log10(&self) -> BigFloat {
        if self.is_zero() {
            return BigFloat::from_f64(f64::NEG_INFINITY);
        }
        
        if !self.is_finite() {
            return BigFloat::from_f64(self.mantissa.log10());
        }

        if self.mantissa < 0.0 {
            return BigFloat::from_f64(f64::NAN);
        }

        let mantissa_log10 = self.mantissa.log10();
        
        // log10(mantissa * 10^exp) = log10(mantissa) + exp
        BigFloat::from_f64(mantissa_log10 + (self.exponent as f64))
    }

    pub fn exp(&self) -> BigFloat {
        if !self.is_finite() {
            return BigFloat::from_f64(self.mantissa.exp());
        }

        let exp_f64 = self.exponent as f64;
        
        if exp_f64 < 0.0 {
            // Very small number, exp() will be close to 1
            return BigFloat::from_f64(1.0);
        }
        
        if exp_f64 > 2.0 {
            // Very large number, result will be infinity
            return BigFloat::from_f64(f64::INFINITY);
        }
        
        // For moderate exponents, convert to f64 and use standard exp
        let as_f64 = self.to_f64_lossy();
        if as_f64.is_finite() {
            BigFloat::from_f64(as_f64.exp())
        } else {
            BigFloat::from_f64(f64::INFINITY)
        }
    }

    pub fn pow(&self, exponent: &BigFloat) -> BigFloat {
        if self.is_zero() {
            if exponent.is_zero() {
                return BigFloat::from_f64(f64::NAN);  // 0^0 is undefined
            }
            return BigFloat::from_f64(0.0);
        }

        if exponent.is_zero() {
            return BigFloat::from_f64(1.0);
        }

        if !self.is_finite() || !exponent.is_finite() {
            let self_f64 = self.to_f64_lossy();
            let exp_f64 = exponent.to_f64_lossy();
            return BigFloat::from_f64(self_f64.powf(exp_f64));
        }

        // Use the identity: a^b = exp(b * ln(a))
        let ln_self = self.ln();
        let product = ln_self * exponent.clone();
        product.exp()
    }

    pub fn powi(&self, n: i32) -> BigFloat {
        if n == 0 {
            return BigFloat::from_f64(1.0);
        }

        if n == 1 {
            return self.clone();
        }

        if n < 0 {
            let positive_result = self.powi(-n);
            return BigFloat::from_f64(1.0) / positive_result;
        }

        // Use binary exponentiation for efficiency
        let mut result = BigFloat::from_f64(1.0);
        let mut base = self.clone();
        let mut exp = n as u32;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp >>= 1;
        }

        result
    }

    pub fn sqrt(&self) -> BigFloat {
        if self.mantissa < 0.0 {
            return BigFloat::from_f64(f64::NAN);
        }

        if self.is_zero() || !self.is_finite() {
            return BigFloat::from_f64(self.mantissa.sqrt());
        }

        let half = BigFloat::from_f64(0.5);
        self.pow(&half)
    }

    fn to_f64_lossy(&self) -> f64 {
        if self.exponent > 308 {
            if self.mantissa >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY }
        } else {
            self.mantissa * 10.0_f64.powi(self.exponent as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ln_basic() {
        let bf = BigFloat::from_f64(2.718281828459045); // e
        let result = bf.ln();
        assert!((result.to_f64_lossy() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_log10_basic() {
        let bf = BigFloat::new(1.0, 3); // 1000
        let result = bf.log10();
        assert!((result.to_f64_lossy() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp_basic() {
        let bf = BigFloat::from_f64(1.0);
        let result = bf.exp();
        assert!((result.to_f64_lossy() - 2.718281828459045).abs() < 1e-10);
    }

    #[test]
    fn test_pow_basic() {
        let base = BigFloat::from_f64(2.0);
        let exp = BigFloat::from_f64(3.0);
        let result = base.pow(&exp);
        assert!((result.to_f64_lossy() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_powi_basic() {
        let base = BigFloat::from_f64(2.0);
        let result = base.powi(10);
        assert!((result.to_f64_lossy() - 1024.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_basic() {
        let bf = BigFloat::from_f64(9.0);
        let result = bf.sqrt();
        assert!((result.to_f64_lossy() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_large_number_log() {
        let bf = BigFloat::new(1.0, 100); // 1e100
        let result = bf.log10();
        assert!((result.to_f64_lossy() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_cases() {
        let zero = BigFloat::from_f64(0.0);
        assert_eq!(zero.ln().to_f64_lossy(), f64::NEG_INFINITY);
        assert_eq!(zero.log10().to_f64_lossy(), f64::NEG_INFINITY);
        assert_eq!(zero.exp().to_f64_lossy(), 1.0);
    }
}