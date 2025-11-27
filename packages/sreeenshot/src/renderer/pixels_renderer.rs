use anyhow::Context as AnyhowContext;
use image::{ImageBuffer, Rgba};
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::window::Window;

use super::RendererTrait;

pub struct PixelsRenderer {
    window: Rc<Window>,
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl PixelsRenderer {
    pub fn pixels(&mut self) -> &mut Pixels<'static> {
        &mut self.pixels
    }
}

impl PixelsRenderer {
    pub fn new(window: Window, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        // Create Rc first
        let window_rc = Rc::new(window);
        
        // Get a reference for SurfaceTexture - we'll use unsafe to extend lifetime
        // This is safe because Rc ensures the window lives as long as we need it
        let window_ref: &Window = &*window_rc;
        let window_static_ref: &'static Window = unsafe {
            std::mem::transmute(window_ref)
        };
        
        let surface_texture = SurfaceTexture::new(width, height, window_static_ref);
        let pixels = Pixels::new(width, height, surface_texture)
            .map_err(|e| anyhow::anyhow!("Failed to create pixels: {:?}", e))?;

        Ok(Self {
            window: window_rc,
            pixels,
            width,
            height,
            screenshot,
        })
    }
}

impl RendererTrait for PixelsRenderer {
    fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
    ) -> anyhow::Result<()> {
        let frame = self.pixels.frame_mut();
        
        // Clear frame with transparent black (80% opacity)
        // Format: RGBA, so we need to set each pixel as [R, G, B, A]
        let overlay_color = [0u8, 0u8, 0u8, 204u8]; // Black with 80% opacity (204/255)
        
        // Fill entire frame with overlay
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&overlay_color);
        }

        // If there's a selection, make that area fully transparent (show original screenshot)
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // Draw the original screenshot in the selection area (fully opaque)
            for py in start_y..end_y.min(self.height) {
                for px in start_x..end_x.min(self.width) {
                    if let Some(pixel) = self.screenshot.get_pixel_checked(px, py) {
                        let idx = ((py * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx] = pixel[0];     // R
                            frame[idx + 1] = pixel[1]; // G
                            frame[idx + 2] = pixel[2]; // B
                            frame[idx + 3] = 255;      // A (fully opaque)
                        }
                    }
                }
            }

            // Draw selection border (white, fully opaque)
            let border_color = [255u8, 255u8, 255u8, 255u8];
            let border_width = 2u32;

            // Top and bottom borders
            for px in start_x.saturating_sub(border_width)..end_x.saturating_add(border_width).min(self.width) {
                for bw in 0..border_width {
                    let top_y = start_y.saturating_sub(bw);
                    let bottom_y = end_y.saturating_add(bw);
                    if top_y < self.height && (px < start_x || px >= end_x) {
                        let idx = ((top_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                    if bottom_y < self.height && (px < start_x || px >= end_x) {
                        let idx = ((bottom_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }

            // Left and right borders
            for py in start_y.saturating_sub(border_width)..end_y.saturating_add(border_width).min(self.height) {
                for bw in 0..border_width {
                    let left_x = start_x.saturating_sub(bw);
                    let right_x = end_x.saturating_add(bw);
                    if left_x < self.width {
                        let idx = ((py * self.width + left_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                    if right_x < self.width {
                        let idx = ((py * self.width + right_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
        }

        self.pixels.render()
            .map_err(|e| anyhow::anyhow!("Failed to render pixels: {:?}", e))?;
        Ok(())
    }
    
    fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

