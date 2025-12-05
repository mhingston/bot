//! Tool options panels for screenshot tools
//!
//! Each drawing tool can have an options panel that appears below the main toolbar,
//! allowing users to customize stroke width, color, fill mode, etc.

use egui::{Color32, Pos2, Rect, Ui, Vec2};

// ============================================================================
// Style Constants
// ============================================================================

const PANEL_HEIGHT: f32 = 32.0;
const PANEL_PADDING: f32 = 6.0;
const PANEL_CORNER_RADIUS: f32 = 6.0;
const PANEL_GAP: f32 = 4.0;

const PANEL_BG_COLOR: Color32 = Color32::from_rgba_premultiplied(35, 35, 35, 245);

/// Predefined color palette
pub const COLOR_PALETTE: &[Color32] = &[
    Color32::from_rgb(255, 0, 0),     // Red
    Color32::from_rgb(255, 165, 0),   // Orange
    Color32::from_rgb(255, 255, 0),   // Yellow
    Color32::from_rgb(0, 255, 0),     // Green
    Color32::from_rgb(0, 255, 255),   // Cyan
    Color32::from_rgb(0, 0, 255),     // Blue
    Color32::from_rgb(128, 0, 128),   // Purple
    Color32::from_rgb(255, 192, 203), // Pink
    Color32::WHITE,
    Color32::BLACK,
];

// ============================================================================
// Common Tool Options
// ============================================================================

/// Fill mode for shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillMode {
    /// Stroke only (outline)
    #[default]
    Stroke,
    /// Fill only (solid)
    Fill,
    /// Both stroke and fill
    Both,
}

/// Line style for strokes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

/// Arrow style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowStyle {
    /// Arrow at end only
    #[default]
    Single,
    /// Arrows at both ends
    Double,
    /// No arrow (just a line)
    None,
}

/// Common options shared by many tools
#[derive(Debug, Clone)]
pub struct CommonOptions {
    /// Stroke color
    pub color: Color32,
    /// Stroke width
    pub stroke_width: f32,
    /// Fill mode
    pub fill_mode: FillMode,
    /// Line style
    pub line_style: LineStyle,
}

impl Default for CommonOptions {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            stroke_width: 2.0,
            fill_mode: FillMode::Stroke,
            line_style: LineStyle::Solid,
        }
    }
}

// ============================================================================
// Tool-Specific Options
// ============================================================================

/// Options for rectangle tool
#[derive(Debug, Clone, Default)]
pub struct RectangleOptions {
    pub common: CommonOptions,
}

/// Options for ellipse tool
#[derive(Debug, Clone, Default)]
pub struct EllipseOptions {
    pub common: CommonOptions,
}

/// Options for arrow tool
#[derive(Debug, Clone)]
pub struct ArrowOptions {
    pub common: CommonOptions,
    pub arrow_style: ArrowStyle,
}

impl Default for ArrowOptions {
    fn default() -> Self {
        Self { common: CommonOptions::default(), arrow_style: ArrowStyle::Single }
    }
}

/// Options for annotate (freehand) tool
#[derive(Debug, Clone, Default)]
pub struct AnnotateOptions {
    pub common: CommonOptions,
}

/// Options for highlighter tool
#[derive(Debug, Clone)]
pub struct HighlighterOptions {
    pub color: Color32,
    pub width: f32,
    pub opacity: f32,
}

impl Default for HighlighterOptions {
    fn default() -> Self {
        Self { color: Color32::YELLOW, width: 20.0, opacity: 0.5 }
    }
}

/// Options for mosaic tool
#[derive(Debug, Clone)]
pub struct MosaicOptions {
    /// Block size for mosaic effect
    pub block_size: u32,
    /// Use blur instead of mosaic
    pub use_blur: bool,
    /// Blur strength (if use_blur is true)
    pub blur_strength: f32,
}

impl Default for MosaicOptions {
    fn default() -> Self {
        Self { block_size: 10, use_blur: false, blur_strength: 5.0 }
    }
}

/// Options for text tool
#[derive(Debug, Clone)]
pub struct TextOptions {
    pub color: Color32,
    pub font_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub font_family: String,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            font_size: 16.0,
            bold: false,
            italic: false,
            font_family: "sans-serif".to_string(),
        }
    }
}

/// Options for sequence (numbered markers) tool
#[derive(Debug, Clone)]
pub struct SequenceOptions {
    pub color: Color32,
    pub size: f32,
    pub start_number: u32,
}

impl Default for SequenceOptions {
    fn default() -> Self {
        Self { color: Color32::RED, size: 24.0, start_number: 1 }
    }
}

// ============================================================================
// Options Panel
// ============================================================================

/// The options panel that appears below the main toolbar
pub struct OptionsPanel {
    /// Position (computed from action bar position)
    position: Pos2,
    /// Size
    size: Vec2,
    /// Current tool's options type
    tool_id: String,
    /// Common options (shared state)
    pub common: CommonOptions,
    /// Arrow-specific options
    pub arrow: ArrowOptions,
    /// Highlighter-specific options
    pub highlighter: HighlighterOptions,
    /// Mosaic-specific options
    pub mosaic: MosaicOptions,
    /// Text-specific options
    pub text: TextOptions,
    /// Sequence-specific options
    pub sequence: SequenceOptions,
}

impl OptionsPanel {
    /// Create a new options panel positioned below the action bar
    pub fn new(action_bar_pos: Pos2, action_bar_size: Vec2) -> Self {
        let position =
            Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        let size = Vec2::new(action_bar_size.x, PANEL_HEIGHT + 2.0 * PANEL_PADDING);

        Self {
            position,
            size,
            tool_id: String::new(),
            common: CommonOptions::default(),
            arrow: ArrowOptions::default(),
            highlighter: HighlighterOptions::default(),
            mosaic: MosaicOptions::default(),
            text: TextOptions::default(),
            sequence: SequenceOptions::default(),
        }
    }

    /// Update position based on action bar
    pub fn update_position(&mut self, action_bar_pos: Pos2, action_bar_size: Vec2) {
        self.position =
            Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        self.size.x = action_bar_size.x;
    }

    /// Set the current tool
    pub fn set_tool(&mut self, tool_id: &str) {
        self.tool_id = tool_id.to_string();
    }

    /// Get bounds
    pub fn bounds(&self) -> Rect {
        Rect::from_min_size(self.position, self.size)
    }

    /// Check if point is inside panel
    pub fn contains(&self, pos: Pos2) -> bool {
        self.bounds().contains(pos)
    }

    /// Render the options panel for the current tool using egui widgets
    pub fn render(&mut self, ui: &mut Ui) {
        let rect = self.bounds();

        // Draw background
        ui.painter().rect_filled(rect, PANEL_CORNER_RADIUS, PANEL_BG_COLOR);

        // Create a child UI within the panel bounds for proper widget interaction
        let content_rect = rect.shrink(PANEL_PADDING);
        let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));

        child_ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

            match self.tool_id.as_str() {
                "rectangle" | "ellipse" | "polyline" => {
                    self.render_shape_options_interactive(ui);
                }
                "arrow" => {
                    self.render_arrow_options_interactive(ui);
                }
                "annotate" => {
                    self.render_annotate_options_interactive(ui);
                }
                "highlighter" => {
                    self.render_highlighter_options_interactive(ui);
                }
                "mosaic" | "blur" => {
                    self.render_mosaic_options_interactive(ui);
                }
                "text" => {
                    self.render_text_options_interactive(ui);
                }
                "sequence" => {
                    self.render_sequence_options_interactive(ui);
                }
                _ => {}
            }
        });
    }

    /// Render shape options with interactive widgets
    fn render_shape_options_interactive(&mut self, ui: &mut Ui) {
        // Line style buttons
        self.render_line_style_buttons(ui);
        ui.add_space(8.0);

        // Stroke width slider
        self.render_stroke_width_slider(ui);
        ui.add_space(8.0);

        // Fill mode buttons
        self.render_fill_mode_buttons(ui);
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    /// Render arrow options with interactive widgets
    fn render_arrow_options_interactive(&mut self, ui: &mut Ui) {
        // Arrow style buttons
        self.render_arrow_style_buttons(ui);
        ui.add_space(8.0);

        // Stroke width slider
        self.render_stroke_width_slider(ui);
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    /// Render annotate options with interactive widgets
    fn render_annotate_options_interactive(&mut self, ui: &mut Ui) {
        // Stroke width slider
        self.render_stroke_width_slider(ui);
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    /// Render highlighter options with interactive widgets
    fn render_highlighter_options_interactive(&mut self, ui: &mut Ui) {
        // Width slider for highlighter
        ui.add(
            egui::Slider::new(&mut self.highlighter.width, 5.0..=50.0)
                .show_value(false)
                .custom_formatter(|v, _| format!("{:.0}", v)),
        );
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    /// Render mosaic options with interactive widgets
    fn render_mosaic_options_interactive(&mut self, ui: &mut Ui) {
        // Mosaic/Blur toggle
        if ui.selectable_label(!self.mosaic.use_blur, "Mosaic").clicked() {
            self.mosaic.use_blur = false;
        }
        if ui.selectable_label(self.mosaic.use_blur, "Blur").clicked() {
            self.mosaic.use_blur = true;
        }
        ui.add_space(8.0);

        // Block size slider
        let mut block_size_f = self.mosaic.block_size as f32;
        if ui
            .add(
                egui::Slider::new(&mut block_size_f, 5.0..=30.0)
                    .show_value(false)
                    .custom_formatter(|v, _| format!("{:.0}", v)),
            )
            .changed()
        {
            self.mosaic.block_size = block_size_f as u32;
        }
    }

    /// Render text options with interactive widgets
    fn render_text_options_interactive(&mut self, ui: &mut Ui) {
        // Bold button
        if ui.selectable_label(self.text.bold, "B").clicked() {
            self.text.bold = !self.text.bold;
        }
        // Italic button
        if ui.selectable_label(self.text.italic, "I").clicked() {
            self.text.italic = !self.text.italic;
        }
        ui.add_space(8.0);

        // Font size slider
        ui.add(
            egui::Slider::new(&mut self.text.font_size, 8.0..=72.0)
                .show_value(false)
                .custom_formatter(|v, _| format!("{:.0}", v)),
        );
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    /// Render sequence options with interactive widgets
    fn render_sequence_options_interactive(&mut self, ui: &mut Ui) {
        // Size slider
        ui.add(
            egui::Slider::new(&mut self.sequence.size, 16.0..=48.0)
                .show_value(false)
                .custom_formatter(|v, _| format!("{:.0}", v)),
        );
        ui.add_space(8.0);

        // Color picker
        self.render_color_picker(ui);
    }

    // ========================================================================
    // Interactive widget helpers
    // ========================================================================

    /// Render line style toggle buttons
    fn render_line_style_buttons(&mut self, ui: &mut Ui) {
        let styles = [(LineStyle::Solid, "━"), (LineStyle::Dashed, "┅"), (LineStyle::Dotted, "┈")];

        for (style, icon) in styles {
            let is_selected = self.common.line_style == style;
            if ui.selectable_label(is_selected, icon).clicked() {
                self.common.line_style = style;
            }
        }
    }

    /// Render stroke width slider (1-50)
    fn render_stroke_width_slider(&mut self, ui: &mut Ui) {
        ui.add(
            egui::Slider::new(&mut self.common.stroke_width, 1.0..=50.0)
                .show_value(false)
                .custom_formatter(|v, _| format!("{:.0}", v)),
        );
    }

    /// Render fill mode toggle buttons
    fn render_fill_mode_buttons(&mut self, ui: &mut Ui) {
        let modes = [(FillMode::Stroke, "□"), (FillMode::Fill, "■"), (FillMode::Both, "▣")];

        for (mode, icon) in modes {
            let is_selected = self.common.fill_mode == mode;
            if ui.selectable_label(is_selected, icon).clicked() {
                self.common.fill_mode = mode;
            }
        }
    }

    /// Render arrow style toggle buttons
    fn render_arrow_style_buttons(&mut self, ui: &mut Ui) {
        let styles =
            [(ArrowStyle::Single, "→"), (ArrowStyle::Double, "↔"), (ArrowStyle::None, "—")];

        for (style, icon) in styles {
            let is_selected = self.arrow.arrow_style == style;
            if ui.selectable_label(is_selected, icon).clicked() {
                self.arrow.arrow_style = style;
            }
        }
    }

    /// Render color picker with palette and custom color button
    fn render_color_picker(&mut self, ui: &mut Ui) {
        // Color palette
        for &color in COLOR_PALETTE {
            let is_selected = self.common.color == color;
            let size = if is_selected { 20.0 } else { 16.0 };

            let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());

            // Draw color swatch
            ui.painter().rect_filled(rect, 3.0, color);

            // Draw selection border
            if is_selected {
                ui.painter().rect_stroke(
                    rect,
                    3.0,
                    egui::Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Outside,
                );
            }

            if response.clicked() {
                self.common.color = color;
            }
        }

        ui.add_space(4.0);

        // Custom color picker button
        let picker_response = egui::color_picker::color_edit_button_srgba(
            ui,
            &mut self.common.color,
            egui::color_picker::Alpha::Opaque,
        );
        if picker_response.changed() {
            // Color was changed via picker
        }
    }

    /// Handle click on options panel, returns true if click is inside panel
    /// (egui widgets handle the actual click processing automatically)
    pub fn handle_click(&mut self, pos: Pos2) -> bool {
        // Just return true if click is inside panel - egui handles the rest
        self.contains(pos)
    }
}

impl Default for OptionsPanel {
    fn default() -> Self {
        Self::new(Pos2::ZERO, Vec2::ZERO)
    }
}
