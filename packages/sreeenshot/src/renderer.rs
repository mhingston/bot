use anyhow::Context as AnyhowContext;
use image::ImageBuffer;
use image::Rgba;
use std::rc::Rc;
use winit::window::Window;

mod softbuffer_renderer;
mod pixels_renderer;

pub use softbuffer_renderer::SoftbufferRenderer;
pub use pixels_renderer::PixelsRenderer;

pub trait RendererTrait {
    fn render(&mut self, selection: Option<(f32, f32, f32, f32)>, toolbar: Option<&crate::ui::Toolbar>) -> anyhow::Result<()>;
    fn window(&self) -> &Rc<Window>;
}

pub enum Renderer {
    Softbuffer(SoftbufferRenderer),
    Pixels(PixelsRenderer),
}

impl Renderer {
    pub fn new_softbuffer(
        window: Rc<Window>,
        screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> anyhow::Result<Self> {
        Ok(Self::Softbuffer(SoftbufferRenderer::new(window, screenshot)?))
    }

    pub fn new_pixels(
        window: Rc<Window>,
        screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> anyhow::Result<Self> {
        Ok(Self::Pixels(PixelsRenderer::new(window, screenshot)?))
    }
}

impl RendererTrait for Renderer {
    fn render(&mut self, selection: Option<(f32, f32, f32, f32)>, toolbar: Option<&crate::ui::Toolbar>) -> anyhow::Result<()> {
        match self {
            Self::Softbuffer(r) => r.render(selection, toolbar),
            Self::Pixels(r) => r.render(selection, toolbar),
        }
    }

    fn window(&self) -> &Rc<Window> {
        match self {
            Self::Softbuffer(r) => r.window(),
            Self::Pixels(r) => r.window(),
        }
    }
}

impl Renderer {
    pub fn render(&mut self, selection: Option<(f32, f32, f32, f32)>, toolbar: Option<&crate::ui::Toolbar>) -> anyhow::Result<()> {
        RendererTrait::render(self, selection, toolbar)
    }

    pub fn window(&self) -> &Rc<Window> {
        RendererTrait::window(self)
    }
}
