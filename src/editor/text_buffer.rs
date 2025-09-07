//! Core TextBuffer abstraction for efficient text editing
//!
//! Provides rope-based text storage with diff-based LSP changes, structured cursor/selection model,
//! and operation undo stack as specified in the improvement plan.

use std::collections::VecDeque;
use std::ops::Range;
use std::path::PathBuf;
use ropey::{Rope, RopeSlice};

/// Core text buffer with rope-based storage for efficient editing operations
pub struct TextBuffer {
    /// Rope-based text storage for efficient insertions/deletions
    pub rope: Rope,
    /// File path this buffer represents
    pub file_path: Option<PathBuf>,
    /// Buffer is dirty (has unsaved changes)
    pub is_dirty: bool,
    /// Line ending style
    pub line_ending: LineEnding,
    /// Text encoding
    pub encoding: TextEncoding,
    /// Undo/redo stack
    pub undo_stack: UndoStack,
    /// Current version for LSP synchronization
    pub version: u64,
    /// Change tracking for LSP diff-based updates
    pub change_tracker: ChangeTracker,
    /// Buffer metadata
    pub metadata: BufferMetadata,
}

/// Structured cursor and selection model
#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    /// Primary cursor position
    pub position: TextPosition,
    /// Selection anchor (None if no selection)
    pub anchor: Option<TextPosition>,
    /// Cursor affinity for line wrap edge cases
    pub affinity: CursorAffinity,
}

/// Text position with line and column
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextPosition {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based, UTF-8 character index)
    pub column: usize,
    /// Byte offset in the document
    pub offset: usize,
}

/// Cursor affinity for ambiguous positions
#[derive(Clone, Debug, PartialEq)]
pub enum CursorAffinity {
    /// Cursor prefers upstream position
    Upstream,
    /// Cursor prefers downstream position
    Downstream,
}

/// Selection model supporting multiple cursors
#[derive(Clone, Debug)]
pub struct SelectionSet {
    /// All cursors/selections
    pub cursors: Vec<Cursor>,
    /// Primary cursor index
    pub primary: usize,
}

/// Line ending styles
#[derive(Clone, Debug, PartialEq)]
pub enum LineEnding {
    /// Unix-style line endings (\n)
    Unix,
    /// Windows-style line endings (\r\n)
    Windows,
    /// Classic Mac line endings (\r)
    Mac,
}

/// Text encoding types
#[derive(Clone, Debug, PartialEq)]
pub enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Latin1,
}

/// Undo/redo stack with operation history
pub struct UndoStack {
    /// Undo operations stack
    pub undo_stack: VecDeque<TextOperation>,
    /// Redo operations stack
    pub redo_stack: VecDeque<TextOperation>,
    /// Maximum undo history size
    pub max_history: usize,
    /// Current transaction group
    pub current_transaction: Option<TransactionGroup>,
}

/// Individual text operation for undo/redo
#[derive(Clone, Debug)]
pub struct TextOperation {
    /// Operation type
    pub operation: OperationType,
    /// Position where operation occurred
    pub position: TextPosition,
    /// Text involved in operation
    pub text: String,
    /// Selection state before operation
    pub old_selection: SelectionSet,
    /// Selection state after operation
    pub new_selection: SelectionSet,
    /// Timestamp of operation
    pub timestamp: std::time::Instant,
}

/// Types of text operations
#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    /// Insert text at position
    Insert,
    /// Delete text range
    Delete,
    /// Replace text range
    Replace,
}

/// Transaction group for atomic undo/redo
#[derive(Clone, Debug)]
pub struct TransactionGroup {
    /// Operations in this transaction
    pub operations: Vec<TextOperation>,
    /// Transaction description
    pub description: String,
    /// Transaction start time
    pub start_time: std::time::Instant,
}

/// Change tracking for LSP diff-based updates
pub struct ChangeTracker {
    /// Recent changes for LSP synchronization
    pub changes: VecDeque<TextChange>,
    /// Last synced version
    pub last_synced_version: u64,
    /// Maximum change history
    pub max_changes: usize,
}

/// Individual text change event
#[derive(Clone, Debug)]
pub struct TextChange {
    /// Range of text that changed
    pub range: TextRange,
    /// New text content
    pub text: String,
    /// Length of replaced text
    pub range_length: usize,
    /// Change version
    pub version: u64,
}

/// Text range specification
#[derive(Clone, Debug, PartialEq)]
pub struct TextRange {
    /// Start position
    pub start: TextPosition,
    /// End position
    pub end: TextPosition,
}

/// Buffer metadata
#[derive(Clone, Debug)]
pub struct BufferMetadata {
    /// File size in bytes
    pub file_size: usize,
    /// Line count
    pub line_count: usize,
    /// Character count
    pub char_count: usize,
    /// Created timestamp
    pub created_at: std::time::Instant,
    /// Last modified timestamp
    pub modified_at: std::time::Instant,
    /// File permissions (if applicable)
    pub readonly: bool,
}

impl TextBuffer {
    /// Create new empty text buffer
    pub fn new() -> Self {
        let rope = Rope::new();
        let metadata = BufferMetadata {
            file_size: 0,
            line_count: 1,
            char_count: 0,
            created_at: std::time::Instant::now(),
            modified_at: std::time::Instant::now(),
            readonly: false,
        };

        Self {
            rope,
            file_path: None,
            is_dirty: false,
            line_ending: LineEnding::Unix,
            encoding: TextEncoding::Utf8,
            undo_stack: UndoStack::new(),
            version: 0,
            change_tracker: ChangeTracker::new(),
            metadata,
        }
    }

    /// Create text buffer from string content
    pub fn from_string(content: String) -> Self {
        let rope = Rope::from_str(&content);
        let metadata = BufferMetadata {
            file_size: content.len(),
            line_count: rope.len_lines(),
            char_count: rope.len_chars(),
            created_at: std::time::Instant::now(),
            modified_at: std::time::Instant::now(),
            readonly: false,
        };

        Self {
            rope,
            file_path: None,
            is_dirty: false,
            line_ending: Self::detect_line_ending(&content),
            encoding: TextEncoding::Utf8,
            undo_stack: UndoStack::new(),
            version: 0,
            change_tracker: ChangeTracker::new(),
            metadata,
        }
    }

    /// Create text buffer from file
    pub fn from_file(file_path: PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(&file_path)?;
        let mut buffer = Self::from_string(content);
        buffer.file_path = Some(file_path);
        buffer.is_dirty = false;
        Ok(buffer)
    }

    /// Insert text at position
    pub fn insert(&mut self, position: TextPosition, text: &str, selection: SelectionSet) -> Result<SelectionSet, TextBufferError> {
        // Validate position
        if !self.is_valid_position(&position) {
            return Err(TextBufferError::InvalidPosition(position));
        }

        // Convert position to byte offset
        let offset = self.position_to_offset(&position)?;
        
        // Perform insertion
        self.rope.insert(offset, text);
        
        // Update version and metadata
        self.version += 1;
        self.is_dirty = true;
        self.update_metadata();
        
        // Create operation for undo stack
        let operation = TextOperation {
            operation: OperationType::Insert,
            position: position.clone(),
            text: text.to_string(),
            old_selection: selection.clone(),
            new_selection: self.adjust_selection_after_insert(&selection, &position, text.len()),
            timestamp: std::time::Instant::now(),
        };
        
        // Record change for LSP
        let change = TextChange {
            range: TextRange {
                start: position.clone(),
                end: position.clone(),
            },
            text: text.to_string(),
            range_length: 0,
            version: self.version,
        };
        
        // Update stacks
        self.undo_stack.push_operation(operation);
        self.change_tracker.record_change(change);
        
        Ok(self.adjust_selection_after_insert(&selection, &position, text.len()))
    }

    /// Delete text range
    pub fn delete(&mut self, range: TextRange, selection: SelectionSet) -> Result<SelectionSet, TextBufferError> {
        // Validate range
        if !self.is_valid_range(&range) {
            return Err(TextBufferError::InvalidRange(range));
        }

        // Convert range to byte offsets
        let start_offset = self.position_to_offset(&range.start)?;
        let end_offset = self.position_to_offset(&range.end)?;
        
        // Get text being deleted for undo
        let deleted_text = self.rope.slice(start_offset..end_offset).to_string();
        
        // Perform deletion
        self.rope.remove(start_offset..end_offset);
        
        // Update version and metadata
        self.version += 1;
        self.is_dirty = true;
        self.update_metadata();
        
        // Create operation for undo stack
        let operation = TextOperation {
            operation: OperationType::Delete,
            position: range.start.clone(),
            text: deleted_text,
            old_selection: selection.clone(),
            new_selection: self.adjust_selection_after_delete(&selection, &range),
            timestamp: std::time::Instant::now(),
        };
        
        // Record change for LSP
        let change = TextChange {
            range: range.clone(),
            text: String::new(),
            range_length: end_offset - start_offset,
            version: self.version,
        };
        
        // Update stacks
        self.undo_stack.push_operation(operation);
        self.change_tracker.record_change(change);
        
        Ok(self.adjust_selection_after_delete(&selection, &range))
    }

    /// Replace text range
    pub fn replace(&mut self, range: TextRange, text: &str, selection: SelectionSet) -> Result<SelectionSet, TextBufferError> {
        // Validate range
        if !self.is_valid_range(&range) {
            return Err(TextBufferError::InvalidRange(range));
        }

        // Convert range to byte offsets
        let start_offset = self.position_to_offset(&range.start)?;
        let end_offset = self.position_to_offset(&range.end)?;
        
        // Get text being replaced for undo
        let old_text = self.rope.slice(start_offset..end_offset).to_string();
        
        // Perform replacement
        self.rope.remove(start_offset..end_offset);
        self.rope.insert(start_offset, text);
        
        // Update version and metadata
        self.version += 1;
        self.is_dirty = true;
        self.update_metadata();
        
        // Create operation for undo stack
        let operation = TextOperation {
            operation: OperationType::Replace,
            position: range.start.clone(),
            text: old_text,
            old_selection: selection.clone(),
            new_selection: self.adjust_selection_after_replace(&selection, &range, text.len()),
            timestamp: std::time::Instant::now(),
        };
        
        // Record change for LSP
        let change = TextChange {
            range: range.clone(),
            text: text.to_string(),
            range_length: end_offset - start_offset,
            version: self.version,
        };
        
        // Update stacks
        self.undo_stack.push_operation(operation);
        self.change_tracker.record_change(change);
        
        Ok(self.adjust_selection_after_replace(&selection, &range, text.len()))
    }

    /// Undo last operation
    pub fn undo(&mut self) -> Result<SelectionSet, TextBufferError> {
        if let Some(operation) = self.undo_stack.pop_undo() {
            match operation.operation {
                OperationType::Insert => {
                    // Undo insert by deleting the inserted text
                    let end_pos = self.offset_to_position(
                        self.position_to_offset(&operation.position)? + operation.text.len()
                    )?;
                    let range = TextRange {
                        start: operation.position,
                        end: end_pos,
                    };
                    self.rope.remove(self.position_to_offset(&range.start)?..self.position_to_offset(&range.end)?);
                }
                OperationType::Delete => {
                    // Undo delete by inserting the deleted text
                    let offset = self.position_to_offset(&operation.position)?;
                    self.rope.insert(offset, &operation.text);
                }
                OperationType::Replace => {
                    // Undo replace by replacing with original text
                    let end_pos = self.offset_to_position(
                        self.position_to_offset(&operation.position)? + operation.text.len()
                    )?;
                    let range = TextRange {
                        start: operation.position,
                        end: end_pos,
                    };
                    self.rope.remove(self.position_to_offset(&range.start)?..self.position_to_offset(&range.end)?);
                    self.rope.insert(self.position_to_offset(&range.start)?, &operation.text);
                }
            }
            
            self.version += 1;
            self.is_dirty = true;
            self.update_metadata();
            
            // Move operation to redo stack
            self.undo_stack.push_redo(operation.clone());
            
            Ok(operation.old_selection)
        } else {
            Err(TextBufferError::NothingToUndo)
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) -> Result<SelectionSet, TextBufferError> {
        if let Some(operation) = self.undo_stack.pop_redo() {
            match operation.operation {
                OperationType::Insert => {
                    let offset = self.position_to_offset(&operation.position)?;
                    self.rope.insert(offset, &operation.text);
                }
                OperationType::Delete => {
                    let end_pos = self.offset_to_position(
                        self.position_to_offset(&operation.position)? + operation.text.len()
                    )?;
                    let range = TextRange {
                        start: operation.position,
                        end: end_pos,
                    };
                    self.rope.remove(self.position_to_offset(&range.start)?..self.position_to_offset(&range.end)?);
                }
                OperationType::Replace => {
                    let end_pos = self.offset_to_position(
                        self.position_to_offset(&operation.position)? + operation.text.len()
                    )?;
                    let range = TextRange {
                        start: operation.position,
                        end: end_pos,
                    };
                    self.rope.remove(self.position_to_offset(&range.start)?..self.position_to_offset(&range.end)?);
                    // Insert new text (stored in the operation's text field for replace operations)
                    // Note: For proper redo, we'd need to store both old and new text
                }
            }
            
            self.version += 1;
            self.is_dirty = true;
            self.update_metadata();
            
            // Move operation back to undo stack
            self.undo_stack.push_operation(operation.clone());
            
            Ok(operation.new_selection)
        } else {
            Err(TextBufferError::NothingToRedo)
        }
    }

    /// Get text content as string
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get line content as string
    pub fn line_content(&self, line: usize) -> Result<String, TextBufferError> {
        if line >= self.metadata.line_count {
            return Err(TextBufferError::LineOutOfBounds(line));
        }
        
        Ok(self.rope.line(line).to_string())
    }

    /// Get text slice for range
    pub fn slice(&self, range: TextRange) -> Result<String, TextBufferError> {
        let start_offset = self.position_to_offset(&range.start)?;
        let end_offset = self.position_to_offset(&range.end)?;
        Ok(self.rope.slice(start_offset..end_offset).to_string())
    }

    /// Convert position to byte offset
    pub fn position_to_offset(&self, position: &TextPosition) -> Result<usize, TextBufferError> {
        if position.line >= self.metadata.line_count {
            return Err(TextBufferError::LineOutOfBounds(position.line));
        }
        
        let line_start = self.rope.line_to_byte(position.line);
        let line_slice = self.rope.line(position.line);
        
        if position.column > line_slice.len_chars() {
            return Err(TextBufferError::ColumnOutOfBounds(position.column));
        }
        
        let column_bytes = line_slice.slice(..position.column).len_bytes();
        Ok(line_start + column_bytes)
    }

    /// Convert byte offset to position
    pub fn offset_to_position(&self, offset: usize) -> Result<TextPosition, TextBufferError> {
        if offset > self.rope.len_bytes() {
            return Err(TextBufferError::OffsetOutOfBounds(offset));
        }
        
        let line = self.rope.byte_to_line(offset);
        let line_start = self.rope.line_to_byte(line);
        let column_bytes = offset - line_start;
        let line_slice = self.rope.line(line);
        let column = line_slice.slice(..line_slice.byte_to_char(column_bytes)).len_chars();
        
        Ok(TextPosition {
            line,
            column,
            offset,
        })
    }

    /// Get changes since last LSP sync
    pub fn get_changes_since(&self, version: u64) -> Vec<TextChange> {
        self.change_tracker.changes
            .iter()
            .filter(|change| change.version > version)
            .cloned()
            .collect()
    }

    /// Mark buffer as synced with LSP
    pub fn mark_synced(&mut self) {
        self.change_tracker.last_synced_version = self.version;
    }

    /// Validate position is within buffer bounds
    fn is_valid_position(&self, position: &TextPosition) -> bool {
        if position.line >= self.metadata.line_count {
            return false;
        }
        
        let line_len = self.rope.line(position.line).len_chars();
        position.column <= line_len
    }

    /// Validate range is within buffer bounds
    fn is_valid_range(&self, range: &TextRange) -> bool {
        self.is_valid_position(&range.start) && 
        self.is_valid_position(&range.end) &&
        range.start <= range.end
    }

    /// Update buffer metadata after changes
    fn update_metadata(&mut self) {
        self.metadata.file_size = self.rope.len_bytes();
        self.metadata.line_count = self.rope.len_lines();
        self.metadata.char_count = self.rope.len_chars();
        self.metadata.modified_at = std::time::Instant::now();
    }

    /// Detect line ending style from content
    fn detect_line_ending(content: &str) -> LineEnding {
        if content.contains("\r\n") {
            LineEnding::Windows
        } else if content.contains('\r') {
            LineEnding::Mac
        } else {
            LineEnding::Unix
        }
    }

    /// Adjust selection after insert operation
    fn adjust_selection_after_insert(&self, selection: &SelectionSet, position: &TextPosition, insert_len: usize) -> SelectionSet {
        let mut new_selection = selection.clone();
        for cursor in &mut new_selection.cursors {
            if cursor.position >= *position {
                cursor.position.offset += insert_len;
                // Update line/column based on new offset
                if let Ok(new_pos) = self.offset_to_position(cursor.position.offset) {
                    cursor.position = new_pos;
                }
            }
            if let Some(ref mut anchor) = cursor.anchor {
                if *anchor >= *position {
                    anchor.offset += insert_len;
                    // Update line/column based on new offset
                    if let Ok(new_pos) = self.offset_to_position(anchor.offset) {
                        *anchor = new_pos;
                    }
                }
            }
        }
        new_selection
    }

    /// Adjust selection after delete operation
    fn adjust_selection_after_delete(&self, selection: &SelectionSet, range: &TextRange) -> SelectionSet {
        let mut new_selection = selection.clone();
        let delete_len = range.end.offset - range.start.offset;
        
        for cursor in &mut new_selection.cursors {
            if cursor.position > range.end {
                cursor.position.offset -= delete_len;
                if let Ok(new_pos) = self.offset_to_position(cursor.position.offset) {
                    cursor.position = new_pos;
                }
            } else if cursor.position > range.start {
                cursor.position = range.start.clone();
            }
            
            if let Some(ref mut anchor) = cursor.anchor {
                if *anchor > range.end {
                    anchor.offset -= delete_len;
                    if let Ok(new_pos) = self.offset_to_position(anchor.offset) {
                        *anchor = new_pos;
                    }
                } else if *anchor > range.start {
                    *anchor = range.start.clone();
                }
            }
        }
        new_selection
    }

    /// Adjust selection after replace operation
    fn adjust_selection_after_replace(&self, selection: &SelectionSet, range: &TextRange, new_len: usize) -> SelectionSet {
        let mut new_selection = selection.clone();
        let old_len = range.end.offset - range.start.offset;
        let len_diff = new_len as i64 - old_len as i64;
        
        for cursor in &mut new_selection.cursors {
            if cursor.position > range.end {
                cursor.position.offset = (cursor.position.offset as i64 + len_diff) as usize;
                if let Ok(new_pos) = self.offset_to_position(cursor.position.offset) {
                    cursor.position = new_pos;
                }
            } else if cursor.position > range.start {
                cursor.position = range.start.clone();
                cursor.position.offset += new_len;
                if let Ok(new_pos) = self.offset_to_position(cursor.position.offset) {
                    cursor.position = new_pos;
                }
            }
            
            if let Some(ref mut anchor) = cursor.anchor {
                if *anchor > range.end {
                    anchor.offset = (anchor.offset as i64 + len_diff) as usize;
                    if let Ok(new_pos) = self.offset_to_position(anchor.offset) {
                        *anchor = new_pos;
                    }
                } else if *anchor > range.start {
                    *anchor = range.start.clone();
                    anchor.offset += new_len;
                    if let Ok(new_pos) = self.offset_to_position(anchor.offset) {
                        *anchor = new_pos;
                    }
                }
            }
        }
        new_selection
    }
}

impl UndoStack {
    /// Create new undo stack
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 1000,
            current_transaction: None,
        }
    }

    /// Push operation to undo stack
    pub fn push_operation(&mut self, operation: TextOperation) {
        // Clear redo stack when new operation is added
        self.redo_stack.clear();
        
        // Add to undo stack
        self.undo_stack.push_back(operation);
        
        // Limit stack size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }
    }

    /// Pop operation from undo stack
    pub fn pop_undo(&mut self) -> Option<TextOperation> {
        self.undo_stack.pop_back()
    }

    /// Push operation to redo stack
    pub fn push_redo(&mut self, operation: TextOperation) {
        self.redo_stack.push_back(operation);
        
        // Limit stack size
        if self.redo_stack.len() > self.max_history {
            self.redo_stack.pop_front();
        }
    }

    /// Pop operation from redo stack
    pub fn pop_redo(&mut self) -> Option<TextOperation> {
        self.redo_stack.pop_back()
    }

    /// Start transaction group
    pub fn begin_transaction(&mut self, description: String) {
        self.current_transaction = Some(TransactionGroup {
            operations: Vec::new(),
            description,
            start_time: std::time::Instant::now(),
        });
    }

    /// End transaction group
    pub fn end_transaction(&mut self) {
        if let Some(transaction) = self.current_transaction.take() {
            if !transaction.operations.is_empty() {
                // Convert transaction to single compound operation
                // This would need more sophisticated implementation
            }
        }
    }
}

impl ChangeTracker {
    /// Create new change tracker
    pub fn new() -> Self {
        Self {
            changes: VecDeque::new(),
            last_synced_version: 0,
            max_changes: 100,
        }
    }

    /// Record a text change
    pub fn record_change(&mut self, change: TextChange) {
        self.changes.push_back(change);
        
        // Limit change history
        if self.changes.len() > self.max_changes {
            self.changes.pop_front();
        }
    }

    /// Get changes since version
    pub fn get_changes_since(&self, version: u64) -> Vec<TextChange> {
        self.changes
            .iter()
            .filter(|change| change.version > version)
            .cloned()
            .collect()
    }
}

impl SelectionSet {
    /// Create new selection set with single cursor
    pub fn single(position: TextPosition) -> Self {
        Self {
            cursors: vec![Cursor {
                position,
                anchor: None,
                affinity: CursorAffinity::Downstream,
            }],
            primary: 0,
        }
    }

    /// Create selection with range
    pub fn range(start: TextPosition, end: TextPosition) -> Self {
        Self {
            cursors: vec![Cursor {
                position: end,
                anchor: Some(start),
                affinity: CursorAffinity::Downstream,
            }],
            primary: 0,
        }
    }

    /// Get primary cursor
    pub fn primary_cursor(&self) -> &Cursor {
        &self.cursors[self.primary]
    }

    /// Get primary cursor mutably
    pub fn primary_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[self.primary]
    }
}

/// Text buffer error types
#[derive(Debug, thiserror::Error)]
pub enum TextBufferError {
    #[error("Invalid position: line {0:?}")]
    InvalidPosition(TextPosition),
    #[error("Invalid range: {0:?}")]
    InvalidRange(TextRange),
    #[error("Line {0} out of bounds")]
    LineOutOfBounds(usize),
    #[error("Column {0} out of bounds")]
    ColumnOutOfBounds(usize),
    #[error("Offset {0} out of bounds")]
    OffsetOutOfBounds(usize),
    #[error("Nothing to undo")]
    NothingToUndo,
    #[error("Nothing to redo")]
    NothingToRedo,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CursorAffinity {
    fn default() -> Self {
        CursorAffinity::Downstream
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_buffer_creation() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.to_string(), "");
        assert_eq!(buffer.metadata.line_count, 1);
        assert_eq!(buffer.metadata.char_count, 0);
    }

    #[test]
    fn test_insert_operation() {
        let mut buffer = TextBuffer::from_string("Hello".to_string());
        let position = TextPosition { line: 0, column: 5, offset: 5 };
        let selection = SelectionSet::single(position.clone());
        
        let result = buffer.insert(position, " World", selection);
        assert!(result.is_ok());
        assert_eq!(buffer.to_string(), "Hello World");
        assert_eq!(buffer.version, 1);
        assert!(buffer.is_dirty);
    }

    #[test]
    fn test_delete_operation() {
        let mut buffer = TextBuffer::from_string("Hello World".to_string());
        let range = TextRange {
            start: TextPosition { line: 0, column: 5, offset: 5 },
            end: TextPosition { line: 0, column: 11, offset: 11 },
        };
        let selection = SelectionSet::single(range.start.clone());
        
        let result = buffer.delete(range, selection);
        assert!(result.is_ok());
        assert_eq!(buffer.to_string(), "Hello");
        assert_eq!(buffer.version, 1);
    }

    #[test]
    fn test_undo_redo() {
        let mut buffer = TextBuffer::from_string("Hello".to_string());
        let position = TextPosition { line: 0, column: 5, offset: 5 };
        let selection = SelectionSet::single(position.clone());
        
        // Insert text
        buffer.insert(position, " World", selection).unwrap();
        assert_eq!(buffer.to_string(), "Hello World");
        
        // Undo
        let undo_result = buffer.undo().unwrap();
        assert_eq!(buffer.to_string(), "Hello");
        assert_eq!(undo_result.cursors.len(), 1);
        
        // Redo
        let redo_result = buffer.redo().unwrap();
        assert_eq!(buffer.to_string(), "Hello World");
        assert_eq!(redo_result.cursors.len(), 1);
    }

    #[test]
    fn test_multi_line_operations() {
        let mut buffer = TextBuffer::from_string("Line 1\nLine 2\nLine 3".to_string());
        assert_eq!(buffer.metadata.line_count, 3);
        
        // Insert at beginning of line 2
        let position = TextPosition { line: 1, column: 0, offset: 7 };
        let selection = SelectionSet::single(position.clone());
        buffer.insert(position, "New ", selection).unwrap();
        
        assert_eq!(buffer.to_string(), "Line 1\nNew Line 2\nLine 3");
        assert_eq!(buffer.metadata.line_count, 3);
    }

    #[test]
    fn test_position_conversions() {
        let buffer = TextBuffer::from_string("Hello\nWorld\nRust".to_string());
        
        // Test line/column to offset
        let pos = TextPosition { line: 1, column: 2, offset: 0 };
        let offset = buffer.position_to_offset(&pos).unwrap();
        assert_eq!(offset, 8); // "Hello\n" (6) + "Wo" (2) = 8
        
        // Test offset to line/column
        let position = buffer.offset_to_position(8).unwrap();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 2);
    }

    #[test]
    fn test_line_operations() {
        let buffer = TextBuffer::from_string("Line 1\nLine 2\nLine 3".to_string());
        
        let line1 = buffer.line_content(1).unwrap();
        assert_eq!(line1, "Line 2");
        
        let lines = buffer.lines_in_range(0, 2).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
    }

    #[test]
    fn test_search_functionality() {
        let buffer = TextBuffer::from_string("Hello World\nHello Rust\nGoodbye World".to_string());
        
        let matches = buffer.find_all("Hello");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].start.line, 0);
        assert_eq!(matches[0].start.column, 0);
        assert_eq!(matches[1].start.line, 1);
        assert_eq!(matches[1].start.column, 0);
    }

    #[test]
    fn test_replace_functionality() {
        let mut buffer = TextBuffer::from_string("Hello World Hello".to_string());
        let selection = SelectionSet::single(TextPosition { line: 0, column: 0, offset: 0 });
        
        let replaced_count = buffer.replace_all("Hello", "Hi", selection).unwrap();
        assert_eq!(replaced_count, 2);
        assert_eq!(buffer.to_string(), "Hi World Hi");
    }

    #[test]
    fn test_change_tracking() {
        let mut buffer = TextBuffer::new();
        
        // Make some changes
        let pos1 = TextPosition { line: 0, column: 0, offset: 0 };
        let selection1 = SelectionSet::single(pos1.clone());
        buffer.insert(pos1, "Hello", selection1).unwrap();
        
        let pos2 = TextPosition { line: 0, column: 5, offset: 5 };
        let selection2 = SelectionSet::single(pos2.clone());
        buffer.insert(pos2, " World", selection2).unwrap();
        
        // Check LSP changes
        let changes = buffer.get_lsp_changes();
        assert_eq!(changes.len(), 2);
        assert_eq!(buffer.version, 2);
    }

    #[test]
    fn test_selection_operations() {
        let mut buffer = TextBuffer::from_string("Hello World".to_string());
        let mut selection = SelectionSet::new();
        
        // Add multiple cursors
        selection.add_cursor(Cursor::new(TextPosition { line: 0, column: 0, offset: 0 }));
        selection.add_cursor(Cursor::new(TextPosition { line: 0, column: 6, offset: 6 }));
        
        assert_eq!(selection.cursors.len(), 2);
        
        // Test selection text extraction
        let range = TextRange {
            start: TextPosition { line: 0, column: 0, offset: 0 },
            end: TextPosition { line: 0, column: 5, offset: 5 },
        };
        let selected_text = buffer.text_in_range(&range).unwrap();
        assert_eq!(selected_text, "Hello");
    }

    #[test]
    fn test_file_operations() {
        use std::fs;
        use std::io::Write;
        
        // Create a temporary file
        let test_content = "Test file content\nSecond line";
        let temp_file = std::env::temp_dir().join("test_buffer.txt");
        
        // Write test content to file
        fs::write(&temp_file, test_content).unwrap();
        
        // Load buffer from file
        let buffer = TextBuffer::from_file(temp_file.clone()).unwrap();
        assert_eq!(buffer.to_string(), test_content);
        assert_eq!(buffer.file_path, Some(temp_file.clone()));
        assert!(!buffer.is_dirty);
        
        // Clean up
        fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_large_text_performance() {
        // Test with larger text to ensure rope efficiency
        let large_text = "A line of text.\n".repeat(1000);
        let mut buffer = TextBuffer::from_string(large_text.clone());
        
        assert_eq!(buffer.metadata.line_count, 1000);
        assert_eq!(buffer.to_string(), large_text);
        
        // Test insertion in middle
        let middle_pos = TextPosition { line: 500, column: 0, offset: 500 * 16 };
        let selection = SelectionSet::single(middle_pos.clone());
        buffer.insert(middle_pos, "INSERTED ", selection).unwrap();
        
        // Should be efficient and maintain correctness
        let line_500 = buffer.line_content(500).unwrap();
        assert!(line_500.starts_with("INSERTED "));
    }

    #[test]
    fn test_unicode_handling() {
        let unicode_text = "Hello 🦀 Rust 🌍 World";
        let mut buffer = TextBuffer::from_string(unicode_text.to_string());
        
        // Test position calculations with unicode
        let pos = TextPosition { line: 0, column: 6, offset: 0 };
        let offset = buffer.position_to_offset(&pos).unwrap();
        
        // Insert after the crab emoji
        let insert_pos = TextPosition { line: 0, column: 7, offset: offset + 4 };
        let selection = SelectionSet::single(insert_pos.clone());
        buffer.insert(insert_pos, " and", selection).unwrap();
        
        assert!(buffer.to_string().contains("🦀 and Rust"));
    }

    #[test]
    fn test_error_conditions() {
        let buffer = TextBuffer::from_string("Short text".to_string());
        
        // Test invalid line
        assert!(buffer.line_content(10).is_err());
        
        // Test invalid position
        let invalid_pos = TextPosition { line: 10, column: 0, offset: 100 };
        assert!(buffer.position_to_offset(&invalid_pos).is_err());
        
        // Test invalid offset
        assert!(buffer.offset_to_position(1000).is_err());
    }

    #[test]
    fn test_metadata_updates() {
        let mut buffer = TextBuffer::new();
        assert_eq!(buffer.metadata.line_count, 1);
        assert_eq!(buffer.metadata.char_count, 0);
        
        // Insert text with newlines
        let pos = TextPosition { line: 0, column: 0, offset: 0 };
        let selection = SelectionSet::single(pos.clone());
        buffer.insert(pos, "Line 1\nLine 2\nLine 3", selection).unwrap();
        
        assert_eq!(buffer.metadata.line_count, 3);
        assert_eq!(buffer.metadata.char_count, 20);
        assert!(buffer.metadata.last_modified.is_some());
    }

    #[test]
    fn test_concurrent_modifications() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let buffer = Arc::new(Mutex::new(TextBuffer::from_string("Initial text".to_string())));
        let mut handles = vec![];
        
        // Spawn multiple threads that modify the buffer
        for i in 0..5 {
            let buffer_clone = Arc::clone(&buffer);
            let handle = thread::spawn(move || {
                let mut buf = buffer_clone.lock().unwrap();
                let pos = TextPosition { line: 0, column: 12, offset: 12 };
                let selection = SelectionSet::single(pos.clone());
                buf.insert(pos, &format!(" {}", i), selection).unwrap();
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_buffer = buffer.lock().unwrap();
        assert!(final_buffer.to_string().len() > "Initial text".len());
        assert!(final_buffer.version >= 5);
    }
    
    #[test]
    fn test_benchmark_operations() {
        // Benchmark test for performance validation with larger operations
        use std::time::Instant;
        
        let start = Instant::now();
        let mut buffer = TextBuffer::new();
        
        // Insert a large amount of text
        for i in 0..1000 {
            let pos = TextPosition { 
                line: i, 
                column: 0, 
                offset: buffer.rope.len_chars()
            };
            let selection = SelectionSet::single(pos.clone());
            buffer.insert(pos, &format!("Line {} with some content\n", i), selection).unwrap();
        }
        
        let insert_duration = start.elapsed();
        assert!(insert_duration.as_millis() < 1000, "Large insert should complete in under 1 second");
        
        // Test search performance
        let start = Instant::now();
        let matches = buffer.find_all("Line 500");
        let search_duration = start.elapsed();
        
        assert!(!matches.is_empty());
        assert!(search_duration.as_millis() < 100, "Search should complete quickly");
    }
}