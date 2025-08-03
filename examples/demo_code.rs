//! Demo Rust Code for Modern Editor
//! This code demonstrates syntax highlighting, diagnostics, and autocomplete

use std::collections::HashMap;

fn main() {
    let name = "World"; // This will cause a type error for demo
    let number: i32 = name; // Error: mismatched types
    
    println!("Hello, {}!", name);
    let unused_var = 42; // Warning: unused variable
    
    // Function definition with various Rust features
    let result = calculate_fibonacci(10);
    println!("Fibonacci result: {}", result);
    
    // Pattern matching example
    match result {
        0 => println!("Zero"),
        1..=10 => println!("Small number"),
        _ => println!("Large number"),
    }
    
    // Complex data structures
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    // Iterator and closure example
    let numbers: Vec<i32> = (1..=5).collect();
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|&x| x > 4)
        .collect();
    
    println!("Doubled and filtered: {:?}", doubled);
}

fn calculate_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2),
    }
}

// Struct and impl example
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }
    
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I'm {} years old", self.name, self.age)
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

// Trait example
trait Displayable {
    fn display(&self) -> String;
}

impl Displayable for Person {
    fn display(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

// Error handling example
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Async function example (commented to avoid compilation issues)
/*
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // Simulate async operation
    Ok("Fetched data".to_string())
}
*/
