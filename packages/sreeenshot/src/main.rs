use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{Key, NamedKey},
};

mod window;
mod renderer;
mod selection;
mod capture;
mod ui;
mod plugins;

use window::create_fullscreen_window;
use renderer::Renderer;
use selection::Selection;
use capture::capture_and_save_to_clipboard;
use ui::Toolbar;
use plugins::{PluginRegistry, PluginContext, PluginResult};
use plugins::{SavePlugin, CopyPlugin, CancelPlugin, AnnotatePlugin};

struct App {
    renderer: Option<Renderer>,
    selection: Selection,
    monitor: Option<xcap::Monitor>,
    screenshot: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    mouse_pos: Vec2,
    should_exit: bool,
    selection_completed: bool,
    toolbar: Option<Toolbar>,
    plugin_registry: PluginRegistry,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Capture screenshot first
        let monitors = match xcap::Monitor::all() {
            Ok(monitors) => monitors,
            Err(e) => {
                eprintln!("Failed to get monitors: {}", e);
                event_loop.exit();
                return;
            }
        };

        let monitor = match monitors
            .into_iter()
            .find(|m| m.is_primary().unwrap_or(false))
        {
            Some(m) => m,
            None => {
                eprintln!("Could not find primary monitor");
                event_loop.exit();
                return;
            }
        };

        let screenshot = match monitor.capture_image() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to capture screen: {}", e);
                event_loop.exit();
                return;
            }
        };

        // Get primary monitor from winit to get correct logical size (DPI-aware)
        let primary_monitor = event_loop
            .primary_monitor()
            .or_else(|| event_loop.available_monitors().next());
        
        let size = if let Some(winit_monitor) = primary_monitor {
            // Use winit's monitor size (logical pixels, DPI-aware)
            winit_monitor.size()
        } else {
            // Fallback: convert physical pixels to logical pixels
            // On macOS Retina displays, DPI scale is typically 2.0
            let width = monitor.width().unwrap_or(1920);
            let height = monitor.height().unwrap_or(1080);
            // Assume 2x scale for Retina, but this is a fallback
            winit::dpi::PhysicalSize::new(width / 2, height / 2)
        };

        let window = match create_fullscreen_window(event_loop, size) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create window: {}", e);
                event_loop.exit();
                return;
            }
        };

        // Choose renderer backend based on environment variable or default to pixels
        let use_pixels = std::env::var("USE_PIXELS")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        
        // Create Rc<Window> first so we can reuse it in fallback
        let window_rc = std::rc::Rc::new(window);
        
        let renderer = if use_pixels {
            match Renderer::new_pixels(window_rc.clone(), screenshot.clone()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create pixels renderer: {}, falling back to softbuffer", e);
                    // Fallback to softbuffer
                    match Renderer::new_softbuffer(window_rc, screenshot.clone()) {
                        Ok(r) => r,
                        Err(e2) => {
                            eprintln!("Failed to create softbuffer renderer: {}", e2);
                            event_loop.exit();
                            return;
                        }
                    }
                }
            }
        } else {
            match Renderer::new_softbuffer(window_rc.clone(), screenshot.clone()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create softbuffer renderer: {}, falling back to pixels", e);
                    // Fallback to pixels
                    match Renderer::new_pixels(window_rc, screenshot.clone()) {
                        Ok(r) => r,
                        Err(e2) => {
                            eprintln!("Failed to create pixels renderer: {}", e2);
                            event_loop.exit();
                            return;
                        }
                    }
                }
            }
        };

        renderer.window().set_visible(true);
        renderer.window().request_redraw();

        self.renderer = Some(renderer);
        self.monitor = Some(monitor);
        self.screenshot = Some(screenshot);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.should_exit {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    // Show selection if active (during dragging) or completed
                    let rect = if self.selection.is_active() || self.selection_completed {
                        self.selection.rect()
                    } else {
                        None
                    };
                    // Only show toolbar and info when selection is completed
                    let toolbar = if self.selection_completed {
                        self.toolbar.as_ref()
                    } else {
                        None
                    };
                    if let Err(e) = renderer.render(rect, toolbar) {
                        eprintln!("Render error: {}", e);
                    }
                }
            }
            WindowEvent::Resized(new_size) => {
                // Handle window resize for pixels renderer
                if let Some(renderer) = &mut self.renderer {
                    if let Renderer::Pixels(pixels_renderer) = renderer {
                        if let Err(e) = pixels_renderer.pixels().resize_surface(new_size.width, new_size.height) {
                            eprintln!("Failed to resize pixels surface: {}", e);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vec2::new(position.x as f32, position.y as f32);
                // Only update selection if not completed (allow dragging during selection)
                if self.selection.is_active() && !self.selection_completed {
                    self.selection.update(self.mouse_pos);
                    if let Some(renderer) = &self.renderer {
                        renderer.window().request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        // Only allow starting new selection if not completed
                        if !self.selection_completed {
                            self.selection.start(self.mouse_pos);
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        // Check if clicking on toolbar button
                        if let Some(toolbar) = &self.toolbar {
                            if let Some(button_id) = toolbar.check_click(self.mouse_pos) {
                                // Execute plugin
                                let context = PluginContext {
                                    selection_coords: self.selection.coords(),
                                    screenshot: self.screenshot.clone(),
                                    monitor: self.monitor.clone(),
                                };
                                
                                let result = self.plugin_registry.execute_plugin(button_id, &context);
                                
                                match result {
                                    PluginResult::Exit => {
                                        self.should_exit = true;
                                        event_loop.exit();
                                    }
                                    PluginResult::Continue => {
                                        // Handle cancel plugin
                                        if button_id == "cancel" {
                                            self.selection.cancel();
                                            self.selection_completed = false;
                                            self.toolbar = None;
                                            if let Some(renderer) = &self.renderer {
                                                renderer.window().request_redraw();
                                            }
                                        }
                                    }
                                    PluginResult::Success => {
                                        // Plugin executed successfully
                                        if let Some(renderer) = &self.renderer {
                                            renderer.window().request_redraw();
                                        }
                                    }
                                    PluginResult::Failure(msg) => {
                                        eprintln!("Plugin error: {}", msg);
                                    }
                                }
                                return;
                            }
                        }
                        
                        if let Some(_coords) = self.selection.finish() {
                            // Selection completed, but don't exit - allow further operations
                            self.selection_completed = true;
                            
                            // Create toolbar when selection is completed
                            if let Some(rect) = self.selection.rect() {
                                // Get screen height for toolbar positioning
                                let screen_height = if let Some(renderer) = &self.renderer {
                                    renderer.window().inner_size().height as f32
                                } else {
                                    1920.0 // Fallback
                                };
                                let plugin_info = self.plugin_registry.get_enabled_plugin_info();
                                self.toolbar = Some(Toolbar::new(rect.0, rect.1, rect.2, rect.3, screen_height, &plugin_info));
                            }
                            
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                    (_, MouseButton::Right) => {
                        self.selection.cancel();
                        self.selection_completed = false;
                        self.toolbar = None;
                        if let Some(renderer) = &self.renderer {
                            renderer.window().request_redraw();
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: key,
                        ..
                    },
                ..
            } => {
                match key {
                    Key::Named(NamedKey::Escape) => {
                        self.should_exit = true;
                        event_loop.exit();
                    }
                    Key::Named(NamedKey::Enter) | Key::Named(NamedKey::Space) => {
                        if let Some(coords) = self.selection.coords() {
                            // Mark selection as completed, but don't exit
                            self.selection_completed = true;
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::CloseRequested => {
                self.should_exit = true;
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize plugin registry and register plugins
    let mut plugin_registry = PluginRegistry::new();
    
    // Register plugins
    plugin_registry.register(Box::new(SavePlugin::new()));
    plugin_registry.register(Box::new(CopyPlugin::new()));
    plugin_registry.register(Box::new(CancelPlugin::new()));
    plugin_registry.register(Box::new(AnnotatePlugin::new()));
    
    // Enable plugins via configuration array
    let enabled_plugins = vec!["save", "copy", "cancel", "annotate"];
    for plugin_id in enabled_plugins {
        plugin_registry.enable(plugin_id);
    }
    
    let mut app = App {
        renderer: None,
        selection: Selection::new(),
        monitor: None,
        screenshot: None,
        mouse_pos: Vec2::ZERO,
        should_exit: false,
        selection_completed: false,
        toolbar: None,
        plugin_registry,
    };

    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
