//! # Slider Component
//!
//! A numeric input component with visual slider interface and range constraints.
//! Sliders provide an intuitive way for users to select numeric values within
//! a specified range using a visual sliding interface.
//!
//! This component supports both interactive slider mode and read-only display,
//! making it suitable for settings controls, parameter adjustments, and
//! numeric value displays with optional editing capabilities.

use egui::Ui;
use crate::rcl::ui::component::Component;

/// A numeric slider component with customizable range and editing modes
/// 
/// The Slider component provides an interactive way to select numeric values
/// within a defined range. It can switch between interactive slider mode and
/// read-only display mode.
/// 
/// # Features
/// 
/// - **Range Constraints**: Configurable minimum and maximum values
/// - **Visual Interface**: Intuitive slider control for value selection
/// - **Dual Modes**: Interactive slider and read-only display
/// - **Precision Control**: Supports floating-point values with display formatting
/// - **Bounds Validation**: Automatically constrains values to valid range
/// 
/// # Use Cases
/// 
/// - Settings and configuration panels
/// - Parameter adjustment interfaces
/// - Volume, brightness, and other continuous controls
/// - Numeric input with visual feedback
/// - Game settings and graphics options
/// 
/// # Examples
/// 
/// ```ignore
/// use crate::rcl::ui::basic::slider::Slider;
/// use crate::rcl::ui::component::Component;
/// 
/// let mut slider = Slider::new(50.0, 0.0, 100.0);
/// 
/// // Render in UI context
/// slider.render(&mut ui);
/// 
/// // Make editable
/// slider.set_editable(true);
/// ```
pub struct Slider {
    /// The current numeric value of the slider
    /// 
    /// This value is constrained to be within the range defined by `min` and `max`.
    /// In interactive mode, users can adjust this value using the slider control.
    pub value: f32,
    
    /// The minimum allowed value for the slider
    /// 
    /// The slider will not allow the value to go below this threshold.
    /// This defines the left boundary of the slider range.
    pub min: f32,
    
    /// The maximum allowed value for the slider
    /// 
    /// The slider will not allow the value to exceed this threshold.
    /// This defines the right boundary of the slider range.
    pub max: f32,
    
    /// Whether the slider is currently in interactive mode
    /// 
    /// When `true`, the slider displays as an interactive control that allows
    /// value adjustment. When `false`, it displays as a read-only numeric label.
    pub editable: bool,
}

impl Slider {
    /// Creates a new slider with the specified value and range
    /// 
    /// The slider is created in read-only mode by default. Use `set_editable(true)`
    /// or click the "Edit" button to enable interactive mode.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The initial value for the slider (will be clamped to range)
    /// * `min` - The minimum allowed value
    /// * `max` - The maximum allowed value
    /// 
    /// # Returns
    /// 
    /// A new `Slider` instance with the specified parameters
    /// 
    /// # Panics
    /// 
    /// Panics if `min` is greater than `max`.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// let slider = Slider::new(75.0, 0.0, 100.0);
    /// assert_eq!(slider.value(), 75.0);
    /// assert_eq!(slider.min(), 0.0);
    /// assert_eq!(slider.max(), 100.0);
    /// assert!(!slider.is_editable());
    /// ```
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        assert!(min <= max, "Slider minimum value must be less than or equal to maximum value");
        
        Self {
            value: value.clamp(min, max),
            min,
            max,
            editable: false,
        }
    }
    
    /// Sets the editable state of the slider
    /// 
    /// # Arguments
    /// 
    /// * `editable` - `true` to enable interactive mode, `false` for read-only mode
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }
    
    /// Returns whether the slider is currently in interactive mode
    /// 
    /// # Returns
    /// 
    /// `true` if the slider is interactive, `false` if read-only
    pub fn is_editable(&self) -> bool {
        self.editable
    }
    
    /// Gets the current slider value
    /// 
    /// # Returns
    /// 
    /// The current numeric value of the slider
    pub fn value(&self) -> f32 {
        self.value
    }
    
    /// Sets the slider value, clamping it to the valid range
    /// 
    /// # Arguments
    /// 
    /// * `value` - The new value for the slider (will be clamped to min..=max)
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }
    
    /// Gets the minimum allowed value
    /// 
    /// # Returns
    /// 
    /// The minimum value constraint for the slider
    pub fn min(&self) -> f32 {
        self.min
    }
    
    /// Gets the maximum allowed value
    /// 
    /// # Returns
    /// 
    /// The maximum value constraint for the slider
    pub fn max(&self) -> f32 {
        self.max
    }
    
    /// Sets a new range for the slider, adjusting the current value if necessary
    /// 
    /// # Arguments
    /// 
    /// * `min` - The new minimum value
    /// * `max` - The new maximum value
    /// 
    /// # Panics
    /// 
    /// Panics if `min` is greater than `max`.
    pub fn set_range(&mut self, min: f32, max: f32) {
        assert!(min <= max, "Slider minimum value must be less than or equal to maximum value");
        
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
    }
    
    /// Returns the range span (max - min)
    /// 
    /// # Returns
    /// 
    /// The total range of the slider as a positive value
    pub fn range(&self) -> f32 {
        self.max - self.min
    }
    
    /// Returns the current value as a percentage of the range (0.0 to 1.0)
    /// 
    /// # Returns
    /// 
    /// A value between 0.0 and 1.0 representing the relative position
    pub fn normalized_value(&self) -> f32 {
        if self.range() == 0.0 {
            0.0
        } else {
            (self.value - self.min) / self.range()
        }
    }
    
    /// Sets the value using a normalized input (0.0 to 1.0)
    /// 
    /// # Arguments
    /// 
    /// * `normalized` - A value between 0.0 and 1.0 representing relative position
    pub fn set_normalized_value(&mut self, normalized: f32) {
        let clamped = normalized.clamp(0.0, 1.0);
        self.value = self.min + (clamped * self.range());
    }
}

impl Component for Slider {
    /// Returns the component type name
    /// 
    /// # Returns
    /// 
    /// The string "Slider" identifying this component type
    fn name(&self) -> &str {
        "Slider"
    }
    
    /// Renders the slider component in either interactive or read-only mode
    /// 
    /// The rendering behavior depends on the current editable state:
    /// - **Interactive mode**: Shows a draggable slider control with value display
    /// - **Read-only mode**: Displays the current value as formatted text
    /// 
    /// An "Edit" toggle button is provided to switch between modes.
    /// 
    /// # Arguments
    /// 
    /// * `ui` - Mutable reference to the egui UI context for rendering
    /// 
    /// # Layout
    /// 
    /// The component renders vertically with:
    /// 1. The main control (slider or value label)
    /// 2. An "Edit" toggle button
    /// 
    /// # User Interaction
    /// 
    /// - In interactive mode, users can drag the slider handle to adjust the value
    /// - The slider automatically constrains values to the defined range
    /// - Real-time value updates are reflected immediately
    /// - The "Edit" button toggles between interactive and read-only modes
    /// - Value precision is displayed to 2 decimal places
    fn render(&mut self, ui: &mut Ui) {
        if self.editable {
            // Interactive mode - show draggable slider control
            // The slider automatically handles:
            // - Value clamping to the specified range
            // - Visual feedback during interaction
            // - Smooth dragging and positioning
            // - Value text display alongside the slider
            ui.add(egui::Slider::new(&mut self.value, self.min..=self.max).text("Value"));
        } else {
            // Read-only mode - display current value as formatted text
            // Shows the value with 2 decimal precision for consistency
            ui.label(format!("Value: {:.2}", self.value));
        }
        
        // Toggle button to switch between modes
        // Button text changes to provide clear state indication
        let edit_button_text = if self.editable { "Done" } else { "Edit" };
        if ui.button(edit_button_text).clicked() {
            self.editable = !self.editable;
        }
    }
    
    fn get_property(&self, name: &str) -> Option<String> {
        match name {
            "value" => Some(self.value.to_string()),
            "min" => Some(self.min.to_string()),
            "max" => Some(self.max.to_string()),
            "editable" => Some(self.editable.to_string()),
            _ => None,
        }
    }
    
    fn set_property(&mut self, name: &str, value: &str) -> bool {
        match name {
            "value" => {
                if let Ok(val) = value.parse::<f32>() {
                    self.set_value(val); // Uses the existing method that clamps to range
                    true
                } else {
                    false
                }
            }
            "min" => {
                if let Ok(min) = value.parse::<f32>() {
                    if min <= self.max {
                        self.set_range(min, self.max); // Uses existing method
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "max" => {
                if let Ok(max) = value.parse::<f32>() {
                    if max >= self.min {
                        self.set_range(self.min, max); // Uses existing method
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "editable" => {
                if let Ok(editable) = value.parse::<bool>() {
                    self.editable = editable;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    fn get_property_names(&self) -> Vec<String> {
        vec!["value".to_string(), "min".to_string(), "max".to_string(), "editable".to_string()]
    }
}
