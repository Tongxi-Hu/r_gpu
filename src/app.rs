use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::wgpu_ctx::WgpuCtx;

#[derive(Default)]
pub struct App<'w> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'w>>,
}

impl<'w> ApplicationHandler for App<'w> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes();
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("error create window"),
            );
            let wgpu_ctx = WgpuCtx::new(window.clone());
            self.wgpu_ctx = Some(wgpu_ctx);
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let (Some(window), Some(wgpu_ctx)) =
                    (self.window.as_ref(), self.wgpu_ctx.as_mut())
                {
                    wgpu_ctx.resize(size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
                    wgpu_ctx.draw();
                }
            }
            _ => (),
        }
    }
}
