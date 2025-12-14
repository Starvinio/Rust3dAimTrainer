use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::CursorGrabMode;
use winit::window::Fullscreen;
use winit::window::{Window, WindowBuilder};

use crate::engine::{CONFIG, EngineError};


pub fn event_loop_setup() -> Result<(EventLoop<()>, Window), EngineError> {

    let event_loop= EventLoop::new()?;
    
    let window = WindowBuilder::new()
        .with_title("Rust3dAimTrainer")
        .with_inner_size(PhysicalSize::new(
            CONFIG.display.width as u32,
            CONFIG.display.height as u32,
        ))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        ?;



    let _ = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    Ok((event_loop, window))
}
