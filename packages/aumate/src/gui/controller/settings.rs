//! Settings feature for controller configuration

use std::sync::{Arc, Mutex};
use std::thread;

use super::context::ControllerContext;
use super::feature::ControllerFeature;
use super::types::TabInfo;
use crate::error::Result;
use crate::gui::content::Content;

/// Set macOS dock icon visibility
#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)] // objc macros check for cargo-clippy feature
fn set_dock_icon_visibility(visible: bool) {
    unsafe {
        use objc::runtime::{Class, Object};
        use objc::{msg_send, sel, sel_impl};

        let ns_app_class = match Class::get("NSApplication") {
            Some(c) => c,
            None => {
                log::error!("Failed to get NSApplication class");
                return;
            }
        };

        let app: *mut Object = msg_send![ns_app_class, sharedApplication];

        // NSApplicationActivationPolicyRegular = 0 (dock icon visible)
        // NSApplicationActivationPolicyAccessory = 1 (dock icon hidden)
        let policy: i64 = if visible { 0 } else { 1 };

        let _: () = msg_send![app, setActivationPolicy: policy];
        log::info!("Set dock icon visibility: {}", visible);
    }
}

/// Settings feature for controller configuration
pub struct SettingsFeature {
    /// Whether a background image load is pending (file picker)
    pending_controller_background: bool,
    /// Async background load storage
    async_background_load: Arc<Mutex<Option<Content>>>,
    /// Whether background is currently loading
    is_loading_background: bool,
    /// Whether to show dock icon (macOS)
    #[cfg(target_os = "macos")]
    show_dock_icon: bool,
}

impl SettingsFeature {
    pub fn new() -> Self {
        Self {
            pending_controller_background: false,
            async_background_load: Arc::new(Mutex::new(None)),
            is_loading_background: false,
            #[cfg(target_os = "macos")]
            show_dock_icon: true,
        }
    }
}

impl Default for SettingsFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for SettingsFeature {
    fn id(&self) -> &'static str {
        "settings"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("settings", "Settings", 100) // High order = last tab
    }

    fn update(&mut self, ctx: &mut ControllerContext) {
        // Check for async-loaded background
        if let Ok(mut pending) = self.async_background_load.try_lock() {
            if let Some(content) = pending.take() {
                // Set the background directly via context
                ctx.set_controller_background(Some(content));
                self.is_loading_background = false;
                log::info!("Background image loaded asynchronously");
                ctx.request_repaint();
            }
        }
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Controller Settings");
        ui.add_space(8.0);

        // Background image section
        ui.group(|ui| {
            ui.label("Background Image:");
            ui.horizontal(|ui| {
                if self.is_loading_background {
                    ui.label("Loading...");
                    ui.spinner();
                } else {
                    // Check if background is set
                    let has_background = ctx.has_controller_background();
                    if has_background {
                        ui.label("Set");
                        if ui.button("Clear").clicked() {
                            ctx.set_controller_background(None);
                        }
                    } else {
                        ui.label("None");
                    }
                }
                if !self.is_loading_background && ui.button("Browse...").clicked() {
                    self.pending_controller_background = true;
                }
            });
        });

        // Handle pending controller background update (file picker)
        if self.pending_controller_background {
            self.pending_controller_background = false;
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
                .pick_file()
            {
                // Load asynchronously to avoid blocking UI
                self.is_loading_background = true;
                let async_load = self.async_background_load.clone();
                let path_clone = path.clone();

                thread::spawn(move || {
                    log::info!("Loading controller background async: {:?}", path_clone);
                    match Content::from_path_sized(&path_clone, 1200, 1200) {
                        Ok(content) => {
                            if let Ok(mut pending) = async_load.lock() {
                                *pending = Some(content);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to load controller background: {}", e);
                        }
                    }
                });
            }
        }

        // macOS: Dock icon visibility setting
        #[cfg(target_os = "macos")]
        {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label("Application:");
                if ui.checkbox(&mut self.show_dock_icon, "Show Dock Icon").changed() {
                    set_dock_icon_visibility(self.show_dock_icon);
                }
            });
        }
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Settings feature initialized");
        Ok(())
    }
}
