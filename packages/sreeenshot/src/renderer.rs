use anyhow::Context as AnyhowContext;
use image::{ImageBuffer, Rgba};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::window::Window;

pub struct Renderer {
    window: Rc<Window>,
    context: Context<Rc<Window>>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Renderer {
    pub fn new(window: Window, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        let window = Rc::new(window);
        let context = unsafe { Context::new(window.clone()) }
            .map_err(|e| anyhow::anyhow!("Failed to create graphics context: {:?}", e))?;
        let surface = unsafe { Surface::new(&context, window.clone()) }
            .map_err(|e| anyhow::anyhow!("Failed to create surface: {:?}", e))?;

        Ok(Self {
            window,
            context,
            surface,
            width,
            height,
            screenshot,
        })
    }

    pub fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
    ) -> anyhow::Result<()> {
        let width = NonZeroU32::new(self.width)
            .ok_or_else(|| anyhow::anyhow!("Width must be non-zero"))?;
        let height = NonZeroU32::new(self.height)
            .ok_or_else(|| anyhow::anyhow!("Height must be non-zero"))?;
        
        self.surface.resize(width, height)
            .map_err(|e| anyhow::anyhow!("Failed to resize surface: {:?}", e))?;
        
        let mut buffer = self.surface.buffer_mut()
            .map_err(|e| anyhow::anyhow!("Failed to get buffer: {:?}", e))?;
        
        let mut pixels = vec![0u32; (self.width * self.height) as usize];

        // First, draw the entire screenshot
        for py in 0..self.height {
            for px in 0..self.width {
                if let Some(pixel) = self.screenshot.get_pixel_checked(px, py) {
                    let r = pixel[0] as u32;
                    let g = pixel[1] as u32;
                    let b = pixel[2] as u32;
                    let a = pixel[3] as u32;
                    let rgba = (a << 24) | (r << 16) | (g << 8) | b;
                    pixels[(py * self.width + px) as usize] = rgba;
                }
            }
        }

        // If there's a selection, apply 80% black overlay to non-selected areas
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // Apply 80% black overlay to areas outside selection
            // This means we darken the pixels by 80%
            for py in 0..self.height {
                for px in 0..self.width {
                    let is_in_selection = px >= start_x && px < end_x && py >= start_y && py < end_y;
                    if !is_in_selection {
                        // Get current pixel
                        let pixel = pixels[(py * self.width + px) as usize];
                        // Extract RGB components (ARGB format)
                        let a = (pixel >> 24) & 0xFF;
                        let r = (pixel >> 16) & 0xFF;
                        let g = (pixel >> 8) & 0xFF;
                        let b = pixel & 0xFF;
                        // Darken by 80% (multiply by 0.2)
                        let new_r = ((r as f32) * 0.2) as u32;
                        let new_g = ((g as f32) * 0.2) as u32;
                        let new_b = ((b as f32) * 0.2) as u32;
                        // Reconstruct pixel
                        pixels[(py * self.width + px) as usize] = (a << 24) | (new_r << 16) | (new_g << 8) | new_b;
                    }
                }
            }

            // Draw selection border
            let border_color = 0xFFFFFFFFu32; // White border
            let border_width = 2u32;

            // Top and bottom borders
            for px in start_x.saturating_sub(border_width)..end_x.saturating_add(border_width).min(self.width) {
                for bw in 0..border_width {
                    let top_y = start_y.saturating_sub(bw);
                    let bottom_y = end_y.saturating_add(bw);
                    if top_y < self.height && (px < start_x || px >= end_x) {
                        pixels[(top_y * self.width + px) as usize] = border_color;
                    }
                    if bottom_y < self.height && (px < start_x || px >= end_x) {
                        pixels[(bottom_y * self.width + px) as usize] = border_color;
                    }
                }
            }

            // Left and right borders
            for py in start_y.saturating_sub(border_width)..end_y.saturating_add(border_width).min(self.height) {
                for bw in 0..border_width {
                    let left_x = start_x.saturating_sub(bw);
                    let right_x = end_x.saturating_add(bw);
                    if left_x < self.width {
                        pixels[(py * self.width + left_x) as usize] = border_color;
                    }
                    if right_x < self.width {
                        pixels[(py * self.width + right_x) as usize] = border_color;
                    }
                }
            }
        }

        // Copy pixels to buffer
        for (i, pixel) in pixels.iter().enumerate() {
            buffer[i] = *pixel;
        }
        
        buffer.present()
            .map_err(|e| anyhow::anyhow!("Failed to present buffer: {:?}", e))?;
        Ok(())
    }
    
    pub fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

