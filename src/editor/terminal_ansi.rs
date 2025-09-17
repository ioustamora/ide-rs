//! ANSI StyledSpan pipeline for terminal rendering
//!
//! Provides ANSI parser, StyledSpan buffer, scrollback ring, and theme colors
//! as specified in the improvement plan Phase P0.

use std::collections::VecDeque;
use egui::Color32;
use serde::{Serialize, Deserialize};

/// ANSI parser with StyledSpan output
pub struct AnsiParser {
    /// Current parser state
    pub state: ParserState,
    /// Current style being built
    pub current_style: AnsiStyle,
    /// Style stack for nested formatting
    pub style_stack: Vec<AnsiStyle>,
    /// Color palette for indexed colors
    pub color_palette: ColorPalette,
    /// Parser configuration
    pub config: ParserConfig,
}

/// Parser state machine
#[derive(Clone, Debug, PartialEq)]
pub enum ParserState {
    /// Normal text parsing
    Normal,
    /// Escape sequence started (\x1B)
    Escape,
    /// Control sequence started (\x1B[)
    Csi,
    /// Operating system command (\x1BOsc)
    Osc,
    /// Device control string
    Dcs,
    /// Application program command
    Apc,
    /// Privacy message
    Pm,
}

/// Styled text span with ANSI formatting
#[derive(Clone, Debug, PartialEq)]
pub struct StyledSpan {
    /// Text content
    pub text: String,
    /// Style information
    pub style: AnsiStyle,
    /// Span type for special handling
    pub span_type: SpanType,
}

/// ANSI style information
#[derive(Clone, Debug, PartialEq)]
pub struct AnsiStyle {
    /// Foreground color
    pub foreground: AnsiColor,
    /// Background color
    pub background: AnsiColor,
    /// Text attributes
    pub attributes: TextAttributes,
}

/// ANSI color representation
#[derive(Clone, Debug, PartialEq)]
pub enum AnsiColor {
    /// Default terminal color
    Default,
    /// Named color (0-15)
    Named(u8),
    /// RGB color (256-color mode or truecolor)
    Rgb(u8, u8, u8),
    /// Indexed color (16-255)
    Indexed(u8),
}

/// Text formatting attributes
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TextAttributes {
    /// Bold/bright text
    pub bold: bool,
    /// Dim text
    pub dim: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Strikethrough text
    pub strikethrough: bool,
    /// Blinking text
    pub blink: bool,
    /// Inverted colors
    pub inverse: bool,
    /// Hidden text
    pub hidden: bool,
}

/// Type of styled span for special handling
#[derive(Clone, Debug, PartialEq)]
pub enum SpanType {
    /// Normal text
    Text,
    /// Hyperlink
    Link(String), // URL
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Success message
    Success,
    /// Info message
    Info,
    /// File path (clickable)
    FilePath,
    /// Line number reference
    LineRef { file: String, line: u32 },
}

/// Terminal scrollback buffer using ring buffer
pub struct ScrollbackBuffer {
    /// Ring buffer of styled lines
    pub lines: VecDeque<StyledLine>,
    /// Maximum number of lines to keep
    pub max_lines: usize,
    /// Current line being built
    pub current_line: StyledLine,
    /// Viewport start line
    pub viewport_start: usize,
    /// Viewport height in lines
    pub viewport_height: usize,
    /// Buffer statistics
    pub stats: BufferStats,
}

/// Single line of styled text
#[derive(Clone, Debug)]
pub struct StyledLine {
    /// Styled spans in this line
    pub spans: Vec<StyledSpan>,
    /// Line timestamp
    pub timestamp: std::time::Instant,
    /// Line type for categorization
    pub line_type: LineType,
    /// Whether line wraps to next line
    pub wrapped: bool,
}

/// Line type categorization
#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    /// Normal output
    Normal,
    /// Command input
    Input,
    /// Error output
    Error,
    /// Warning output
    Warning,
    /// Success output
    Success,
    /// Info output
    Info,
    /// System message
    System,
}

/// Buffer statistics
#[derive(Clone, Debug, Default)]
pub struct BufferStats {
    /// Total lines processed
    pub total_lines: usize,
    /// Total characters processed
    pub total_chars: usize,
    /// Lines dropped due to buffer limit
    pub dropped_lines: usize,
    /// Parse errors encountered
    pub parse_errors: usize,
}

/// Color palette for terminal themes
#[derive(Clone, Debug)]
pub struct ColorPalette {
    /// Standard 16 colors
    pub standard: [Color32; 16],
    /// Extended 216 colors (6x6x6 cube)
    pub extended: [Color32; 216],
    /// Grayscale ramp (24 colors)
    pub grayscale: [Color32; 24],
    /// Default foreground color
    pub default_foreground: Color32,
    /// Default background color
    pub default_background: Color32,
    /// Cursor color
    pub cursor_color: Color32,
    /// Selection background color
    pub selection_background: Color32,
}

/// Theme configuration for terminal styling
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalTheme {
    /// Theme name
    pub name: String,
    /// Color palette
    pub colors: ThemeColors,
    /// Typography settings
    pub typography: Typography,
    /// UI styling
    pub ui_style: UiStyle,
}

/// Theme colors
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Background color
    pub background: [u8; 4], // RGBA
    /// Foreground color
    pub foreground: [u8; 4], // RGBA
    /// Cursor color
    pub cursor: [u8; 4], // RGBA
    /// Selection color
    pub selection: [u8; 4], // RGBA
    /// ANSI color palette
    pub ansi_colors: [[u8; 4]; 16],
    /// Bright ANSI colors
    pub bright_colors: [[u8; 4]; 16],
}

/// Typography settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Typography {
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: f32,
    /// Line height
    pub line_height: f32,
    /// Character spacing
    pub character_spacing: f32,
}

/// UI styling
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiStyle {
    /// Border color
    pub border_color: [u8; 4],
    /// Scrollbar colors
    pub scrollbar: ScrollbarStyle,
    /// Padding around content
    pub padding: f32,
}

/// Scrollbar styling
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScrollbarStyle {
    pub track_color: [u8; 4],
    pub thumb_color: [u8; 4],
    pub thumb_hover_color: [u8; 4],
}

/// Parser configuration
#[derive(Clone, Debug)]
pub struct ParserConfig {
    /// Maximum escape sequence length
    pub max_escape_length: usize,
    /// Enable 8-bit control sequences
    pub enable_8bit_controls: bool,
    /// Enable truecolor support (24-bit)
    pub enable_truecolor: bool,
    /// Enable hyperlink support (OSC 8)
    pub enable_hyperlinks: bool,
    /// Enable file path detection
    pub enable_file_path_detection: bool,
    /// Strict parsing mode
    pub strict_mode: bool,
}

/// ANSI control sequence
#[derive(Clone, Debug)]
pub struct ControlSequence {
    /// Sequence type
    pub sequence_type: SequenceType,
    /// Parameters
    pub params: Vec<u16>,
    /// Intermediate characters
    pub intermediates: Vec<u8>,
    /// Final character
    pub final_char: u8,
}

/// Types of ANSI sequences
#[derive(Clone, Debug, PartialEq)]
pub enum SequenceType {
    /// SGR (Select Graphic Rendition)
    Sgr,
    /// Cursor movement
    CursorMove,
    /// Erase sequences
    Erase,
    /// Mode setting
    Mode,
    /// Operating system command
    Osc,
    /// Device control string
    Dcs,
    /// Unknown sequence
    Unknown,
}

impl AnsiParser {
    /// Create new ANSI parser
    pub fn new() -> Self {
        Self {
            state: ParserState::Normal,
            current_style: AnsiStyle::default(),
            style_stack: Vec::new(),
            color_palette: ColorPalette::default_dark_theme(),
            config: ParserConfig::default(),
        }
    }

    /// Parse text into styled spans
    pub fn parse(&mut self, input: &str) -> Vec<StyledSpan> {
        let mut spans = Vec::new();
        let mut current_text = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match self.state {
                ParserState::Normal => {
                    match ch {
                        '\x1B' => {
                            // Start of escape sequence
                            if !current_text.is_empty() {
                                spans.push(StyledSpan {
                                    text: current_text.clone(),
                                    style: self.current_style.clone(),
                                    span_type: SpanType::Text,
                                });
                                current_text.clear();
                            }
                            self.state = ParserState::Escape;
                        }
                        _ => {
                            current_text.push(ch);
                        }
                    }
                }
                ParserState::Escape => {
                    match ch {
                        '[' => {
                            self.state = ParserState::Csi;
                        }
                        ']' => {
                            self.state = ParserState::Osc;
                        }
                        'P' => {
                            self.state = ParserState::Dcs;
                        }
                        '_' => {
                            self.state = ParserState::Apc;
                        }
                        '^' => {
                            self.state = ParserState::Pm;
                        }
                        _ => {
                            // Invalid escape sequence, return to normal
                            current_text.push('\x1B');
                            current_text.push(ch);
                            self.state = ParserState::Normal;
                        }
                    }
                }
                ParserState::Csi => {
                    if let Some(sequence) = self.parse_csi_sequence(ch, &mut chars) {
                        self.apply_csi_sequence(&sequence);
                    }
                    self.state = ParserState::Normal;
                }
                ParserState::Osc => {
                    if let Some(command) = self.parse_osc_command(ch, &mut chars) {
                        if let Some(span) = self.apply_osc_command(&command) {
                            spans.push(span);
                        }
                    }
                    self.state = ParserState::Normal;
                }
                _ => {
                    // Skip other control sequences for now
                    self.state = ParserState::Normal;
                }
            }
        }

        // Add remaining text
        if !current_text.is_empty() {
            spans.push(StyledSpan {
                text: current_text,
                style: self.current_style.clone(),
                span_type: SpanType::Text,
            });
        }

        spans
    }

    /// Parse CSI (Control Sequence Introducer) sequence
    fn parse_csi_sequence(&self, first_char: char, chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<ControlSequence> {
        let mut params = Vec::new();
        let mut current_param = String::new();
        let mut intermediates = Vec::new();
        let mut final_char = first_char;

        // Parse parameters
        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    current_param.push(chars.next().unwrap());
                }
                ';' => {
                    chars.next(); // consume ';'
                    if !current_param.is_empty() {
                        if let Ok(param) = current_param.parse::<u16>() {
                            params.push(param);
                        }
                        current_param.clear();
                    } else {
                        params.push(0); // Default parameter
                    }
                }
                ' '..='/' => {
                    // Intermediate characters
                    intermediates.push(chars.next().unwrap() as u8);
                }
                '@'..='~' => {
                    // Final character
                    final_char = chars.next().unwrap();
                    break;
                }
                _ => {
                    // Invalid character, abort
                    return None;
                }
            }
        }

        // Add final parameter
        if !current_param.is_empty() {
            if let Ok(param) = current_param.parse::<u16>() {
                params.push(param);
            }
        }

        let sequence_type = match final_char {
            'm' => SequenceType::Sgr,
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' => SequenceType::CursorMove,
            'J' | 'K' => SequenceType::Erase,
            'h' | 'l' => SequenceType::Mode,
            _ => SequenceType::Unknown,
        };

        Some(ControlSequence {
            sequence_type,
            params,
            intermediates,
            final_char: final_char as u8,
        })
    }

    /// Apply CSI sequence to current style
    fn apply_csi_sequence(&mut self, sequence: &ControlSequence) {
        match sequence.sequence_type {
            SequenceType::Sgr => {
                self.apply_sgr_sequence(&sequence.params);
            }
            _ => {
                // Other sequences not implemented yet
            }
        }
    }

    /// Apply SGR (Select Graphic Rendition) sequence
    fn apply_sgr_sequence(&mut self, params: &[u16]) {
        if params.is_empty() {
            // Reset all attributes
            self.current_style = AnsiStyle::default();
            return;
        }

        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => {
                    // Reset all
                    self.current_style = AnsiStyle::default();
                }
                1 => {
                    // Bold
                    self.current_style.attributes.bold = true;
                }
                2 => {
                    // Dim
                    self.current_style.attributes.dim = true;
                }
                3 => {
                    // Italic
                    self.current_style.attributes.italic = true;
                }
                4 => {
                    // Underline
                    self.current_style.attributes.underline = true;
                }
                5 => {
                    // Blink
                    self.current_style.attributes.blink = true;
                }
                7 => {
                    // Inverse
                    self.current_style.attributes.inverse = true;
                }
                8 => {
                    // Hidden
                    self.current_style.attributes.hidden = true;
                }
                9 => {
                    // Strikethrough
                    self.current_style.attributes.strikethrough = true;
                }
                22 => {
                    // Normal intensity (not bold, not dim)
                    self.current_style.attributes.bold = false;
                    self.current_style.attributes.dim = false;
                }
                23 => {
                    // Not italic
                    self.current_style.attributes.italic = false;
                }
                24 => {
                    // Not underlined
                    self.current_style.attributes.underline = false;
                }
                25 => {
                    // Not blinking
                    self.current_style.attributes.blink = false;
                }
                27 => {
                    // Not inverse
                    self.current_style.attributes.inverse = false;
                }
                28 => {
                    // Not hidden
                    self.current_style.attributes.hidden = false;
                }
                29 => {
                    // Not strikethrough
                    self.current_style.attributes.strikethrough = false;
                }
                30..=37 => {
                    // Foreground colors (black to white)
                    self.current_style.foreground = AnsiColor::Named((params[i] - 30) as u8);
                }
                38 => {
                    // Extended foreground color
                    if i + 2 < params.len() {
                        match params[i + 1] {
                            2 => {
                                // RGB color
                                if i + 4 < params.len() {
                                    self.current_style.foreground = AnsiColor::Rgb(
                                        params[i + 2] as u8,
                                        params[i + 3] as u8,
                                        params[i + 4] as u8,
                                    );
                                    i += 4;
                                }
                            }
                            5 => {
                                // Indexed color
                                self.current_style.foreground = AnsiColor::Indexed(params[i + 2] as u8);
                                i += 2;
                            }
                            _ => {}
                        }
                    }
                }
                39 => {
                    // Default foreground color
                    self.current_style.foreground = AnsiColor::Default;
                }
                40..=47 => {
                    // Background colors (black to white)
                    self.current_style.background = AnsiColor::Named((params[i] - 40) as u8);
                }
                48 => {
                    // Extended background color
                    if i + 2 < params.len() {
                        match params[i + 1] {
                            2 => {
                                // RGB color
                                if i + 4 < params.len() {
                                    self.current_style.background = AnsiColor::Rgb(
                                        params[i + 2] as u8,
                                        params[i + 3] as u8,
                                        params[i + 4] as u8,
                                    );
                                    i += 4;
                                }
                            }
                            5 => {
                                // Indexed color
                                self.current_style.background = AnsiColor::Indexed(params[i + 2] as u8);
                                i += 2;
                            }
                            _ => {}
                        }
                    }
                }
                49 => {
                    // Default background color
                    self.current_style.background = AnsiColor::Default;
                }
                90..=97 => {
                    // Bright foreground colors
                    self.current_style.foreground = AnsiColor::Named((params[i] - 90 + 8) as u8);
                }
                100..=107 => {
                    // Bright background colors
                    self.current_style.background = AnsiColor::Named((params[i] - 100 + 8) as u8);
                }
                _ => {
                    // Unknown parameter, ignore
                }
            }
            i += 1;
        }
    }

    /// Parse OSC (Operating System Command) sequence
    fn parse_osc_command(&self, _first_char: char, chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
        let mut command = String::new();
        
        while let Some(ch) = chars.next() {
            match ch {
                '\x07' => break, // BEL terminator
                '\x1B' => {
                    // ESC terminator
                    if chars.peek() == Some(&'\\') {
                        chars.next(); // consume '\'
                        break;
                    }
                }
                _ => {
                    command.push(ch);
                }
            }
        }
        
        if command.is_empty() {
            None
        } else {
            Some(command)
        }
    }

    /// Apply OSC command (e.g., hyperlinks)
    fn apply_osc_command(&self, command: &str) -> Option<StyledSpan> {
        if command.starts_with("8;") {
            // Hyperlink
            let parts: Vec<&str> = command.splitn(3, ';').collect();
            if parts.len() == 3 {
                let url = parts[2];
                return Some(StyledSpan {
                    text: url.to_string(),
                    style: self.current_style.clone(),
                    span_type: SpanType::Link(url.to_string()),
                });
            }
        }
        None
    }

    /// Convert ANSI color to egui Color32
    pub fn ansi_color_to_color32(&self, color: &AnsiColor) -> Color32 {
        match color {
            AnsiColor::Default => self.color_palette.default_foreground,
            AnsiColor::Named(index) => {
                if *index < 16 {
                    self.color_palette.standard[*index as usize]
                } else {
                    self.color_palette.default_foreground
                }
            }
            AnsiColor::Rgb(r, g, b) => Color32::from_rgb(*r, *g, *b),
            AnsiColor::Indexed(index) => {
                if *index < 16 {
                    self.color_palette.standard[*index as usize]
                } else if *index < 232 {
                    // 6x6x6 color cube
                    let cube_index = (*index - 16) as usize;
                    if cube_index < 216 {
                        self.color_palette.extended[cube_index]
                    } else {
                        self.color_palette.default_foreground
                    }
                } else {
                    // Grayscale ramp (232-255)
                    let gray_index = (*index - 232) as usize;
                    if gray_index < 24 {
                        self.color_palette.grayscale[gray_index]
                    } else {
                        self.color_palette.default_foreground
                    }
                }
            }
        }
    }
}

impl ScrollbackBuffer {
    /// Create new scrollback buffer
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::new(),
            max_lines,
            current_line: StyledLine::new(),
            viewport_start: 0,
            viewport_height: 25, // Default terminal height
            stats: BufferStats::default(),
        }
    }

    /// Add styled spans to current line
    pub fn add_spans(&mut self, spans: Vec<StyledSpan>) {
        for span in spans {
            self.stats.total_chars += span.text.len();
            self.current_line.spans.push(span);
        }
    }

    /// Finalize current line and start new one
    pub fn new_line(&mut self) {
        if !self.current_line.spans.is_empty() || self.current_line.line_type != LineType::Normal {
            self.lines.push_back(self.current_line.clone());
            self.stats.total_lines += 1;
            
            // Limit buffer size
            if self.lines.len() > self.max_lines {
                self.lines.pop_front();
                self.stats.dropped_lines += 1;
            }
        }
        
        self.current_line = StyledLine::new();
    }

    /// Get lines in viewport
    pub fn get_viewport_lines(&self) -> Vec<&StyledLine> {
        let start = self.viewport_start.min(self.lines.len());
        let end = (self.viewport_start + self.viewport_height).min(self.lines.len());
        
        self.lines.range(start..end).collect()
    }

    /// Scroll viewport up
    pub fn scroll_up(&mut self, lines: usize) {
        self.viewport_start = self.viewport_start.saturating_sub(lines);
    }

    /// Scroll viewport down
    pub fn scroll_down(&mut self, lines: usize) {
        let max_start = self.lines.len().saturating_sub(self.viewport_height);
        self.viewport_start = (self.viewport_start + lines).min(max_start);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.viewport_start = self.lines.len().saturating_sub(self.viewport_height);
    }

    /// Check if scrolled to bottom
    pub fn is_at_bottom(&self) -> bool {
        self.viewport_start + self.viewport_height >= self.lines.len()
    }

    /// Clear all lines
    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_line = StyledLine::new();
        self.viewport_start = 0;
        self.stats = BufferStats::default();
    }

    /// Search for text in buffer
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        for (line_index, line) in self.lines.iter().enumerate() {
            for (span_index, span) in line.spans.iter().enumerate() {
                if let Some(char_index) = span.text.find(query) {
                    results.push(SearchResult {
                        line_index,
                        span_index,
                        char_index,
                        match_length: query.len(),
                    });
                }
            }
        }
        
        results
    }
}

impl StyledLine {
    /// Create new empty styled line
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            timestamp: std::time::Instant::now(),
            line_type: LineType::Normal,
            wrapped: false,
        }
    }

    /// Get total text content
    pub fn text(&self) -> String {
        self.spans.iter().map(|span| span.text.as_str()).collect()
    }

    /// Get line length in characters
    pub fn len(&self) -> usize {
        self.spans.iter().map(|span| span.text.len()).sum()
    }

    /// Check if line is empty
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|span| span.text.is_empty())
    }
}

impl ColorPalette {
    /// Create default dark theme color palette
    pub fn default_dark_theme() -> Self {
        Self {
            standard: [
                Color32::from_rgb(0, 0, 0),       // Black
                Color32::from_rgb(128, 0, 0),     // Red
                Color32::from_rgb(0, 128, 0),     // Green
                Color32::from_rgb(128, 128, 0),   // Yellow
                Color32::from_rgb(0, 0, 128),     // Blue
                Color32::from_rgb(128, 0, 128),   // Magenta
                Color32::from_rgb(0, 128, 128),   // Cyan
                Color32::from_rgb(192, 192, 192), // White
                Color32::from_rgb(128, 128, 128), // Bright Black
                Color32::from_rgb(255, 0, 0),     // Bright Red
                Color32::from_rgb(0, 255, 0),     // Bright Green
                Color32::from_rgb(255, 255, 0),   // Bright Yellow
                Color32::from_rgb(0, 0, 255),     // Bright Blue
                Color32::from_rgb(255, 0, 255),   // Bright Magenta
                Color32::from_rgb(0, 255, 255),   // Bright Cyan
                Color32::from_rgb(255, 255, 255), // Bright White
            ],
            extended: Self::generate_extended_colors(),
            grayscale: Self::generate_grayscale_ramp(),
            default_foreground: Color32::from_rgb(192, 192, 192),
            default_background: Color32::from_rgb(0, 0, 0),
            cursor_color: Color32::from_rgb(255, 255, 255),
            selection_background: Color32::from_rgb(0, 120, 215),
        }
    }

    /// Generate 6x6x6 color cube for extended colors
    fn generate_extended_colors() -> [Color32; 216] {
        let mut colors = [Color32::BLACK; 216];
        let levels = [0, 95, 135, 175, 215, 255];
        
        for r in 0..6 {
            for g in 0..6 {
                for b in 0..6 {
                    let index = 36 * r + 6 * g + b;
                    colors[index] = Color32::from_rgb(levels[r], levels[g], levels[b]);
                }
            }
        }
        
        colors
    }

    /// Generate 24-level grayscale ramp
    fn generate_grayscale_ramp() -> [Color32; 24] {
        let mut colors = [Color32::BLACK; 24];
        
        for i in 0..24 {
            let level = 8 + i * 10;
            colors[i] = Color32::from_rgb(level as u8, level as u8, level as u8);
        }
        
        colors
    }
}

impl Default for AnsiStyle {
    fn default() -> Self {
        Self {
            foreground: AnsiColor::Default,
            background: AnsiColor::Default,
            attributes: TextAttributes::default(),
        }
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_escape_length: 256,
            enable_8bit_controls: true,
            enable_truecolor: true,
            enable_hyperlinks: true,
            enable_file_path_detection: true,
            strict_mode: false,
        }
    }
}

/// Search result in scrollback buffer
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub line_index: usize,
    pub span_index: usize,
    pub char_index: usize,
    pub match_length: usize,
}

/// Terminal renderer for egui integration
pub struct TerminalRenderer {
    /// ANSI parser
    pub parser: AnsiParser,
    /// Scrollback buffer
    pub buffer: ScrollbackBuffer,
    /// Current theme
    pub theme: TerminalTheme,
    /// Rendering configuration
    pub render_config: RenderConfig,
}

/// Rendering configuration
#[derive(Clone, Debug)]
pub struct RenderConfig {
    /// Lines to render outside viewport (overscan)
    pub overscan_lines: usize,
    /// Enable span caching
    pub cache_spans: bool,
    /// Maximum cached spans
    pub max_cached_spans: usize,
}

impl TerminalRenderer {
    /// Create new terminal renderer
    pub fn new() -> Self {
        Self {
            parser: AnsiParser::new(),
            buffer: ScrollbackBuffer::new(10000),
            theme: TerminalTheme::default_dark(),
            render_config: RenderConfig::default(),
        }
    }

    /// Process terminal input
    pub fn process_input(&mut self, input: &str) {
        let spans = self.parser.parse(input);
        
        for span in spans {
            if span.text.contains('\n') {
                // Handle newlines
                let lines: Vec<&str> = span.text.split('\n').collect();
                for (i, line) in lines.iter().enumerate() {
                    if !line.is_empty() {
                        self.buffer.add_spans(vec![StyledSpan {
                            text: line.to_string(),
                            style: span.style.clone(),
                            span_type: span.span_type.clone(),
                        }]);
                    }
                    if i < lines.len() - 1 {
                        self.buffer.new_line();
                    }
                }
            } else {
                self.buffer.add_spans(vec![span]);
            }
        }
    }

    /// Render terminal to egui
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let viewport_lines = self.buffer.get_viewport_lines();
        
        egui::ScrollArea::vertical()
            .stick_to_bottom(self.buffer.is_at_bottom())
            .show(ui, |ui| {
                for line in viewport_lines {
                    ui.horizontal(|ui| {
                        for span in &line.spans {
                            let fg_color = self.parser.ansi_color_to_color32(&span.style.foreground);
                            let mut text = egui::RichText::new(&span.text).color(fg_color);
                            
                            // Apply text attributes
                            if span.style.attributes.bold {
                                text = text.strong();
                            }
                            if span.style.attributes.italic {
                                text = text.italics();
                            }
                            if span.style.attributes.underline {
                                text = text.underline();
                            }
                            if span.style.attributes.strikethrough {
                                text = text.strikethrough();
                            }
                            
                            // Handle special span types
                            match &span.span_type {
                                SpanType::Link(url) => {
                                    if ui.link(text).clicked() {
                                        // Open link (would need platform-specific implementation)
                                        println!("Opening link: {}", url);
                                    }
                                }
                                SpanType::FilePath => {
                                    if ui.button(text).clicked() {
                                        // Open file (would integrate with editor)
                                        println!("Opening file: {}", span.text);
                                    }
                                }
                                SpanType::LineRef { file, line } => {
                                    if ui.button(text).clicked() {
                                        // Jump to line in file
                                        println!("Opening {}:{}", file, line);
                                    }
                                }
                                _ => {
                                    ui.label(text);
                                }
                            }
                        }
                    });
                }
            });
    }
}

impl TerminalTheme {
    /// Create default dark theme
    pub fn default_dark() -> Self {
        Self {
            name: "Dark".to_string(),
            colors: ThemeColors {
                background: [0, 0, 0, 255],
                foreground: [192, 192, 192, 255],
                cursor: [255, 255, 255, 255],
                selection: [0, 120, 215, 128],
                ansi_colors: [
                    [0, 0, 0, 255],         // Black
                    [128, 0, 0, 255],       // Red
                    [0, 128, 0, 255],       // Green
                    [128, 128, 0, 255],     // Yellow
                    [0, 0, 128, 255],       // Blue
                    [128, 0, 128, 255],     // Magenta
                    [0, 128, 128, 255],     // Cyan
                    [192, 192, 192, 255],   // White
                    [128, 128, 128, 255],   // Bright Black
                    [255, 0, 0, 255],       // Bright Red
                    [0, 255, 0, 255],       // Bright Green
                    [255, 255, 0, 255],     // Bright Yellow
                    [0, 0, 255, 255],       // Bright Blue
                    [255, 0, 255, 255],     // Bright Magenta
                    [0, 255, 255, 255],     // Bright Cyan
                    [255, 255, 255, 255],   // Bright White
                ],
                bright_colors: [
                    [64, 64, 64, 255],      // Bright versions
                    [255, 64, 64, 255],
                    [64, 255, 64, 255],
                    [255, 255, 64, 255],
                    [64, 64, 255, 255],
                    [255, 64, 255, 255],
                    [64, 255, 255, 255],
                    [255, 255, 255, 255],
                    [128, 128, 128, 255],
                    [255, 128, 128, 255],
                    [128, 255, 128, 255],
                    [255, 255, 128, 255],
                    [128, 128, 255, 255],
                    [255, 128, 255, 255],
                    [128, 255, 255, 255],
                    [255, 255, 255, 255],
                ],
            },
            typography: Typography {
                font_family: "Cascadia Code".to_string(),
                font_size: 14.0,
                line_height: 1.2,
                character_spacing: 0.0,
            },
            ui_style: UiStyle {
                border_color: [64, 64, 64, 255],
                scrollbar: ScrollbarStyle {
                    track_color: [32, 32, 32, 255],
                    thumb_color: [96, 96, 96, 255],
                    thumb_hover_color: [128, 128, 128, 255],
                },
                padding: 8.0,
            },
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            overscan_lines: 5,
            cache_spans: true,
            max_cached_spans: 1000,
        }
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_parser_basic() {
        let mut parser = AnsiParser::new();
        
        let spans = parser.parse("Hello World");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, "Hello World");
        assert_eq!(spans[0].style.foreground, AnsiColor::Default);
    }

    #[test]
    fn test_ansi_parser_colors() {
        let mut parser = AnsiParser::new();
        
        let spans = parser.parse("\x1B[31mRed Text\x1B[0m");
        assert!(spans.len() >= 1);
        
        // Find the red text span
        let red_span = spans.iter().find(|s| s.text == "Red Text").unwrap();
        assert_eq!(red_span.style.foreground, AnsiColor::Named(1)); // Red
    }

    #[test]
    fn test_ansi_parser_bold() {
        let mut parser = AnsiParser::new();
        
        let spans = parser.parse("\x1B[1mBold Text\x1B[0m");
        assert!(spans.len() >= 1);
        
        let bold_span = spans.iter().find(|s| s.text == "Bold Text").unwrap();
        assert!(bold_span.style.attributes.bold);
    }

    #[test]
    fn test_scrollback_buffer() {
        let mut buffer = ScrollbackBuffer::new(100);
        
        buffer.add_spans(vec![StyledSpan {
            text: "Line 1".to_string(),
            style: AnsiStyle::default(),
            span_type: SpanType::Text,
        }]);
        buffer.new_line();
        
        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.lines[0].text(), "Line 1");
    }

    #[test]
    fn test_color_conversion() {
        let parser = AnsiParser::new();
        
        let color = parser.ansi_color_to_color32(&AnsiColor::Named(1)); // Red
        assert_eq!(color, Color32::from_rgb(128, 0, 0));
        
        let rgb_color = parser.ansi_color_to_color32(&AnsiColor::Rgb(255, 128, 64));
        assert_eq!(rgb_color, Color32::from_rgb(255, 128, 64));
    }
}