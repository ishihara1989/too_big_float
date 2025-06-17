# TOO BIG FLOAT

A Rust library for handling arbitrarily large floating-point numbers that exceed the limits of standard f64.

## Overview

`too_big_float` provides a `BigFloat` type that can represent numbers far beyond the range of standard floating-point types. It uses a mantissa-exponent representation where:
- Mantissa: A normalized f64 value in the range [1.0, 10.0)
- Exponent: An i64 (or BigFloat for extremely large exponents)

This allows representation of numbers like 1e1e100 or larger.

## Features

### ✅ Implemented
- **Basic Arithmetic**: Addition, subtraction, multiplication, division
- **Comparison**: Full ordering with PartialOrd and Ord traits
- **Mathematical Functions**: 
  - `sqrt()` - Square root
  - `ln()`, `log10()` - Natural and base-10 logarithms  
  - `exp()` - Exponential function
  - `pow()`, `powi()` - Power functions
- **String Operations**:
  - `Display` trait for formatting
  - `FromStr` trait for parsing from strings
  - Scientific notation support
- **Conversions**:
  - `from_f64()` and `to_f64()` 
  - Compatible with standard numeric types
- **Standard Traits**:
  - `Clone`, `Debug`, `PartialEq`, `Eq`
  - `Default` (returns 0.0)
  - `Neg` for negation
  - Assignment operators (`+=`, `-=`, `*=`, `/=`)

## Installation

### Option 1: Clone and use as a local dependency

1. Clone the repository:
```bash
git clone https://github.com/ishihara1989/too_big_float.git
cd too_big_float
```

2. Build and test:
```bash
cargo build
cargo test
```

3. Use in your project by adding to your `Cargo.toml`:
```toml
[dependencies]
too_big_float = { path = "../path/to/too_big_float" }
```

### Option 2: Use directly from Git

Add this to your `Cargo.toml`:
```toml
[dependencies]
too_big_float = { git = "https://github.com/your-username/too_big_float.git" }
```

## Usage Examples

### Basic Operations

```rust
use too_big_float::BigFloat;

// Create BigFloat numbers
let a = BigFloat::new(1.5, 100);  // 1.5 × 10^100
let b = BigFloat::new(2.0, 50);   // 2.0 × 10^50
let c = BigFloat::from_f64(123.456);

// Arithmetic operations
let sum = a + b;
let product = a * b;  // 3.0 × 10^150
let quotient = a / b; // 7.5 × 10^49

// Comparisons
assert!(a > b);
assert!(c < a);
```

### Mathematical Functions

```rust
use too_big_float::BigFloat;

let x = BigFloat::new(2.0, 0);  // 2.0

// Logarithms and exponentials
let ln_x = x.ln();          // Natural logarithm
let log10_x = x.log10();    // Base-10 logarithm
let exp_x = x.exp();        // e^x

// Power functions  
let squared = x.powi(2);    // x^2
let power = x.pow(BigFloat::new(3.5, 0)); // x^3.5

// Square root
let sqrt_x = x.sqrt();
```

### String Conversion and Parsing

```rust
use too_big_float::BigFloat;
use std::str::FromStr;

// Convert to string
let big_num = BigFloat::new(1.234, 567);
println!("{}", big_num); // "1.234e567"

// Parse from string
let parsed = BigFloat::from_str("2.5e100").unwrap();
let scientific = BigFloat::from_str("1.23e4.56e78").unwrap(); // Nested exponential

// Handle special values
let inf = BigFloat::from_str("inf").unwrap();
let neg_inf = BigFloat::from_str("-inf").unwrap();
```

### Working with Standard Types

```rust
use too_big_float::BigFloat;

// Convert from f64
let from_float = BigFloat::from_f64(3.14159);

// Convert back to f64 (may overflow to infinity)
let back_to_float = from_float.to_f64();

// Assignment operations
let mut x = BigFloat::new(1.0, 10);
x += BigFloat::new(2.0, 10);  // x = 3.0 × 10^10
x *= BigFloat::new(2.0, 0);   // x = 6.0 × 10^10
```

### Handling Very Large Numbers

```rust
use too_big_float::BigFloat;

// Extremely large numbers
let googol = BigFloat::new(1.0, 100);        // 10^100
let googolplex = BigFloat::new(1.0, googol); // 10^(10^100) - TODO: Not yet implemented

// Numbers beyond f64 range
let huge = BigFloat::new(1.0, 1000);         // 10^1000
println!("{}", huge); // "1e1000"

// Precision is maintained in operations
let precise = BigFloat::new(1.23456789, 500);
let result = precise * BigFloat::new(2.0, 0);
// Result maintains precision: 2.46913578 × 10^500
```

## Implementation Details

### Number Representation
- **Mantissa**: Normalized f64 in range [1.0, 10.0) (except for zero and special values)
- **Exponent**: i64 for normal cases, BigFloat for extremely large exponents (future feature)
- **Base**: Always base 10

### Arithmetic Strategy
- **Addition/Subtraction**: Scale smaller number to match larger number's exponent
- **Multiplication/Division**: Combine mantissas and add/subtract exponents
- **Precision**: Limited by f64 precision (~15-17 decimal digits)

### Special Values
- Supports positive/negative infinity
- Supports NaN (Not a Number)
- Zero handling optimized

## Limitations

- Mantissa precision limited to f64 (approximately 15-17 decimal digits)
- Very large exponents stored as BigFloat not yet fully implemented
- Some mathematical functions may have reduced accuracy for extreme values

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.