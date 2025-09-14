//! # Code Generation Intermediate Representation (IR)
//!
//! This module provides a structured intermediate representation for code generation
//! that enables sophisticated transformations, optimizations, and multi-target output.
//! The IR acts as a bridge between high-level component descriptions and target code.

use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Core IR node representing any element in the code structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrNode {
    /// Unique identifier for this node
    pub id: Uuid,
    /// Type of IR node
    pub node_type: IrNodeType,
    /// Child nodes in the IR tree
    pub children: Vec<IrNode>,
    /// Node-specific attributes
    pub attributes: BTreeMap<String, IrValue>,
    /// Source location information for debugging
    pub source_info: Option<SourceInfo>,
    /// Metadata for code generation passes
    pub metadata: HashMap<String, String>,
}

/// Types of IR nodes in the code generation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrNodeType {
    /// Root module or namespace
    Module {
        name: String,
        visibility: Visibility,
        imports: Vec<ImportDeclaration>,
    },
    
    /// Struct, class, or component definition
    Struct {
        name: String,
        visibility: Visibility,
        generic_params: Vec<GenericParameter>,
        fields: Vec<FieldDeclaration>,
        derives: Vec<String>,
    },
    
    /// Function, method, or event handler
    Function {
        name: String,
        visibility: Visibility,
        parameters: Vec<Parameter>,
        return_type: Option<TypeReference>,
        is_async: bool,
        attributes: Vec<AttributeDeclaration>,
    },
    
    /// Variable declaration or property
    Variable {
        name: String,
        var_type: TypeReference,
        is_mutable: bool,
        initial_value: Option<Box<IrNode>>,
    },
    
    /// Expression (function calls, operations, literals)
    Expression {
        expr_type: ExpressionType,
        data_type: Option<TypeReference>,
    },
    
    /// Statement (assignments, control flow)
    Statement {
        stmt_type: StatementType,
    },
    
    /// Code block with scope
    Block {
        statements: Vec<IrNode>,
        scope_type: ScopeType,
    },
    
    /// Guarded section for user-editable code
    GuardedSection {
        guard_id: String,
        guard_type: GuardType,
        default_content: Option<String>,
        user_content: Option<String>,
        is_modified: bool,
    },
    
    /// Conditional code generation
    Conditional {
        condition: String,
        then_branch: Box<IrNode>,
        else_branch: Option<Box<IrNode>>,
    },
    
    /// Template expansion point
    Template {
        template_name: String,
        parameters: HashMap<String, IrValue>,
        iteration_context: Option<IterationContext>,
    },
    
    /// Comment or documentation
    Comment {
        content: String,
        comment_type: CommentType,
    },
    
    /// Raw code literal (for escape hatches)
    RawCode {
        content: String,
        language: String,
    },
}

/// Values that can be stored in IR attributes or passed to templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<IrValue>),
    Object(BTreeMap<String, IrValue>),
    Null,
}

/// Source code location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub span_length: usize,
}

/// Visibility levels for code elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
    PublicCrate,
}

/// Import/use declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub path: String,
    pub alias: Option<String>,
    pub import_type: ImportType,
    pub items: Vec<String>, // For selective imports
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportType {
    Module,        // use std::collections
    Selective,     // use std::collections::{HashMap, HashSet}
    Glob,          // use std::collections::*
    Alias,         // use std::collections as coll
}

/// Generic parameter for structs/functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericParameter {
    pub name: String,
    pub bounds: Vec<String>,
    pub default: Option<String>,
}

/// Field in a struct/class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDeclaration {
    pub name: String,
    pub field_type: TypeReference,
    pub visibility: Visibility,
    pub attributes: Vec<AttributeDeclaration>,
    pub default_value: Option<IrValue>,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: TypeReference,
    pub default_value: Option<IrValue>,
    pub is_self: bool,
    pub is_mutable: bool,
}

/// Type reference in the IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeReference {
    pub name: String,
    pub generic_args: Vec<TypeReference>,
    pub is_optional: bool,
    pub is_reference: bool,
    pub is_mutable_ref: bool,
}

/// Attribute/annotation declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDeclaration {
    pub name: String,
    pub arguments: Vec<IrValue>,
}

/// Expression types in the IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionType {
    Literal(IrValue),
    Variable(String),
    FunctionCall {
        function: String,
        arguments: Vec<IrNode>,
    },
    MethodCall {
        receiver: Box<IrNode>,
        method: String,
        arguments: Vec<IrNode>,
    },
    BinaryOp {
        operator: BinaryOperator,
        left: Box<IrNode>,
        right: Box<IrNode>,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<IrNode>,
    },
    FieldAccess {
        object: Box<IrNode>,
        field: String,
    },
    IndexAccess {
        object: Box<IrNode>,
        index: Box<IrNode>,
    },
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<IrNode>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, BitAnd, BitOr, BitXor,
    LeftShift, RightShift,
    Assign, AddAssign, SubAssign,
}

/// Unary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
    Neg, Not, BitNot,
    Deref, Ref, RefMut,
}

/// Statement types in the IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementType {
    Expression(Box<IrNode>),
    Return(Option<Box<IrNode>>),
    Break(Option<String>),
    Continue(Option<String>),
    If {
        condition: Box<IrNode>,
        then_branch: Box<IrNode>,
        else_branch: Option<Box<IrNode>>,
    },
    While {
        condition: Box<IrNode>,
        body: Box<IrNode>,
    },
    For {
        variable: String,
        iterable: Box<IrNode>,
        body: Box<IrNode>,
    },
    Match {
        expression: Box<IrNode>,
        arms: Vec<MatchArm>,
    },
    Let {
        pattern: String,
        value: Box<IrNode>,
        is_mutable: bool,
    },
}

/// Match arm for pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: String,
    pub guard: Option<Box<IrNode>>,
    pub body: Box<IrNode>,
}

/// Scope types for code blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeType {
    Function,
    Block,
    Loop,
    Conditional,
    Guarded,
    Module,
}

/// Guard types for user-editable sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuardType {
    /// User can add any code
    UserCode,
    /// User can add imports
    Imports,
    /// User can add struct fields
    Fields,
    /// User can add methods
    Methods,
    /// User can add event handlers
    EventHandlers,
    /// User can add validation logic
    Validation,
    /// User can add initialization code
    Initialization,
    /// User can add custom properties
    Properties,
}

/// Template iteration context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationContext {
    pub data_source: String,
    pub item_variable: String,
    pub index_variable: Option<String>,
    pub separator: Option<String>,
    pub condition: Option<String>,
}

/// Comment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentType {
    Line,
    Block,
    Documentation,
    Generated,
}

impl IrNode {
    /// Create a new IR node with unique ID
    pub fn new(node_type: IrNodeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            node_type,
            children: Vec::new(),
            attributes: BTreeMap::new(),
            source_info: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Add a child node
    pub fn add_child(&mut self, child: IrNode) {
        self.children.push(child);
    }
    
    /// Add multiple child nodes
    pub fn add_children(&mut self, mut children: Vec<IrNode>) {
        self.children.append(&mut children);
    }
    
    /// Set an attribute
    pub fn set_attribute(&mut self, key: String, value: IrValue) {
        self.attributes.insert(key, value);
    }
    
    /// Get an attribute
    pub fn get_attribute(&self, key: &str) -> Option<&IrValue> {
        self.attributes.get(key)
    }
    
    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Find nodes by type (depth-first search)
    pub fn find_nodes_by_type(&self, target_type: fn(&IrNodeType) -> bool) -> Vec<&IrNode> {
        let mut result = Vec::new();
        
        if target_type(&self.node_type) {
            result.push(self);
        }
        
        for child in &self.children {
            result.extend(child.find_nodes_by_type(target_type));
        }
        
        result
    }
    
    /// Find a node by ID
    pub fn find_node_by_id(&self, target_id: Uuid) -> Option<&IrNode> {
        if self.id == target_id {
            return Some(self);
        }
        
        for child in &self.children {
            if let Some(node) = child.find_node_by_id(target_id) {
                return Some(node);
            }
        }
        
        None
    }
    
    /// Count nodes by type
    pub fn count_nodes_by_type(&self, target_type: fn(&IrNodeType) -> bool) -> usize {
        let mut count = if target_type(&self.node_type) { 1 } else { 0 };
        
        for child in &self.children {
            count += child.count_nodes_by_type(target_type);
        }
        
        count
    }
    
    /// Validate the IR tree structure
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        self.validate_recursive(&mut errors, "");
        errors
    }
    
    fn validate_recursive(&self, errors: &mut Vec<ValidationError>, path: &str) {
        let current_path = if path.is_empty() {
            self.id.to_string()
        } else {
            format!("{}.{}", path, self.id)
        };
        
        // Validate node-specific constraints
        match &self.node_type {
            IrNodeType::Function { parameters, .. } => {
                // Check for duplicate parameter names
                let mut param_names = std::collections::HashSet::new();
                for param in parameters {
                    if !param_names.insert(&param.name) {
                        errors.push(ValidationError::DuplicateParameter {
                            function_path: current_path.clone(),
                            parameter: param.name.clone(),
                        });
                    }
                }
            }
            IrNodeType::Struct { fields, .. } => {
                // Check for duplicate field names
                let mut field_names = std::collections::HashSet::new();
                for field in fields {
                    if !field_names.insert(&field.name) {
                        errors.push(ValidationError::DuplicateField {
                            struct_path: current_path.clone(),
                            field: field.name.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
        
        // Recursively validate children
        for child in &self.children {
            child.validate_recursive(errors, &current_path);
        }
    }
}

/// Validation errors for IR trees
#[derive(Debug, Clone)]
pub enum ValidationError {
    DuplicateParameter { function_path: String, parameter: String },
    DuplicateField { struct_path: String, field: String },
    InvalidType { node_path: String, type_name: String },
    MissingRequiredAttribute { node_path: String, attribute: String },
    InvalidGuardType { guard_path: String, guard_type: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateParameter { function_path, parameter } =>
                write!(f, "Duplicate parameter '{}' in function at {}", parameter, function_path),
            Self::DuplicateField { struct_path, field } =>
                write!(f, "Duplicate field '{}' in struct at {}", field, struct_path),
            Self::InvalidType { node_path, type_name } =>
                write!(f, "Invalid type '{}' at node {}", type_name, node_path),
            Self::MissingRequiredAttribute { node_path, attribute } =>
                write!(f, "Missing required attribute '{}' at node {}", attribute, node_path),
            Self::InvalidGuardType { guard_path, guard_type } =>
                write!(f, "Invalid guard type '{}' at guard {}", guard_type, guard_path),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Builder for creating IR nodes with fluent API
pub struct IrNodeBuilder {
    node: IrNode,
}

impl IrNodeBuilder {
    pub fn new(node_type: IrNodeType) -> Self {
        Self {
            node: IrNode::new(node_type),
        }
    }
    
    pub fn attribute(mut self, key: String, value: IrValue) -> Self {
        self.node.set_attribute(key, value);
        self
    }
    
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.node.set_metadata(key, value);
        self
    }
    
    pub fn child(mut self, child: IrNode) -> Self {
        self.node.add_child(child);
        self
    }
    
    pub fn children(mut self, children: Vec<IrNode>) -> Self {
        self.node.add_children(children);
        self
    }
    
    pub fn source_info(mut self, source_info: SourceInfo) -> Self {
        self.node.source_info = Some(source_info);
        self
    }
    
    pub fn build(self) -> IrNode {
        self.node
    }
}

/// Convenience functions for common IR node types
impl IrNode {
    /// Create a module node
    pub fn module(name: String, visibility: Visibility) -> Self {
        Self::new(IrNodeType::Module {
            name,
            visibility,
            imports: Vec::new(),
        })
    }
    
    /// Create a struct node
    pub fn struct_def(name: String, visibility: Visibility) -> Self {
        Self::new(IrNodeType::Struct {
            name,
            visibility,
            generic_params: Vec::new(),
            fields: Vec::new(),
            derives: Vec::new(),
        })
    }
    
    /// Create a function node
    pub fn function(name: String, visibility: Visibility) -> Self {
        Self::new(IrNodeType::Function {
            name,
            visibility,
            parameters: Vec::new(),
            return_type: None,
            is_async: false,
            attributes: Vec::new(),
        })
    }
    
    /// Create a guarded section node
    pub fn guarded_section(guard_id: String, guard_type: GuardType) -> Self {
        Self::new(IrNodeType::GuardedSection {
            guard_id,
            guard_type,
            default_content: None,
            user_content: None,
            is_modified: false,
        })
    }
    
    /// Create a block node
    pub fn block(scope_type: ScopeType) -> Self {
        Self::new(IrNodeType::Block {
            statements: Vec::new(),
            scope_type,
        })
    }
    
    /// Create a comment node
    pub fn comment(content: String, comment_type: CommentType) -> Self {
        Self::new(IrNodeType::Comment {
            content,
            comment_type,
        })
    }
    
    /// Create a template node
    pub fn template(template_name: String) -> Self {
        Self::new(IrNodeType::Template {
            template_name,
            parameters: HashMap::new(),
            iteration_context: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ir_node_creation() {
        let node = IrNode::module("test_module".to_string(), Visibility::Public);
        
        match &node.node_type {
            IrNodeType::Module { name, visibility, .. } => {
                assert_eq!(name, "test_module");
                assert!(matches!(visibility, Visibility::Public));
            }
            _ => panic!("Expected module node type"),
        }
    }
    
    #[test]
    fn test_ir_node_builder() {
        let node = IrNodeBuilder::new(IrNodeType::Comment {
            content: "Test comment".to_string(),
            comment_type: CommentType::Line,
        })
        .attribute("priority".to_string(), IrValue::Integer(1))
        .metadata("generator".to_string(), "test".to_string())
        .build();
        
        assert_eq!(node.get_attribute("priority"), Some(&IrValue::Integer(1)));
        assert_eq!(node.get_metadata("generator"), Some(&"test".to_string()));
    }
    
    #[test]
    fn test_node_search() {
        let mut root = IrNode::module("root".to_string(), Visibility::Public);
        let child1 = IrNode::function("func1".to_string(), Visibility::Public);
        let child2 = IrNode::function("func2".to_string(), Visibility::Private);
        
        root.add_child(child1);
        root.add_child(child2);
        
        let functions = root.find_nodes_by_type(|node_type| {
            matches!(node_type, IrNodeType::Function { .. })
        });
        
        assert_eq!(functions.len(), 2);
    }
    
    #[test]
    fn test_validation() {
        let mut func_node = IrNode::function("test_func".to_string(), Visibility::Public);
        
        // Add duplicate parameters to trigger validation error
        if let IrNodeType::Function { parameters, .. } = &mut func_node.node_type {
            parameters.push(Parameter {
                name: "param1".to_string(),
                param_type: TypeReference {
                    name: "String".to_string(),
                    generic_args: Vec::new(),
                    is_optional: false,
                    is_reference: false,
                    is_mutable_ref: false,
                },
                default_value: None,
                is_self: false,
                is_mutable: false,
            });
            parameters.push(Parameter {
                name: "param1".to_string(), // Duplicate name
                param_type: TypeReference {
                    name: "i32".to_string(),
                    generic_args: Vec::new(),
                    is_optional: false,
                    is_reference: false,
                    is_mutable_ref: false,
                },
                default_value: None,
                is_self: false,
                is_mutable: false,
            });
        }
        
        let errors = func_node.validate();
        assert_eq!(errors.len(), 1);
        
        match &errors[0] {
            ValidationError::DuplicateParameter { parameter, .. } => {
                assert_eq!(parameter, "param1");
            }
            _ => panic!("Expected DuplicateParameter error"),
        }
    }
}