

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Failed to create event loop: {0}")]
    EventLoop(#[from] winit::error::EventLoopError),

    #[error("Failed to create window: {0}")]
    Window(#[from] winit::error::OsError),

    #[error("Failed to initialize Softbuffer: {0}")]
    Softbuffer(#[from] softbuffer::SoftBufferError),

    #[error("Failed to create audio stream: {0}")]
    Stream(#[from] rodio::stream::StreamError),

    #[error("ANSI compatibility check failed. Please run in a terminal.")]
    ColorCheck
}