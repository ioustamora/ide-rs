//! Demo Rust code for Enhanced LSP Integration
//! 
//! This file contains various Rust constructs to demonstrate
//! the enhanced LSP features like go-to-definition, find references,
//! code actions, diagnostics, and autocomplete.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

/// A sample struct to demonstrate LSP features
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: Option<String>,
}

impl Person {
    /// Create a new Person instance
    /// 
    /// This function demonstrates signature help and parameter hints.
    /// Try typing `Person::new(` to see the signature help popup.
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            email: None,
        }
    }
    
    /// Set the email address
    /// 
    /// Try right-clicking on this method name to see code actions
    /// and find references functionality.
    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
    
    /// Get a formatted string representation
    /// 
    /// This demonstrates error handling and Result types.
    /// Try introducing errors to see real-time diagnostics.
    pub fn format_info(&self) -> Result<String, std::fmt::Error> {
        let email_part = match &self.email {
            Some(email) => format!(" ({})", email),
            None => String::new(),
        };
        
        Ok(format!("{} (age: {}){}", self.name, self.age, email_part))
    }
}

/// Trait to demonstrate trait implementations and LSP navigation
pub trait Displayable {
    fn display(&self) -> String;
}

impl Displayable for Person {
    fn display(&self) -> String {
        format!("Person: {}", self.name)
    }
}

/// Generic function to demonstrate type inference and autocomplete
pub fn process_items<T>(items: Vec<T>) -> Vec<T>
where
    T: Clone + std::fmt::Debug,
{
    println!("Processing {} items", items.len());
    
    // Try typing `items.` to see method autocomplete
    items.into_iter()
        .inspect(|item| println!("Item: {:?}", item))
        .collect()
}

/// Function with complex parameters for signature help demonstration
pub fn complex_function(
    name: &str,
    options: HashMap<String, String>,
    callback: impl Fn(&str) -> Result<String, io::Error>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Processing: {}", name);
    
    // Try calling this function to see signature help
    for (key, value) in options {
        match callback(&value) {
            Ok(result) => println!("{}: {} -> {}", key, value, result),
            Err(e) => eprintln!("Error processing {}: {}", key, e),
        }
    }
    
    Ok("Processing completed".to_string())
}

/// Main function with various constructs for testing LSP features
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create some sample data
    let mut person = Person::new("Alice".to_string(), 30);
    person.set_email("alice@example.com".to_string());
    
    // Demonstrate error handling and diagnostics
    match person.format_info() {
        Ok(info) => println!("Person info: {}", info),
        Err(e) => eprintln!("Failed to format person info: {}", e),
    }
    
    // Demonstrate trait usage
    println!("Display: {}", person.display());
    
    // Demonstrate generic function usage
    let numbers = vec![1, 2, 3, 4, 5];
    let processed = process_items(numbers);
    println!("Processed numbers: {:?}", processed);
    
    // Demonstrate complex function call with signature help
    let mut options = HashMap::new();
    options.insert("key1".to_string(), "value1".to_string());
    options.insert("key2".to_string(), "value2".to_string());
    
    let result = complex_function(
        "test_operation",
        options,
        |value| Ok(value.to_uppercase()),
    )?;
    
    println!("Result: {}", result);
    
    // Some intentional errors for diagnostics demonstration
    // Uncomment the lines below to see error squiggles:
    
    // let invalid_syntax = ; // Missing expression
    // let type_mismatch: u32 = "string"; // Type mismatch
    // let unused_variable = 42; // Unused variable warning
    // println!("{}", undefined_variable); // Undefined variable
    
    // Try auto-completion here - type `std::` to see suggestions
    let _file = std::fs::File::open("nonexistent.txt");
    
    // More complex expressions for go-to-definition testing
    let people = vec![
        Person::new("Bob".to_string(), 25),
        Person::new("Carol".to_string(), 35),
    ];
    
    for person in people {
        // Try right-clicking on method names for context menu
        match person.format_info() {
            Ok(info) => println!("{}", info),
            Err(_) => println!("Error formatting person"),
        }
    }
    
    Ok(())
}

/// Additional function to test find references
pub fn helper_function() {
    println!("Helper function called");
    
    // Call main function to create cross-references
    if let Err(e) = main() {
        eprintln!("Main function error: {}", e);
    }
}

/// Macro for testing macro expansion and LSP features
macro_rules! debug_print {
    ($($arg:tt)*) => {
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

/// Function using the macro
pub fn test_macro() {
    debug_print!("This is a debug message: {}", 42);
}

// Module for testing module navigation
pub mod sub_module {
    use super::Person;
    
    /// Function in submodule
    pub fn process_person(person: &Person) -> String {
        format!("Processing in submodule: {}", person.name)
    }
    
    /// Nested module for deeper navigation testing
    pub mod nested {
        /// Deeply nested function
        pub fn deep_function() {
            println!("Deep in the module hierarchy");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_creation() {
        let person = Person::new("Test".to_string(), 25);
        assert_eq!(person.name, "Test");
        assert_eq!(person.age, 25);
        assert!(person.email.is_none());
    }
    
    #[test]
    fn test_email_setting() {
        let mut person = Person::new("Test".to_string(), 25);
        person.set_email("test@example.com".to_string());
        assert_eq!(person.email, Some("test@example.com".to_string()));
    }
}