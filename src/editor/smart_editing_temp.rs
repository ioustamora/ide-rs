    
    /// Generate spacing suggestions for consistent component spacing
    fn generate_spacing_suggestions(&self, 
                                  visual_designer: &VisualDesigner,
                                  dragging_component: usize,
                                  current_position: Pos2) -> Vec<SpacingGuide> {
        let mut suggestions = Vec::new();
        
        // Common spacing values to suggest
        let common_spacings = [8.0, 16.0, 24.0, 32.0, 48.0, 64.0];
        
        for (component_idx, position) in &visual_designer.layout.positions {
            if *component_idx == dragging_component {
                continue;
            }
            
            let size = visual_designer.layout.sizes.get(component_idx)
                .copied().unwrap_or(Vec2::new(100.0, 30.0));
            
            // Calculate distances
            let horizontal_distance = (current_position.x - (position.x + size.x)).abs();
            let vertical_distance = (current_position.y - (position.y + size.y)).abs();
            
            // Check for common spacing patterns
            for &spacing in &common_spacings {
                let tolerance = 4.0;
                
                // Horizontal spacing suggestion
                if (horizontal_distance - spacing).abs() < tolerance {
                    suggestions.push(SpacingGuide {
                        start: Pos2::new(position.x + size.x, position.y + size.y / 2.0),
                        end: Pos2::new(current_position.x, position.y + size.y / 2.0),
                        spacing,
                        components: (*component_idx, dragging_component),
                        confidence: 1.0 - (horizontal_distance - spacing).abs() / tolerance,
                    });
                }
                
                // Vertical spacing suggestion
                if (vertical_distance - spacing).abs() < tolerance {
                    suggestions.push(SpacingGuide {
                        start: Pos2::new(position.x + size.x / 2.0, position.y + size.y),
                        end: Pos2::new(position.x + size.x / 2.0, current_position.y),
                        spacing,
                        components: (*component_idx, dragging_component),
                        confidence: 1.0 - (vertical_distance - spacing).abs() / tolerance,
                    });
                }
            }
        }
        
        // Sort by confidence
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top suggestions
        suggestions.into_iter().take(3).collect()
    }