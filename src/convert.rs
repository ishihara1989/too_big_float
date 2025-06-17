use crate::bigfloat::{BigFloat, Exponent};
use std::fmt;
use std::str::FromStr;

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mantissa.is_nan() {
            return write!(f, "NaN");
        }
        
        if self.mantissa.is_infinite() {
            return write!(f, "{}", if self.mantissa.is_sign_positive() { "∞" } else { "-∞" });
        }
        
        if self.is_zero() {
            return write!(f, "0");
        }

        match &self.exponent {
            Exponent::Long(exp) => {
                if *exp >= -4 && *exp <= 6 {
                    // Use standard notation for reasonable-sized numbers
                    let value = self.mantissa * 10.0_f64.powi(*exp as i32);
                    write!(f, "{}", value)
                } else {
                    // Use scientific notation
                    write!(f, "{}e{}", self.mantissa, exp)
                }
            }
            Exponent::BigFloat(big_exp) => {
                // For very large exponents, show as mantissa * 10^(big_exponent)
                write!(f, "{} × 10^({})", self.mantissa, big_exp)
            }
        }
    }
}

impl FromStr for BigFloat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        
        if s.is_empty() {
            return Err("Empty string".to_string());
        }

        // Handle special cases
        match s.to_lowercase().as_str() {
            "nan" => return Ok(BigFloat::from_f64(f64::NAN)),
            "inf" | "infinity" | "∞" => return Ok(BigFloat::from_f64(f64::INFINITY)),
            "-inf" | "-infinity" | "-∞" => return Ok(BigFloat::from_f64(f64::NEG_INFINITY)),
            "0" | "0.0" => return Ok(BigFloat::from_f64(0.0)),
            _ => {}
        }

        // Try to parse as standard f64 first
        if let Ok(val) = s.parse::<f64>() {
            return Ok(BigFloat::from_f64(val));
        }

        // Handle scientific notation with potentially large exponents
        if let Some(e_pos) = s.to_lowercase().find('e') {
            let (mantissa_str, exp_str) = s.split_at(e_pos);
            let exp_str = &exp_str[1..]; // Remove 'e'
            
            let mantissa: f64 = mantissa_str.parse()
                .map_err(|_| format!("Invalid mantissa: {}", mantissa_str))?;
            
            // Try to parse exponent as i64 first
            if let Ok(exp) = exp_str.parse::<i64>() {
                return Ok(BigFloat::new(mantissa, exp));
            }
            
            // If that fails, try to parse as BigFloat
            if let Ok(exp_bigfloat) = exp_str.parse::<BigFloat>() {
                return Ok(BigFloat {
                    mantissa,
                    exponent: Exponent::BigFloat(Box::new(exp_bigfloat)),
                });
            }
            
            return Err(format!("Invalid exponent: {}", exp_str));
        }

        // Handle notation like "1e1e100" (nested exponentials)
        if s.matches('e').count() >= 2 {
            // Find the first 'e'
            if let Some(first_e) = s.find('e') {
                let (mantissa_str, rest) = s.split_at(first_e);
                let exp_str = &rest[1..]; // Remove first 'e'
                
                let mantissa: f64 = mantissa_str.parse()
                    .map_err(|_| format!("Invalid mantissa: {}", mantissa_str))?;
                
                // Parse the rest as a BigFloat (recursive)
                let exp_bigfloat = exp_str.parse::<BigFloat>()?;
                
                return Ok(BigFloat {
                    mantissa,
                    exponent: Exponent::BigFloat(Box::new(exp_bigfloat)),
                });
            }
        }

        Err(format!("Unable to parse: {}", s))
    }
}

impl From<f64> for BigFloat {
    fn from(value: f64) -> Self {
        BigFloat::from_f64(value)
    }
}

impl From<f32> for BigFloat {
    fn from(value: f32) -> Self {
        BigFloat::from_f64(value as f64)
    }
}

impl From<i32> for BigFloat {
    fn from(value: i32) -> Self {
        BigFloat::from_f64(value as f64)
    }
}

impl From<i64> for BigFloat {
    fn from(value: i64) -> Self {
        BigFloat::from_f64(value as f64)
    }
}

impl BigFloat {
    pub fn to_f64(&self) -> Option<f64> {
        match &self.exponent {
            Exponent::Long(exp) => {
                if *exp > 308 || *exp < -324 {
                    None // Out of f64 range
                } else {
                    Some(self.mantissa * 10.0_f64.powi(*exp as i32))
                }
            }
            Exponent::BigFloat(_) => None, // Cannot represent in f64
        }
    }

    pub fn to_f64_saturating(&self) -> f64 {
        match &self.exponent {
            Exponent::Long(exp) => {
                if *exp > 308 {
                    if self.mantissa >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY }
                } else if *exp < -324 {
                    0.0
                } else {
                    self.mantissa * 10.0_f64.powi(*exp as i32)
                }
            }
            Exponent::BigFloat(_) => {
                if self.mantissa >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_basic() {
        let bf = BigFloat::new(1.23, 2);
        assert_eq!(format!("{}", bf), "123");
    }

    #[test]
    fn test_display_scientific() {
        let bf = BigFloat::new(1.23, 10);
        assert_eq!(format!("{}", bf), "1.23e10");
    }

    #[test]
    fn test_display_very_large() {
        let large_exp = BigFloat::new(1.0, 100);
        let bf = BigFloat {
            mantissa: 1.23,
            exponent: Exponent::BigFloat(Box::new(large_exp)),
        };
        assert!(format!("{}", bf).contains("1.23 × 10^"));
    }

    #[test]
    fn test_parse_basic() {
        let bf: BigFloat = "123.45".parse().unwrap();
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(2));
    }

    #[test]
    fn test_parse_scientific() {
        let bf: BigFloat = "1.23e10".parse().unwrap();
        assert!((bf.mantissa() - 1.23).abs() < 1e-10);
        assert_eq!(bf.exponent(), &Exponent::Long(10));
    }

    #[test]
    fn test_parse_nested_exponential() {
        let bf: BigFloat = "1e1e10".parse().unwrap();
        assert!((bf.mantissa() - 1.0).abs() < 1e-10);
        match bf.exponent() {
            Exponent::BigFloat(exp) => {
                assert!((exp.mantissa() - 1.0).abs() < 1e-10);
                assert_eq!(exp.exponent(), &Exponent::Long(10));
            }
            _ => panic!("Expected BigFloat exponent"),
        }
    }

    #[test]
    fn test_parse_special_values() {
        assert!("NaN".parse::<BigFloat>().unwrap().mantissa().is_nan());
        assert_eq!("inf".parse::<BigFloat>().unwrap().mantissa(), f64::INFINITY);
        assert_eq!("-inf".parse::<BigFloat>().unwrap().mantissa(), f64::NEG_INFINITY);
        assert_eq!("0".parse::<BigFloat>().unwrap().mantissa(), 0.0);
    }

    #[test]
    fn test_conversions() {
        let bf: BigFloat = 123.45_f64.into();
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        
        let bf: BigFloat = 42_i32.into();
        assert!((bf.mantissa() - 4.2).abs() < 1e-10);
    }

    #[test]
    fn test_to_f64() {
        let bf = BigFloat::new(1.23, 2);
        assert_eq!(bf.to_f64().unwrap(), 123.0);
        
        let large = BigFloat::new(1.0, 1000);
        assert!(large.to_f64().is_none());
        assert_eq!(large.to_f64_saturating(), f64::INFINITY);
    }
}