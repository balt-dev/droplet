//! Holds the structures of the main context of the engine.

use std::process::ExitCode;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

/// Holds data pertaining to the main data of the engine.
///
/// This will be passed around a lot.
pub struct Context {

}


/// A struct used to aid in building a context.
pub struct AppBuilder {
    /// The position to start the game window at. See [`winit::dpi::Position`].
    pub position: mint::Point2<f64>,
    /// The size to start the game window with. See [`winit::dpi::Size`].
    pub size: mint::Vector2<f64>
}


impl AppBuilder {
    /// Builds the app, starting up the engine and blocking.
    pub fn run(self) {
        let winit_event_loop = EventLoop::new().expect("Failed to start winit event loop");

        winit_event_loop.set_control_flow(ControlFlow::Poll);

        let window_builder = WindowBuilder::new()
            .build(&winit_event_loop)
            .expect("Failed to build window");
    }
}
