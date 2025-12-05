//! Floating windows feature for creating and managing effect windows

use std::path::PathBuf;

use super::context::ControllerContext;
use super::feature::ControllerFeature;
use super::types::TabInfo;
use crate::error::Result;
use crate::gui::content::Content;
use crate::gui::effect::{PresetEffect, PresetEffectOptions};
use crate::gui::shape::WindowShape;
use crate::gui::window::{Position, Size, WindowCommand, WindowConfig, WindowLevel};

/// All available window shapes
const ALL_SHAPES: &[(&str, WindowShape)] =
    &[("Circle", WindowShape::Circle), ("Rectangle", WindowShape::Rectangle)];

/// All available preset effects
const ALL_EFFECTS: &[PresetEffect] = &[
    PresetEffect::RotatingHalo,
    PresetEffect::PulseRipple,
    PresetEffect::FlowingLight,
    PresetEffect::StardustScatter,
    PresetEffect::ElectricSpark,
    PresetEffect::SmokeWisp,
    PresetEffect::RainDrop,
    PresetEffect::LaserBeam,
    PresetEffect::LightningArc,
    PresetEffect::MeteorShower,
    PresetEffect::SonarPulse,
    PresetEffect::MatrixRain,
    PresetEffect::AuroraWave,
    PresetEffect::OrbitRings,
    PresetEffect::HeartbeatPulse,
    PresetEffect::CosmicStrings,
    PresetEffect::SilkRibbon,
];

/// Floating windows feature for creating and managing effect windows
pub struct FloatingWindowsFeature {
    /// Currently selected effect
    selected_effect: PresetEffect,
    /// Effect options
    effect_options: PresetEffectOptions,
    /// Currently selected shape
    selected_shape: WindowShape,
    /// New window size
    new_window_size: u32,
    /// New window X position
    new_window_x: f32,
    /// New window Y position
    new_window_y: f32,
    /// Image path for window content
    flow_window_image_path: Option<PathBuf>,
    /// Pending image update for a window
    pending_image_update_window: Option<(winit::window::WindowId, (u32, u32))>,
}

impl FloatingWindowsFeature {
    pub fn new() -> Self {
        Self {
            selected_effect: PresetEffect::SilkRibbon,
            effect_options: PresetEffectOptions::default(),
            selected_shape: WindowShape::Circle,
            new_window_size: 50,
            new_window_x: 500.0,
            new_window_y: 300.0,
            flow_window_image_path: None,
            pending_image_update_window: None,
        }
    }

    /// Open file picker for flow window image
    fn open_image_picker_for_flow_window(&mut self) {
        if let Some(path) =
            rfd::FileDialog::new().add_filter("Images", &["png", "svg", "jpg", "jpeg"]).pick_file()
        {
            self.flow_window_image_path = Some(path);
        }
    }

    /// Create a new floating window
    fn create_window(&mut self, ctx: &mut ControllerContext) {
        // Load image content if path is selected
        let content = if let Some(path) = &self.flow_window_image_path {
            match Content::from_path_sized(path, self.new_window_size, self.new_window_size) {
                Ok(content) => Some(content),
                Err(e) => {
                    log::error!("Failed to load image: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let config = WindowConfig {
            id: None,
            title: Some(ctx.registry.generate_name()),
            position: Position::new(self.new_window_x as f64, self.new_window_y as f64),
            size: Size::new(self.new_window_size, self.new_window_size),
            effect_margin: 0.0,
            shape: self.selected_shape.clone(),
            draggable: true,
            resizable: false,
            click_through: false,
            level: WindowLevel::AlwaysOnTop,
            opacity: 1.0,
            icon: None,
            content,
            widget_content: None,
            effect: None,
            show_animation: None,
            hide_animation: None,
        };

        let _ = ctx.command_sender.send(WindowCommand::Create {
            config,
            effect: Some((self.selected_effect, self.effect_options.clone())),
        });

        // Move position for next window
        self.new_window_x += 60.0;
        if self.new_window_x > 1000.0 {
            self.new_window_x = 500.0;
            self.new_window_y += 60.0;
        }
    }

    /// Render the create section
    fn render_create_section(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Create New Window");
        ui.add_space(8.0);

        // Effect selection
        ui.horizontal(|ui| {
            ui.label("Effect:");
            egui::ComboBox::from_id_salt("effect_selector")
                .selected_text(format!("{:?}", self.selected_effect))
                .show_ui(ui, |ui| {
                    for effect in ALL_EFFECTS {
                        ui.selectable_value(
                            &mut self.selected_effect,
                            *effect,
                            format!("{:?}", effect),
                        );
                    }
                });
        });

        ui.add_space(4.0);

        // Effect options based on selected effect
        ui.collapsing("Effect Options", |ui| {
            self.render_effect_options(ui);
        });

        ui.add_space(4.0);

        // Shape selection
        ui.horizontal(|ui| {
            ui.label("Shape:");
            egui::ComboBox::from_id_salt("shape_selector")
                .selected_text(format!("{:?}", self.selected_shape))
                .show_ui(ui, |ui| {
                    for (name, shape) in ALL_SHAPES {
                        ui.selectable_value(&mut self.selected_shape, shape.clone(), *name);
                    }
                });
        });

        ui.add_space(4.0);

        // Image Content Section
        ui.horizontal(|ui| {
            ui.label("Image:");
            if let Some(path) = &self.flow_window_image_path {
                let filename = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                ui.label(&filename);
            } else {
                ui.label("None");
            }
            if ui.button("Browse...").clicked() {
                self.open_image_picker_for_flow_window();
            }
            if self.flow_window_image_path.is_some() && ui.button("Clear").clicked() {
                self.flow_window_image_path = None;
            }
        });

        ui.add_space(4.0);

        // Window size
        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut self.new_window_size, 30..=200).suffix("px"));
        });

        // Position
        ui.horizontal(|ui| {
            ui.label("Position X:");
            ui.add(egui::DragValue::new(&mut self.new_window_x).range(0.0..=2000.0));
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut self.new_window_y).range(0.0..=2000.0));
        });

        ui.add_space(8.0);

        // Create button
        if ui.button("Create Window").clicked() {
            self.create_window(ctx);
        }
    }

    /// Render effect-specific options
    fn render_effect_options(&mut self, ui: &mut egui::Ui) {
        // Common options
        ui.horizontal(|ui| {
            ui.label("Intensity:");
            ui.add(egui::Slider::new(&mut self.effect_options.intensity, 0.0..=1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.effect_options.speed, 0.1..=3.0));
        });

        // Effect-specific options
        match self.selected_effect {
            PresetEffect::SilkRibbon => {
                ui.horizontal(|ui| {
                    ui.label("Ribbon Count:");
                    let mut count = self.effect_options.ribbon_count as i32;
                    if ui.add(egui::Slider::new(&mut count, 1..=6)).changed() {
                        self.effect_options.ribbon_count = count as usize;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Petal Amplitude:");
                    ui.add(egui::Slider::new(&mut self.effect_options.petal_amplitude, 5.0..=50.0));
                });
            }
            _ => {
                // Generic options for other effects
                ui.horizontal(|ui| {
                    ui.label("Edge Width:");
                    ui.add(egui::Slider::new(&mut self.effect_options.edge_width, 5.0..=30.0));
                });
            }
        }

        // Color presets
        ui.horizontal(|ui| {
            ui.label("Colors:");
            if ui.button("Cyan/Purple").clicked() {
                self.effect_options.particle_colors =
                    vec![[0.4, 0.8, 1.0, 1.0], [0.8, 0.4, 1.0, 1.0]];
            }
            if ui.button("Fire").clicked() {
                self.effect_options.particle_colors =
                    vec![[1.0, 0.3, 0.0, 1.0], [1.0, 0.6, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0]];
            }
            if ui.button("Rainbow").clicked() {
                self.effect_options.particle_colors = vec![
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 0.5, 0.0, 1.0],
                    [1.0, 1.0, 0.0, 1.0],
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.5, 1.0, 1.0],
                    [0.5, 0.0, 1.0, 1.0],
                ];
            }
        });
    }

    /// Render the manage existing windows section
    fn render_manage_section(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Managed Windows");
        ui.add_space(8.0);

        let windows = ctx.registry.list();

        if windows.is_empty() {
            ui.label("No windows created yet.");
        } else {
            // Table of windows
            egui::Grid::new("window_grid").num_columns(3).spacing([20.0, 4.0]).striped(true).show(
                ui,
                |ui| {
                    ui.label("Name");
                    ui.label("Effect");
                    ui.label("Actions");
                    ui.end_row();

                    for window in &windows {
                        ui.label(&window.name);
                        ui.label(format!(
                            "{:?}",
                            window.effect.unwrap_or(PresetEffect::RotatingHalo)
                        ));

                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                let _ =
                                    ctx.command_sender.send(WindowCommand::Close { id: window.id });
                            }
                            if ui.button("Set Image").clicked() {
                                self.pending_image_update_window = Some((window.id, window.size));
                            }
                        });
                        ui.end_row();
                    }
                },
            );
        }

        // Handle pending image update (file picker)
        if let Some((window_id, size)) = self.pending_image_update_window.take() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
                .pick_file()
            {
                // Load the image content with the window's size
                match Content::from_path_sized(&path, size.0, size.1) {
                    Ok(content) => {
                        let _ = ctx.command_sender.send(WindowCommand::UpdateContent {
                            id: window_id,
                            content: Some(content),
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to load image: {}", e);
                    }
                }
            }
        }
    }
}

impl Default for FloatingWindowsFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for FloatingWindowsFeature {
    fn id(&self) -> &'static str {
        "floating_windows"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("floating_windows", "Floating Windows", 0) // First tab
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        self.render_create_section(ui, ctx);
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        self.render_manage_section(ui, ctx);
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Floating windows feature initialized");
        Ok(())
    }
}
