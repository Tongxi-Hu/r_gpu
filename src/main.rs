use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::app::App;

mod app;
mod common;
mod constant;
mod math;
mod object;
mod physics;
mod render;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app)
}
