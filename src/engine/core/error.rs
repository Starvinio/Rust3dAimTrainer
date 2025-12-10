use thiserror::Error;


#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Failed to create event loop: {0}")]
    EventLoop(#[from] winit::error::EventLoopError),

    #[error("Failed to create window: {0}")]
    Window(#[from] winit::error::OsError),

    #[error("Failed to initialize Softbuffer: {0}")]
    Softbuffer(#[from] softbuffer::SoftBufferError),

    #[error("Failed to create audio stream: {0}")]
    Stream(#[from] rodio::stream::StreamError),



}