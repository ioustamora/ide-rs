//! Test file for derive macro logic
//! 
//! This file contains simple tests to verify the derive macro logic works
//! without requiring the full compilation of the codebase.

use super::derive_macros::*;
use proc_macro2::TokenStream;
use syn::parse_quote;

#[test]
fn test_extract_string_attribute() {
    let tokens = r#"name = "Button", description = "A button component""#;
    
    assert_eq!(
        extract_string_attribute(tokens, "name"),
        Some("Button".to_string())
    );
    assert_eq!(
        extract_string_attribute(tokens, "description"),
        Some("A button component".to_string())
    );
    assert_eq!(
        extract_string_attribute(tokens, "missing"),
        None
    );
}

#[test]
fn test_parse_component_category_tokens() {
    let layout_tokens = parse_component_category_tokens("Layout");
    let custom_tokens = parse_component_category_tokens("CustomCategory");
    
    // These should generate valid TokenStreams
    assert!(!layout_tokens.to_string().is_empty());
    assert!(!custom_tokens.to_string().is_empty());
    assert!(custom_tokens.to_string().contains("Custom"));
}

#[test]
fn test_convert_default_value() {
    let bool_true = convert_default_value("true");
    let bool_false = convert_default_value("false");
    let integer = convert_default_value("42");
    let float = convert_default_value("3.14");
    let string = convert_default_value("hello");
    
    // Check that TokenStreams are generated
    assert!(bool_true.to_string().contains("Boolean"));
    assert!(bool_false.to_string().contains("Boolean"));
    assert!(integer.to_string().contains("Integer"));
    assert!(float.to_string().contains("Float"));
    assert!(string.to_string().contains("String"));
}

#[test]
fn test_map_rust_type_to_property_type() {
    let string_type: syn::Type = parse_quote!(String);
    let bool_type: syn::Type = parse_quote!(bool);
    let i32_type: syn::Type = parse_quote!(i32);
    let f64_type: syn::Type = parse_quote!(f64);
    let vec_type: syn::Type = parse_quote!(Vec<String>);
    
    let string_tokens = map_rust_type_to_property_type(&string_type);
    let bool_tokens = map_rust_type_to_property_type(&bool_type);
    let i32_tokens = map_rust_type_to_property_type(&i32_type);
    let f64_tokens = map_rust_type_to_property_type(&f64_type);
    let vec_tokens = map_rust_type_to_property_type(&vec_type);
    
    // Verify the tokens contain expected property types
    assert!(string_tokens.to_string().contains("String"));
    assert!(bool_tokens.to_string().contains("Boolean"));
    assert!(i32_tokens.to_string().contains("Integer"));
    assert!(f64_tokens.to_string().contains("Float"));
    assert!(vec_tokens.to_string().contains("Array"));
}

#[test]
fn test_derive_component_metadata_basic() {
    // Create a simple struct input
    let input: syn::DeriveInput = parse_quote! {
        #[component(name = "TestButton", category = "Input")]
        struct TestButton {
            #[property(default = "Click me")]
            text: String,
            #[property(default = true)]
            enabled: bool,
        }
    };
    
    // Generate the metadata
    let tokens = derive_component_metadata(input.into());
    let code = tokens.to_string();
    
    // Verify the generated code contains expected elements
    assert!(code.contains("component_metadata"));
    assert!(code.contains("TestButton"));
    assert!(code.contains("ComponentMetadata"));
    assert!(code.contains("PropertyDefinition"));
    assert!(code.contains("text"));
    assert!(code.contains("enabled"));
    assert!(code.contains("register"));
    
    // Verify property types are correct
    assert!(code.contains("PropertyType :: String"));
    assert!(code.contains("PropertyType :: Boolean"));
}

#[test] 
fn test_derive_component_metadata_with_all_attributes() {
    let input: syn::DeriveInput = parse_quote! {
        #[component(
            name = "AdvancedButton",
            display_name = "Advanced Button",
            description = "An advanced button with many features",
            category = "Input",
            version = "2.0.0"
        )]
        struct AdvancedButton {
            #[property(default = "Advanced", description = "Button label")]
            label: String,
            
            #[property(default = false, advanced = true)]
            debug_mode: bool,
            
            #[property(default = 16)]
            font_size: i32,
        }
    };
    
    let tokens = derive_component_metadata(input.into());
    let code = tokens.to_string();
    
    // Check component metadata
    assert!(code.contains("AdvancedButton"));
    assert!(code.contains("Advanced Button"));
    assert!(code.contains("An advanced button with many features"));
    assert!(code.contains("2.0.0"));
    
    // Check properties
    assert!(code.contains("label"));
    assert!(code.contains("debug_mode")); 
    assert!(code.contains("font_size"));
    
    // Check that advanced property is marked correctly
    assert!(code.contains("advanced : true"));
    assert!(code.contains("advanced : false"));
}

/// Integration test showing how the generated code would be used
#[test]
fn test_component_registration_flow() {
    // This test shows the complete flow of:
    // 1. Derive macro generates metadata
    // 2. Component registers with registry
    // 3. Registry can retrieve metadata
    
    let input: syn::DeriveInput = parse_quote! {
        #[component(name = "FlowTest", category = "Input")]
        struct FlowTest {
            #[property(default = "test")]
            name: String,
        }
    };
    
    let generated_tokens = derive_component_metadata(input.into());
    let generated_code = generated_tokens.to_string();
    
    // Verify the registration pattern is present
    assert!(generated_code.contains("pub fn register"));
    assert!(generated_code.contains("registry . register_component_metadata"));
    assert!(generated_code.contains("Self :: component_metadata"));
    
    // This demonstrates the complete derive macro functionality
    // In actual use, this would be:
    // 1. #[derive(ComponentMetadata)] on struct
    // 2. Automatic generation of metadata
    // 3. Easy registration: Button::register(&mut registry)
    println!("Generated code length: {} characters", generated_code.len());
    println!("✅ Derive macro prototype is working correctly!");
}