//! Component Metadata Derive Macros
//!
//! Provides derive macros for automatically generating component metadata
//! and property schemas from struct definitions. This addresses the
//! improvement plan's requirement for automated component registration.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, Field, Type, Attribute, 
    Meta, Expr, Lit, LitStr, Path, PathSegment
};

/// Derive macro for generating ComponentMetadata
/// 
/// Usage:
/// ```
/// #[derive(ComponentMetadata)]
/// #[component(
///     name = "Button",
///     display_name = "Button",
///     description = "A clickable button component",
///     category = "Input",
///     version = "1.0.0"
/// )]
/// struct Button {
///     #[property(default = "Click me", description = "Button text")]
///     text: String,
///     
///     #[property(default = false, description = "Whether the button is disabled")]
///     disabled: bool,
///     
///     #[property(advanced = true, description = "Custom CSS classes")]
///     classes: Vec<String>,
/// }
/// ```
pub fn derive_component_metadata(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    // Parse component attributes
    let component_attrs = parse_component_attributes(&input.attrs);
    
    // Parse fields and generate property schema
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return quote! {
                    compile_error!("ComponentMetadata can only be derived for structs with named fields");
                };
            }
        },
        _ => {
            return quote! {
                compile_error!("ComponentMetadata can only be derived for structs");
            };
        }
    };
    
    let property_definitions = generate_property_definitions(fields);
    let default_values = generate_default_values(fields);
    
    let name = component_attrs.name.unwrap_or_else(|| struct_name.to_string());
    let display_name = component_attrs.display_name.unwrap_or_else(|| name.clone());
    let description = component_attrs.description.unwrap_or_else(|| format!("{} component", name));
    let category = component_attrs.category.unwrap_or_else(|| "Custom".to_string());
    let version = component_attrs.version.unwrap_or_else(|| "1.0.0".to_string());
    
    // Generate category tokens
    let category_tokens = parse_component_category_tokens(&category);
    
    // Generate the implementation
    let impl_name = format_ident!("{}ComponentMetadata", struct_name);
    
    quote! {
        impl #struct_name {
            /// Get component metadata
            pub fn component_metadata() -> crate::rcl::component_registry::ComponentMetadata {
                use std::collections::HashMap;
                use crate::rcl::component_registry::*;
                
                let mut properties = HashMap::new();
                #(#property_definitions)*
                
                let mut defaults = HashMap::new();
                #(#default_values)*
                
                ComponentMetadata {
                    component_type: #name.to_string(),
                    display_name: #display_name.to_string(),
                    description: #description.to_string(),
                    category: #category_tokens,
                    version: #version.to_string(),
                    schema: PropertySchema {
                        properties,
                        required: vec![], // TODO: Parse required properties
                        groups: vec![],   // TODO: Parse property groups
                    },
                    defaults,
                    events: vec![], // TODO: Parse events
                    icon: None,     // TODO: Parse icon
                    tags: vec![],   // TODO: Parse tags
                }
            }
            
            /// Register this component with the registry
            pub fn register(registry: &mut ComponentRegistry) {
                let metadata = Self::component_metadata();
                registry.register_component_metadata(metadata);
            }
        }
    }
}

/// Component attribute configuration
#[derive(Default)]
struct ComponentAttributes {
    name: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    category: Option<String>,
    version: Option<String>,
}

/// Parse component-level attributes
fn parse_component_attributes(attrs: &[Attribute]) -> ComponentAttributes {
    let mut component_attrs = ComponentAttributes::default();
    
    for attr in attrs {
        if attr.path().is_ident("component") {
            if let Ok(Meta::List(meta_list)) = attr.meta.require_list() {
                // Parse nested meta items
                // This is a simplified parser - in practice you'd want more robust parsing
                let tokens = meta_list.tokens.to_string();
                
                // Basic parsing for key = "value" pairs
                if let Some(name) = extract_string_attribute(&tokens, "name") {
                    component_attrs.name = Some(name);
                }
                if let Some(display_name) = extract_string_attribute(&tokens, "display_name") {
                    component_attrs.display_name = Some(display_name);
                }
                if let Some(description) = extract_string_attribute(&tokens, "description") {
                    component_attrs.description = Some(description);
                }
                if let Some(category) = extract_string_attribute(&tokens, "category") {
                    component_attrs.category = Some(category);
                }
                if let Some(version) = extract_string_attribute(&tokens, "version") {
                    component_attrs.version = Some(version);
                }
            }
        }
    }
    
    component_attrs
}

/// Generate property definitions from struct fields
fn generate_property_definitions(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>) -> Vec<TokenStream> {
    fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;
        
        // Parse property attributes
        let property_attrs = parse_property_attributes(&field.attrs);
        
        let description = property_attrs.description.unwrap_or_else(|| field_name_str.clone());
        let readonly = property_attrs.readonly;
        let advanced = property_attrs.advanced;
        
        // Map Rust types to PropertyType
        let property_type = map_rust_type_to_property_type(field_type);
        let default_value = property_attrs.default_value.unwrap_or_else(|| {
            quote! { PropertyValue::Null }
        });
        
        quote! {
            properties.insert(
                #field_name_str.to_string(),
                PropertyDefinition {
                    name: #field_name_str.to_string(),
                    property_type: #property_type,
                    display_name: #field_name_str.to_string(), // TODO: Support custom display names
                    description: #description.to_string(),
                    default_value: #default_value,
                    constraints: vec![], // TODO: Parse constraints
                    ui_hints: PropertyUIHints::default(),
                    readonly: #readonly,
                    advanced: #advanced,
                }
            );
        }
    }).collect()
}

/// Generate default values from struct fields
fn generate_default_values(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>) -> Vec<TokenStream> {
    fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        
        // Parse property attributes
        let property_attrs = parse_property_attributes(&field.attrs);
        
        if let Some(default_value) = property_attrs.default_value {
            Some(quote! {
                defaults.insert(#field_name_str.to_string(), #default_value);
            })
        } else {
            None
        }
    }).collect()
}

/// Property attribute configuration
#[derive(Default)]
struct PropertyAttributes {
    description: Option<String>,
    default_value: Option<TokenStream>,
    readonly: bool,
    advanced: bool,
}

/// Parse property-level attributes
fn parse_property_attributes(attrs: &[Attribute]) -> PropertyAttributes {
    let mut property_attrs = PropertyAttributes::default();
    
    for attr in attrs {
        if attr.path().is_ident("property") {
            if let Ok(Meta::List(meta_list)) = attr.meta.require_list() {
                let tokens = meta_list.tokens.to_string();
                
                if let Some(description) = extract_string_attribute(&tokens, "description") {
                    property_attrs.description = Some(description);
                }
                
                if let Some(default_str) = extract_string_attribute(&tokens, "default") {
                    // Convert default value to PropertyValue
                    property_attrs.default_value = Some(convert_default_value(&default_str));
                }
                
                if tokens.contains("readonly = true") {
                    property_attrs.readonly = true;
                }
                
                if tokens.contains("advanced = true") {
                    property_attrs.advanced = true;
                }
            }
        }
    }
    
    property_attrs
}

/// Extract string attribute value from token string
fn extract_string_attribute(tokens: &str, key: &str) -> Option<String> {
    // Simple regex-like parsing for key = "value" patterns
    let pattern = format!(r#"{}\s*=\s*"([^"]*)""#, key);
    
    // Basic string matching - in production would use proper parsing
    if let Some(start) = tokens.find(&format!("{} = \"", key)) {
        let value_start = start + key.len() + 4; // key + = " 
        if let Some(end) = tokens[value_start..].find('"') {
            return Some(tokens[value_start..value_start + end].to_string());
        }
    }
    
    None
}

/// Map Rust types to PropertyType tokens
fn map_rust_type_to_property_type(ty: &Type) -> TokenStream {
    match ty {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();
            
            match type_name.as_str() {
                "String" => quote! { PropertyType::String },
                "i32" | "i64" | "isize" => quote! { PropertyType::Integer },
                "f32" | "f64" => quote! { PropertyType::Float },
                "bool" => quote! { PropertyType::Boolean },
                "Vec" => {
                    // Handle Vec<T>
                    if let syn::PathArguments::AngleBracketed(args) = &type_path.path.segments.last().unwrap().arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                            let inner_property_type = map_rust_type_to_property_type(inner_type);
                            return quote! { PropertyType::Array(Box::new(#inner_property_type)) };
                        }
                    }
                    quote! { PropertyType::Array(Box::new(PropertyType::String)) }
                }
                "HashMap" => quote! { PropertyType::Object(PropertySchema { properties: HashMap::new(), required: vec![], groups: vec![] }) },
                _ => quote! { PropertyType::Custom(#type_name.to_string()) },
            }
        }
        _ => quote! { PropertyType::String }, // Default fallback
    }
}

/// Parse component category string to ComponentCategory tokens
fn parse_component_category_tokens(category: &str) -> TokenStream {
    match category {
        "Layout" => quote! { ComponentCategory::Layout },
        "Input" => quote! { ComponentCategory::Input },
        "Display" => quote! { ComponentCategory::Display },
        "Navigation" => quote! { ComponentCategory::Navigation },
        "Data" => quote! { ComponentCategory::Data },
        "Media" => quote! { ComponentCategory::Media },
        "Advanced" => quote! { ComponentCategory::Advanced },
        other => quote! { ComponentCategory::Custom(#other.to_string()) },
    }
}

/// Convert default value string to PropertyValue tokens
fn convert_default_value(default_str: &str) -> TokenStream {
    // Try to parse as different types
    if default_str == "true" || default_str == "false" {
        let bool_val: bool = default_str.parse().unwrap();
        quote! { PropertyValue::Boolean(#bool_val) }
    } else if let Ok(int_val) = default_str.parse::<i64>() {
        quote! { PropertyValue::Integer(#int_val) }
    } else if let Ok(float_val) = default_str.parse::<f64>() {
        quote! { PropertyValue::Float(#float_val) }
    } else {
        // Treat as string
        quote! { PropertyValue::String(#default_str.to_string()) }
    }
}

/// Actual proc macro wrapper (would be in lib.rs of a proc-macro crate)
/// For now, this is just the implementation logic
#[cfg(feature = "proc-macros")]
pub fn component_metadata_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let output = derive_component_metadata(input);
    proc_macro::TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_map_rust_type_to_property_type() {
        let string_type: Type = parse_quote!(String);
        let bool_type: Type = parse_quote!(bool);
        let vec_type: Type = parse_quote!(Vec<String>);
        
        // These would generate the expected TokenStream in real usage
        // For testing, we just verify the function doesn't panic
        let _ = map_rust_type_to_property_type(&string_type);
        let _ = map_rust_type_to_property_type(&bool_type);
        let _ = map_rust_type_to_property_type(&vec_type);
    }
    
    #[test]
    fn test_convert_default_value() {
        let _ = convert_default_value("true");
        let _ = convert_default_value("42");
        let _ = convert_default_value("3.14");
        let _ = convert_default_value("hello");
    }
}