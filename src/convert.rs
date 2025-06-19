use crate::bigfloat::BigFloat;
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

        if self.exponent == 0 {
            // For exponent 0, just show the mantissa
            write!(f, "{}", self.mantissa)
        } else if self.exponent <= 6 {
            // Use standard notation for small exponents
            let value = self.mantissa * 10.0_f64.powi(self.exponent as i32);
            write!(f, "{}", value)
        } else {
            // Use scientific notation for large exponents
            write!(f, "{}e{}", self.mantissa, self.exponent)
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
            
            // Try to parse exponent as u128
            if let Ok(exp) = exp_str.parse::<u128>() {
                return Ok(BigFloat::new(mantissa, exp));
            }
            
            // If that fails, try as i64 and convert using the helper method
            if let Ok(exp) = exp_str.parse::<i64>() {
                return Ok(BigFloat::new_from_i64_exponent(mantissa, exp));
            }
            
            
            return Err(format!("Invalid exponent: {}", exp_str));
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
        if self.exponent > 308 {
            None // Out of f64 range
        } else {
            Some(self.mantissa * 10.0_f64.powi(self.exponent as i32))
        }
    }

    pub fn to_f64_saturating(&self) -> f64 {
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
    fn test_parse_basic() {
        let bf: BigFloat = "123.45".parse().unwrap();
        assert!((bf.mantissa() - 1.2345).abs() < 1e-10);
        assert_eq!(bf.exponent(), 2);
    }

    #[test]
    fn test_parse_scientific() {
        let bf: BigFloat = "1.23e10".parse().unwrap();
        assert!((bf.mantissa() - 1.23).abs() < 1e-10);
        assert_eq!(bf.exponent(), 10);
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

    #[test]
    fn test_small_fraction_display() {
        let bf = BigFloat::new(0.123, 0);
        assert_eq!(format!("{}", bf), "0.123");
    }

    #[test]
    fn test_large_exponent_display() {
        let bf = BigFloat::new(1.23, 15);
        assert_eq!(format!("{}", bf), "1.23e15");
    }
}