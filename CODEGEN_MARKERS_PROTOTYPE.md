# Code Generation Markers and Rewrite Prototype

## Overview

This document describes the completed prototype for the enhanced code generation markers and rewrite system, which addresses Sprint 1 task S1-8 from the improvement plan. This system provides intelligent code generation with guarded sections that preserve user modifications while enabling automated code updates.

## Implementation Status: ✅ COMPLETE

The codegen markers and rewrite prototype has been successfully implemented with comprehensive features:

### Core Files Created
- `src/editor/codegen_markers.rs` - Enhanced marker types and code rewriter
- `src/editor/enhanced_codegen.rs` - Integration with existing code generator
- `src/editor/codegen_example.rs` - Comprehensive example demonstrating the system
- `CODEGEN_MARKERS_PROTOTYPE.md` - This documentation

### Enhanced Files
- `Cargo.toml` - Added tempfile for testing examples
- `src/editor/mod.rs` - Added new codegen modules

## Key Features Implemented

### 1. **Enhanced Marker Types**

#### Guard Markers
```rust
MarkerType::Guard {
    id: "user_logic".to_string(),
    preserve_indent: true,
    default_content: Some("// Add your logic here".to_string()),
}
```
- **Purpose**: Protect user-editable sections from regeneration
- **Features**: Indentation preservation, default content, modification tracking
- **Use Case**: Custom business logic, user-specific implementations

#### Generated Markers
```rust
MarkerType::Generated {
    id: "component_props".to_string(), 
    strategy: GenerationStrategy::Replace,
    dependencies: vec!["component.metadata".to_string()],
}
```
- **Purpose**: Auto-generated content that updates with dependencies
- **Strategies**: Replace, Merge, IfEmpty, Append, Prepend
- **Use Case**: Component properties, API interfaces, data structures

#### Conditional Markers
```rust
MarkerType::Conditional {
    id: "debug_code".to_string(),
    condition: "DEBUG=true".to_string(),
    strategy: ConditionalStrategy::Include,
}
```
- **Purpose**: Content that appears/disappears based on conditions
- **Strategies**: Include, Exclude, Switch with alternatives
- **Use Case**: Debug code, feature flags, environment-specific content

#### Import Markers
```rust
MarkerType::Import {
    import_type: ImportType::Module,
    merge_strategy: ImportMergeStrategy::Merge,
}
```
- **Purpose**: Intelligent import/dependency management
- **Types**: Module, Dependency, Local, Namespace
- **Strategies**: KeepExisting, Replace, Merge, Interactive
- **Use Case**: Automatic import resolution, dependency injection

#### Template Markers
```rust
MarkerType::Template {
    id: "field_list".to_string(),
    parameters: field_parameters,
    iteration: Some(IterationSettings {
        data_source: "component.fields".to_string(),
        item_var: "field".to_string(),
        separator: Some(",\n".to_string()),
    }),
}
```
- **Purpose**: Parameterized, repeatable code generation
- **Features**: Parameters with types, iteration support, data binding
- **Use Case**: Field lists, method generation, repetitive structures

### 2. **Multi-Language Support**

#### Supported Languages
- **Rust**: `.rs` files with `//` comments
- **TypeScript/JavaScript**: `.ts`, `.tsx`, `.js` files
- **Python**: `.py` files with `#` comments  
- **Java/C#/C++**: Various file extensions
- **HTML/CSS**: Block comment styles
- **Configuration**: JSON, YAML, TOML

#### Language Detection
```rust
let language = CodeLanguage::from_extension("rs"); // Some(CodeLanguage::Rust)
let comment_style = language.comment_style(); // CommentStyle::DoubleSlash
```

### 3. **Intelligent Code Rewriting**

#### Marker Parsing
```rust
let mut rewriter = CodeRewriter::new(CodeLanguage::Rust, source_code);
rewriter.parse_markers()?; // Extracts all markers from code
```

#### Content Preservation
```rust
// User modifications in guard sections are automatically preserved
if existing_marker.is_modified {
    updated_marker.content = existing_marker.content.clone();
    updated_marker.is_modified = true;
}
```

#### Code Regeneration
```rust
let final_code = rewriter.rewrite_with_markers(updated_markers)?;
// Preserves user code while updating generated sections
```

### 4. **Template System Integration**

#### Enhanced Templates
```rust
let template = EnhancedTemplateBuilder::new("react_component".to_string(), template_content)
    .support_languages(vec![CodeLanguage::TypeScript, CodeLanguage::JavaScript])
    .add_guard("custom_logic".to_string(), 10, Some("// Custom logic here".to_string()))
    .add_generated("props".to_string(), 6, ContentGenerator::Component {
        component_type: "Button".to_string(),
        properties: button_properties,
    })
    .build();
```

#### Content Generators
- **Static**: Fixed content strings
- **Template**: Variable-based template expansion
- **Function**: Programmable content generation  
- **Component**: Integration with derive macro metadata

### 5. **Dependency Tracking**

#### Intelligent Regeneration
```rust
let files_to_regenerate = generator.update_dependencies(changed_files)?;
// Only regenerates files that depend on changed components
```

#### Marker Dependencies
```rust
if marker.should_regenerate(&changed_dependencies) {
    // Regenerate only affected markers, preserve others
}
```

## Architecture Benefits

### 1. **Preservation of User Code**
- **Guard Sections**: User code is never overwritten
- **Modification Tracking**: System knows what users have changed
- **Intelligent Merging**: Generated updates don't conflict with user code

### 2. **Multi-Language Code Generation**
- **Language Detection**: Automatic detection from file extensions
- **Comment Styles**: Proper marker syntax for each language
- **Cross-Platform**: Works with different coding ecosystems

### 3. **Sophisticated Generation Strategies**
- **Conditional**: Content appears based on runtime conditions
- **Template-Based**: Parameterized generation with iteration
- **Dependency-Aware**: Only regenerates when dependencies change

### 4. **Integration with Component System**
- **Derive Macro Integration**: Uses component metadata for generation
- **Property Schemas**: Generates type-safe interfaces
- **Event System**: Coordinates with IDE event bus

## Real-World Example Workflow

### 1. **Initial Component Generation**
```typescript
// Generated React component with markers
import React from 'react';

interface UserProfileProps {
    // <codegen:generated:props:start>
    name: string;
    email: string;
    // <codegen:generated:props:end>
}

const UserProfile: React.FC<UserProfileProps> = (props) => {
    // <codegen:guard:component_logic:start>
    // Add your component logic here
    // <codegen:guard:component_logic:end>
    
    return (
        <div className="UserProfile">
            {/* <codegen:guard:custom_jsx:start> */}
            {/* Add your custom JSX here */}
            {/* <codegen:guard:custom_jsx:end> */}
        </div>
    );
};

export default UserProfile;
```

### 2. **User Adds Custom Logic**
```typescript
// User modifies guard sections
// <codegen:guard:component_logic:start>
const [isEditing, setIsEditing] = useState(false);

const handleEdit = () => {
    setIsEditing(true);
};

const handleSave = () => {
    setIsEditing(false);
    // Save user profile changes
};
// <codegen:guard:component_logic:end>
```

### 3. **Component Schema Changes**
```rust
// Component definition updated with new field
#[derive(ComponentMetadata)]
#[component(name = "UserProfile", category = "Display")]
struct UserProfile {
    #[property(default = "")]
    name: String,
    
    #[property(default = "")]  
    email: String,
    
    #[property(default = false)] // NEW FIELD
    is_admin: bool,
}
```

### 4. **Automatic Regeneration**
```typescript
// System regenerates with preserved user code
interface UserProfileProps {
    // <codegen:generated:props:start>
    name: string;
    email: string;
    is_admin: boolean; // NEW PROPERTY ADDED
    // <codegen:generated:props:end>
}

const UserProfile: React.FC<UserProfileProps> = (props) => {
    // <codegen:guard:component_logic:start>
    const [isEditing, setIsEditing] = useState(false);

    const handleEdit = () => {
        setIsEditing(true);
    };

    const handleSave = () => {
        setIsEditing(false);
        // Save user profile changes
    };
    // USER CODE PRESERVED!
    // <codegen:guard:component_logic:end>
    
    return (
        <div className="UserProfile">
            {/* <codegen:guard:custom_jsx:start> */}
            {/* User's custom JSX preserved */}
            {/* <codegen:guard:custom_jsx:end> */}
        </div>
    );
};
```

## Technical Implementation Details

### Marker Parsing Engine
- **AST-aware**: Understands code structure, not just text
- **Position Tracking**: Maintains accurate line/column/offset positions
- **Error Handling**: Graceful handling of malformed markers
- **Performance**: Efficient parsing of large files

### Code Rewriting Algorithm
1. **Parse existing markers** from source code
2. **Generate new content** based on templates and data
3. **Preserve user modifications** in guard sections
4. **Update generated sections** with new content
5. **Maintain formatting** and indentation
6. **Write final code** with all markers properly placed

### Template Processing
- **Variable Substitution**: `{{variable}}` syntax with type safety
- **Conditional Logic**: `{{#if condition}}...{{/if}}` support
- **Iteration**: `{{#each items}}...{{/each}}` for repetitive content
- **Partial Templates**: Reusable template components

## Integration with Improvement Plan

This prototype addresses key improvement plan objectives:

### Sprint 1 (S1-8) Requirements
✅ **Enhanced Code Markers** - Multiple marker types with sophisticated behavior  
✅ **Rewrite Prototype** - Complete rewriting system with user code preservation  
✅ **Multi-Language Support** - Works across different programming languages  
✅ **Template Integration** - Seamless integration with existing template system

### Benefits for Future Phases
- **P1**: Visual designer can generate code with preserved customizations
- **P2**: Advanced templates with conditional and iterative generation  
- **P3**: Full code generation pipeline with dependency tracking
- **P4**: Production-ready system with comprehensive language support

## Testing and Validation

### Comprehensive Test Coverage
- **Unit Tests**: All helper functions and core algorithms
- **Integration Tests**: Complete generation workflows
- **Language Tests**: Multi-language marker parsing and generation
- **Preservation Tests**: User code preservation across regenerations

### Example Test Cases
```rust
#[test]
fn test_user_code_preservation() {
    let mut generator = EnhancedCodeGenerator::new(temp_dir);
    
    // Generate initial code
    let initial_code = generator.generate_enhanced_code(/*...*/).unwrap();
    
    // Simulate user modifications
    let modified_code = add_user_code_to_guards(initial_code);
    write_file(&output_file, modified_code);
    
    // Regenerate with changes
    let regenerated_code = generator.generate_enhanced_code(/*...*/).unwrap();
    
    // Assert user code is preserved
    assert!(regenerated_code.contains("// User's custom logic"));
    assert!(regenerated_code.contains("custom_method()"));
}
```

## Future Enhancements (Post-Prototype)

### 1. **Advanced Conditional Logic**
- Complex expression evaluation
- Runtime condition checking
- Environment-based generation

### 2. **Visual Marker Editor**
- GUI for marker management
- Visual template editing
- Real-time preview updates

### 3. **Version Control Integration**
- Marker-aware diffs
- Conflict resolution for markers
- Branch-specific generation

### 4. **Performance Optimizations**
- Incremental parsing
- Cached template compilation
- Parallel generation

## Conclusion

The Code Generation Markers and Rewrite Prototype is **complete and fully functional**. It provides a sophisticated foundation for intelligent code generation that:

- **Preserves User Code**: Never overwrites user modifications
- **Supports Multiple Languages**: Works across different programming ecosystems  
- **Enables Complex Generation**: Conditional, template-based, and dependency-aware
- **Integrates Seamlessly**: Works with existing component and template systems

**Status: ✅ S1-8 Complete - All Sprint 1 tasks finished**

This completes the entire Sprint 1 backlog from the improvement plan, providing a solid foundation for the advanced IDE features planned in subsequent phases. The system is ready for integration into the visual designer and can serve as the basis for the comprehensive code generation pipeline described in the improvement plan.

### Sprint 1 Summary: All Tasks Complete ✅

1. **S1-1**: ✅ CI workflow with fmt, clippy, and tests
2. **S1-2**: ✅ Tracing logging with feature gate  
3. **S1-3**: ✅ Basic EventBus for decoupled communication
4. **S1-4**: ✅ Enhanced rope-based TextBuffer with comprehensive tests
5. **S1-5**: ✅ Terminal core abstraction to reduce duplication
6. **S1-6**: ✅ File watcher integration with notify crate
7. **S1-7**: ✅ Component metadata derive macro prototype
8. **S1-8**: ✅ Codegen markers and rewrite prototype

**Ready to proceed with Sprint 2 (S2) tasks from the improvement plan.**