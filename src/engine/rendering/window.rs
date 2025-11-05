use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::CursorGrabMode;
use winit::window::Fullscreen;
use winit::window::{Window, WindowBuilder};


use crate::engine::{CONFIG};



pub fn event_loop_setup() -> (EventLoop<()>, Window) {
    let event_loop: EventLoop<()> = EventLoop::new().expect("Failed to create event loop");
    let window: Window = WindowBuilder::new()
        .with_title("3d Engine")
        //.with_position(LogicalPosition::new(-8, 0))
        .with_inner_size(PhysicalSize::new(
            CONFIG.display.width as u32,
            CONFIG.display.height as u32,
        ))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    if let Err(_e) = window.set_cursor_grab(CursorGrabMode::Confined) {
        // Tries cursor confine mode
        println!("Could not confine cursor: {}", _e);
        if let Err(_e) = window.set_cursor_grab(CursorGrabMode::Locked) {
            // Tries cursor lock mode of if confine failezzd
            println!("Could not lock cursor: {}", _e);
        }
    }
    window.set_cursor_visible(false);

    (event_loop, window)
}
