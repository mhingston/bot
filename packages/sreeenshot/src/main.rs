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

use window::create_fullscreen_window;
use renderer::Renderer;
use selection::Selection;
use capture::capture_and_save_to_clipboard;

struct App {
    renderer: Option<Renderer>,
    selection: Selection,
    monitor: Option<xcap::Monitor>,
    screenshot: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    mouse_pos: Vec2,
    should_exit: bool,
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

        let renderer = match Renderer::new(window, screenshot.clone()) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to create renderer: {}", e);
                event_loop.exit();
                return;
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
                    let rect = if self.selection.is_active() {
                        self.selection.rect()
                    } else {
                        None
                    };
                    if let Err(e) = renderer.render(rect) {
                        eprintln!("Render error: {}", e);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vec2::new(position.x as f32, position.y as f32);
                if self.selection.is_active() {
                    self.selection.update(self.mouse_pos);
                    if let Some(renderer) = &self.renderer {
                        renderer.window().request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        self.selection.start(self.mouse_pos);
                        if let Some(renderer) = &self.renderer {
                            renderer.window().request_redraw();
                        }
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        if let Some(coords) = self.selection.finish() {
                            if let Some(monitor) = &self.monitor {
                                match capture_and_save_to_clipboard(monitor, coords) {
                                    Ok(_) => {
                                        // Successfully saved to clipboard, exit
                                        self.should_exit = true;
                                        event_loop.exit();
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to save to clipboard: {}", e);
                                        event_loop.exit();
                                    }
                                }
                            }
                        }
                    }
                    (_, MouseButton::Right) => {
                        self.selection.cancel();
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
                            if let Some(monitor) = &self.monitor {
                                match capture_and_save_to_clipboard(monitor, coords) {
                                    Ok(_) => {
                                        self.should_exit = true;
                                        event_loop.exit();
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to save to clipboard: {}", e);
                                        event_loop.exit();
                                    }
                                }
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
    let mut app = App {
        renderer: None,
        selection: Selection::new(),
        monitor: None,
        screenshot: None,
        mouse_pos: Vec2::ZERO,
        should_exit: false,
    };

    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
