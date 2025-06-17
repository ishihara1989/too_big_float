use too_big_float::BigFloat;

fn main() {
    println!("=== TOO BIG FLOAT Demo ===");
    
    // Basic operations
    println!("\n1. Basic Operations:");
    let a = BigFloat::new(1.5, 100);  // 1.5 * 10^100
    let b = BigFloat::new(2.5, 100);  // 2.5 * 10^100
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a.clone() + b.clone());
    println!("a * b = {}", a.clone() * b.clone());
    
    // Very large numbers
    println!("\n2. Very Large Numbers:");
    let huge = BigFloat::new(1.0, 1000);  // 1e1000
    println!("1e1000 = {}", huge);
    println!("1e1000 * 2 = {}", huge.clone() * BigFloat::from(2.0));
    
    // Nested exponentials
    println!("\n3. Nested Exponentials:");
    let nested: BigFloat = "1e1e100".parse().unwrap();
    println!("1e1e100 = {}", nested);
    
    // Mathematical functions
    println!("\n4. Mathematical Functions:");
    let small = BigFloat::from(2.0);
    println!("ln(2) = {}", small.ln());
    println!("log10(1000) = {}", BigFloat::new(1.0, 3).log10());
    println!("2^10 = {}", small.powi(10));
    
    // Comparisons
    println!("\n5. Comparisons:");
    let x = BigFloat::new(1.0, 50);
    let y = BigFloat::new(1.0, 100);
    println!("1e50 < 1e100: {}", x < y);
    println!("max(1e50, 1e100) = {}", x.max(y));
    
    // String parsing
    println!("\n6. String Parsing:");
    let parsed: BigFloat = "1.23e456".parse().unwrap();
    println!("Parsed '1.23e456': {}", parsed);
    
    // Precision demonstration
    println!("\n7. Precision with Large Numbers:");
    let base = BigFloat::new(1.0, 100);
    let small_addition = BigFloat::new(1.0, 90);
    println!("1e100 + 1e90 = {}", base + small_addition);
}