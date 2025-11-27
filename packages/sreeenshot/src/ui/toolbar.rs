use glam::Vec2;

pub struct ToolbarButton {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub icon: Option<Vec<u8>>, // PNG icon data
}

pub struct Toolbar {
    pub buttons: Vec<ToolbarButton>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Toolbar {
    pub fn new(selection_x: f32, selection_y: f32, selection_width: f32, selection_height: f32, screen_height: f32, plugin_info: &[crate::plugins::PluginInfo]) -> Self {
        // Toolbar appears below the selection area
        // These sizes will be scaled by DPI in the renderer
        let toolbar_height = 60.0;
        let button_width = 60.0;
        let button_height = 50.0;
        let button_spacing = 12.0;
        
        let mut buttons = Vec::new();
        let mut current_x = 10.0; // Start with padding
        
        // Create buttons from enabled plugins
        for plugin in plugin_info {
            buttons.push(ToolbarButton {
                id: plugin.id.clone(),
                x: current_x,
                y: 5.0, // 5px padding from top
                width: button_width,
                height: button_height,
                icon: plugin.icon.clone(),
            });
            current_x += button_width + button_spacing;
        }
        
        // Calculate toolbar width based on button count
        let toolbar_width = if buttons.is_empty() {
            0.0
        } else {
            current_x + 10.0 // Add right padding
        };
        
        // Position toolbar below selection, centered horizontally
        // If there's not enough space below, position at the bottom of selection
        let toolbar_x = selection_x + (selection_width - toolbar_width) / 2.0;
        
        // Calculate space below selection
        let space_below = screen_height - (selection_y + selection_height);
        let toolbar_y = if space_below >= toolbar_height + 10.0 {
            selection_y + selection_height + 10.0 // 10px gap below selection
        } else {
            selection_y + selection_height - toolbar_height // At bottom of selection
        };
        
        // Adjust button positions relative to toolbar
        for button in &mut buttons {
            button.x += toolbar_x;
            button.y += toolbar_y;
        }
        
        Self {
            buttons,
            x: toolbar_x,
            y: toolbar_y,
            width: toolbar_width,
            height: toolbar_height,
        }
    }
    
    pub fn check_click(&self, mouse_pos: Vec2) -> Option<&str> {
        for button in &self.buttons {
            if mouse_pos.x >= button.x
                && mouse_pos.x <= button.x + button.width
                && mouse_pos.y >= button.y
                && mouse_pos.y <= button.y + button.height
            {
                return Some(&button.id);
            }
        }
        None
    }
}
