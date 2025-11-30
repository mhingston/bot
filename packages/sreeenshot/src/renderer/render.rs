use egui_wgpu::ScreenDescriptor;

/// 将 Egui 输出渲染到 WGPU
#[allow(clippy::too_many_arguments)]
pub fn render_to_wgpu(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    egui_renderer: &mut egui_wgpu::Renderer,
    egui_context: &egui::Context,
    egui_output: &egui::FullOutput,
    width: u32,
    height: u32,
    scale_factor: f64,
) -> anyhow::Result<()> {
    let screen_descriptor =
        ScreenDescriptor { size_in_pixels: [width, height], pixels_per_point: scale_factor as f32 };

    // 将形状转换为绘制任务
    let paint_jobs = egui_context.tessellate(egui_output.shapes.clone(), scale_factor as f32);

    // 更新纹理
    for (id, image_delta) in &egui_output.textures_delta.set {
        egui_renderer.update_texture(device, queue, *id, image_delta);
    }

    // 获取表面纹理
    let output = surface
        .get_current_texture()
        .map_err(|e| anyhow::anyhow!("Failed to get current texture: {:?}", e))?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // 创建命令编码器并渲染
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("egui_render_encoder"),
    });

    // 更新缓冲区
    egui_renderer.update_buffers(device, queue, &mut encoder, &paint_jobs, &screen_descriptor);

    // 执行渲染通道
    // SAFETY: The render pass does not outlive the encoder. We drop the render pass
    // before calling encoder.finish(). The 'static lifetime is required by egui-wgpu's
    // Renderer::render API but the actual usage is safe here.
    {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // SAFETY: We ensure the render_pass is dropped before encoder.finish() is called.
        // The transmute extends the lifetime to 'static which egui-wgpu requires,
        // but we guarantee the render_pass doesn't actually live that long.
        let mut render_pass: wgpu::RenderPass<'static> =
            unsafe { std::mem::transmute(render_pass) };

        egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
    }

    // 提交并呈现
    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
