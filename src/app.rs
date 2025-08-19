use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::renderer::WebGpuContext;

#[derive(Default)]
pub struct App<'w> {
    window: Option<Arc<Window>>,
    web_gpu_context: Option<WebGpuContext<'w>>,
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
            let web_gpu_context = WebGpuContext::new(window.clone());
            self.web_gpu_context = Some(web_gpu_context);
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
                if let (Some(window), Some(web_gpu_context)) =
                    (self.window.as_ref(), self.web_gpu_context.as_mut())
                {
                    web_gpu_context.resize(size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(web_gpu_context) = self.web_gpu_context.as_mut() {
                    web_gpu_context.draw();
                }
            }
            _ => (),
        }
    }
}
