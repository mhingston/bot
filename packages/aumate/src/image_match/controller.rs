//! Image Match feature controller for GUI demo

use std::thread;

use egui::TextureHandle;

use super::{ImageMatcher, MatchConfig, MatchResult};
use crate::error::Result;
use crate::gui::controller::{AsyncTask, ControllerContext, ControllerFeature, TabInfo};

/// Image Match demo feature
pub struct ImageMatchFeature {
    /// Template image data
    template_data: Option<Vec<u8>>,
    /// Template texture for preview
    template_texture: Option<TextureHandle>,
    /// Last search results
    results: Vec<MatchResult>,
    /// Status message
    status: String,
    /// Is currently searching
    is_searching: bool,
    /// Configuration
    config: MatchConfig,
    /// Async task for searching
    search_task: Option<AsyncTask<std::result::Result<Vec<MatchResult>, String>>>,
    /// Show advanced options
    show_advanced: bool,
}

impl ImageMatchFeature {
    pub fn new() -> Self {
        Self {
            template_data: None,
            template_texture: None,
            results: Vec::new(),
            status: "Load a template image to start".to_string(),
            is_searching: false,
            config: MatchConfig::default(),
            search_task: None,
            show_advanced: false,
        }
    }

    fn start_search(&mut self) {
        let Some(template_data) = self.template_data.clone() else {
            self.status = "No template loaded".to_string();
            return;
        };

        let config = self.config.clone();
        let task = AsyncTask::new();
        let callback = task.callback();

        self.status = "Searching...".to_string();
        self.is_searching = true;
        self.results.clear();

        thread::spawn(move || {
            let result = (|| {
                let template = image::load_from_memory(&template_data)
                    .map_err(|e| format!("Failed to load template: {}", e))?;
                let screen_capture = crate::screen::capture_screen().map_err(|e| e.to_string())?;
                let screen = image::load_from_memory(&screen_capture.image)
                    .map_err(|e| format!("Failed to decode screen: {}", e))?;

                ImageMatcher::find_all(&screen, &template, &config).map_err(|e| e.to_string())
            })();
            callback(result);
        });

        self.search_task = Some(task);
    }

    fn check_search_task(&mut self, ctx: &mut ControllerContext) {
        if let Some(ref task) = self.search_task {
            if let Some(result) = task.take() {
                self.is_searching = false;
                match result {
                    Ok(results) => {
                        self.status = format!(
                            "Found {} match{}",
                            results.len(),
                            if results.len() == 1 { "" } else { "es" }
                        );
                        self.results = results;
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                        self.results.clear();
                    }
                }
                self.search_task = None;
                ctx.request_repaint();
            }
        }
    }
}

impl Default for ImageMatchFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for ImageMatchFeature {
    fn id(&self) -> &'static str {
        "image-match"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("image-match", "Image Match", 60)
    }

    fn update(&mut self, ctx: &mut ControllerContext) {
        self.check_search_task(ctx);
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Image Template Matching");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                let color = if self.is_searching {
                    egui::Color32::YELLOW
                } else if !self.results.is_empty() {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.label(egui::RichText::new(&self.status).color(color));

                if self.is_searching {
                    ui.spinner();
                }
            });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Template section
        ui.heading("Template");
        ui.add_space(4.0);

        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Load Template...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif"])
                        .pick_file()
                    {
                        match std::fs::read(&path) {
                            Ok(data) => {
                                self.template_data = Some(data);
                                self.template_texture = None;
                                self.results.clear();
                                self.status = format!(
                                    "Template loaded: {}",
                                    path.file_name().unwrap_or_default().to_string_lossy()
                                );
                            }
                            Err(e) => {
                                self.status = format!("Failed to load: {}", e);
                            }
                        }
                    }
                }

                if ui.button("Paste from Clipboard").clicked() {
                    match crate::clipboard::get_image() {
                        Ok(data) => {
                            self.template_data = Some(data);
                            self.template_texture = None;
                            self.results.clear();
                            self.status = "Template pasted from clipboard".to_string();
                        }
                        Err(_) => {
                            self.status = "No image in clipboard".to_string();
                        }
                    }
                }

                if self.template_data.is_some() && ui.button("Clear").clicked() {
                    self.template_data = None;
                    self.template_texture = None;
                    self.results.clear();
                    self.status = "Template cleared".to_string();
                }
            });

            // Template preview
            if let Some(ref data) = self.template_data {
                ui.add_space(8.0);

                // Load texture if needed
                if self.template_texture.is_none() {
                    if let Ok(img) = image::load_from_memory(data) {
                        let rgba = img.to_rgba8();
                        let size = [rgba.width() as usize, rgba.height() as usize];
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
                        let texture = ui.ctx().load_texture(
                            "image_match_template",
                            color_image,
                            egui::TextureOptions::default(),
                        );
                        self.template_texture = Some(texture);
                    }
                }

                if let Some(ref tex) = self.template_texture {
                    ui.horizontal(|ui| {
                        let max_size = 120.0;
                        let aspect = tex.size()[0] as f32 / tex.size()[1] as f32;
                        let (w, h) = if aspect > 1.0 {
                            (max_size, max_size / aspect)
                        } else {
                            (max_size * aspect, max_size)
                        };
                        ui.image((tex.id(), egui::vec2(w, h)));

                        ui.vertical(|ui| {
                            ui.label(format!("Size: {}x{}", tex.size()[0], tex.size()[1]));
                        });
                    });
                }
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Configuration section
        ui.heading("Configuration");
        ui.add_space(4.0);

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Confidence:");
                ui.add(egui::Slider::new(&mut self.config.confidence, 0.5..=1.0).fixed_decimals(2));
            });

            ui.checkbox(&mut self.config.search_multiple_scales, "Search multiple scales");

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_advanced, "Show advanced options");
            });

            if self.show_advanced {
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                ui.checkbox(&mut self.config.use_grayscale, "Use grayscale (faster)");
                ui.checkbox(&mut self.config.parallel, "Parallel processing");

                ui.horizontal(|ui| {
                    ui.label("Max results:");
                    let mut limit = self.config.limit as u32;
                    ui.add(egui::DragValue::new(&mut limit).range(1..=1000));
                    self.config.limit = limit as usize;
                });

                // Scale steps editor
                ui.horizontal(|ui| {
                    ui.label("Scale steps:");
                    let scales_text: String = self
                        .config
                        .scale_steps
                        .iter()
                        .map(|s| format!("{:.1}", s))
                        .collect::<Vec<_>>()
                        .join(", ");
                    ui.label(egui::RichText::new(scales_text).small());
                });
            }
        });

        ui.add_space(16.0);

        // Search button
        let can_search = self.template_data.is_some() && !self.is_searching;
        ui.add_enabled_ui(can_search, |ui| {
            if ui
                .add_sized([ui.available_width(), 32.0], egui::Button::new("Search Screen"))
                .clicked()
            {
                self.start_search();
            }
        });

        if self.template_data.is_none() {
            ui.label(
                egui::RichText::new("Load a template image first")
                    .color(egui::Color32::GRAY)
                    .small(),
            );
        }

        // Results section
        if !self.results.is_empty() {
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);

            ui.heading("Results");
            ui.add_space(4.0);

            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                egui::Grid::new("image_match_results")
                    .num_columns(5)
                    .striped(true)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("#").strong());
                        ui.label(egui::RichText::new("Position").strong());
                        ui.label(egui::RichText::new("Size").strong());
                        ui.label(egui::RichText::new("Confidence").strong());
                        ui.label(egui::RichText::new("Scale").strong());
                        ui.end_row();

                        for (i, r) in self.results.iter().enumerate() {
                            ui.label(format!("{}", i + 1));
                            ui.label(format!("({}, {})", r.x, r.y));
                            ui.label(format!("{}x{}", r.width, r.height));
                            ui.label(format!("{:.1}%", r.confidence * 100.0));
                            ui.label(format!("{:.2}", r.scale));
                            ui.end_row();
                        }
                    });
            });
        }

        // Request repaint if searching
        if self.is_searching {
            ctx.request_repaint();
        }
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Image Match feature initialized");
        Ok(())
    }
}
