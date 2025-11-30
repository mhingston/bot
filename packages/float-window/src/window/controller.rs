//! Controller window UI for managing flow windows

use super::commands::{CommandSender, WindowCommand, WindowRegistry};
use super::config::{Position, Size, WindowConfig};
use crate::effect::{PresetEffect, PresetEffectOptions};
use crate::shape::WindowShape;
use egui::{Context, Ui};

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

/// Controller state for managing windows
pub struct ControllerState {
    /// Command sender to communicate with the event loop
    command_sender: CommandSender,
    /// Registry of managed windows
    registry: WindowRegistry,
    /// Selected effect for new window
    selected_effect: PresetEffect,
    /// Effect options for new window
    effect_options: PresetEffectOptions,
    /// Window size for new window
    new_window_size: u32,
    /// Position X for new window
    new_window_x: f32,
    /// Position Y for new window
    new_window_y: f32,
}

impl ControllerState {
    /// Create a new controller state
    pub fn new(command_sender: CommandSender, registry: WindowRegistry) -> Self {
        Self {
            command_sender,
            registry,
            selected_effect: PresetEffect::SilkRibbon,
            effect_options: PresetEffectOptions::default(),
            new_window_size: 50,
            new_window_x: 500.0,
            new_window_y: 300.0,
        }
    }

    /// Get the window registry
    pub fn registry(&self) -> &WindowRegistry {
        &self.registry
    }

    /// Render the controller UI
    pub fn render(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flow Window Controller");
            ui.separator();

            // Create new window section
            self.render_create_section(ui);

            ui.separator();

            // Manage existing windows section
            self.render_manage_section(ui);
        });
    }

    /// Render the create new window section
    fn render_create_section(&mut self, ui: &mut Ui) {
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
            self.create_window();
        }
    }

    /// Render effect-specific options
    fn render_effect_options(&mut self, ui: &mut Ui) {
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
                self.effect_options.particle_colors = vec![
                    [0.4, 0.8, 1.0, 1.0],
                    [0.8, 0.4, 1.0, 1.0],
                ];
            }
            if ui.button("Fire").clicked() {
                self.effect_options.particle_colors = vec![
                    [1.0, 0.3, 0.0, 1.0],
                    [1.0, 0.6, 0.0, 1.0],
                    [1.0, 0.9, 0.2, 1.0],
                ];
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
    fn render_manage_section(&mut self, ui: &mut Ui) {
        ui.heading("Managed Windows");
        ui.add_space(8.0);

        let windows = self.registry.list();

        if windows.is_empty() {
            ui.label("No windows created yet.");
        } else {
            // Table of windows
            egui::Grid::new("window_grid")
                .num_columns(3)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Effect");
                    ui.label("Actions");
                    ui.end_row();

                    for window in &windows {
                        ui.label(&window.name);
                        ui.label(format!("{:?}", window.effect.unwrap_or(PresetEffect::RotatingHalo)));

                        if ui.button("Close").clicked() {
                            let _ = self.command_sender.send(WindowCommand::Close { id: window.id });
                        }
                        ui.end_row();
                    }
                });
        }

        ui.add_space(8.0);

        // Close all button
        if !windows.is_empty() && ui.button("Close All Windows").clicked() {
            let _ = self.command_sender.send(WindowCommand::CloseAll);
        }
    }

    /// Create a new window with current settings
    fn create_window(&mut self) {
        let config = WindowConfig {
            id: None,
            title: Some(self.registry.generate_name()),
            position: Position::new(self.new_window_x as f64, self.new_window_y as f64),
            size: Size::new(self.new_window_size, self.new_window_size),
            effect_margin: 0.0, // Will be calculated by builder
            shape: WindowShape::Circle,
            draggable: true,
            resizable: false,
            click_through: false,
            level: super::config::WindowLevel::AlwaysOnTop,
            opacity: 1.0,
            icon: None,
            content: None,
            effect: None, // Effect is passed separately
            show_animation: None,
            hide_animation: None,
        };

        let _ = self.command_sender.send(WindowCommand::Create {
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
}
