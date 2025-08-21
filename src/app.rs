use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::{
    content::{model_object::generate_teapot, world::World},
    math::algebra::matrix::Matrix,
    render::web_gpu::WebGpuContext,
};

const STEP: f32 = 10.0;

#[derive(Default)]
pub struct App<'w> {
    window: Option<Arc<Window>>,
    web_gpu_context: Option<WebGpuContext<'w>>,
    world: Option<World>,
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
            let mut world = World::new(PhysicalSize::<u32> {
                width: 1600,
                height: 1200,
            });
            world.add_geometry(generate_teapot());
            world.init_buffer(
                &self.web_gpu_context.as_ref().unwrap().device,
                &self
                    .web_gpu_context
                    .as_ref()
                    .unwrap()
                    .uniform_bind_group_layout,
            );
            self.world = Some(world);
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
                if let (Some(window), Some(world), Some(web_gpu_context)) = (
                    self.window.as_ref(),
                    self.world.as_mut(),
                    self.web_gpu_context.as_mut(),
                ) {
                    web_gpu_context.resize(size);
                    world.resize(size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(world), Some(web_gpu_context)) =
                    (self.world.as_mut(), self.web_gpu_context.as_mut())
                {
                    world.update_buffer(&web_gpu_context.queue);
                    web_gpu_context.draw(world);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let (MouseScrollDelta::PixelDelta(v), Some(world), Some(window)) =
                    (delta, self.world.as_mut(), self.window.as_mut())
                {
                    world.move_obj(Matrix::<4>::translation(v.x as f32, -v.y as f32, 0.0));
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                if let (Some(window), Some(world)) = (self.window.as_ref(), self.world.as_mut()) {
                    match (code, state) {
                        (KeyCode::KeyW, ElementState::Released) => {
                            world.rotate_obj(Matrix::<4>::rotate_x(-STEP));
                            window.request_redraw();
                        }
                        (KeyCode::KeyS, ElementState::Released) => {
                            world.rotate_obj(Matrix::<4>::rotate_x(STEP));
                            window.request_redraw();
                        }
                        (KeyCode::KeyA, ElementState::Released) => {
                            world.rotate_obj(Matrix::<4>::rotate_y(-STEP));
                            window.request_redraw();
                        }
                        (KeyCode::KeyD, ElementState::Released) => {
                            world.rotate_obj(Matrix::<4>::rotate_y(STEP));
                            window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        }
    }
}
