#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Failed to create event loop: {0}")]
    EventLoopErr(#[from] winit::error::EventLoopError),

    #[error("Failed to create window: {0}")]
    WindowErr(#[from] winit::error::OsError),

    #[error("Failed to initialize Softbuffer: {0}")]
    SoftbufferErr(#[from] softbuffer::SoftBufferError),

    #[error("Failed to create audio stream: {0}")]
    StreamErr(#[from] rodio::stream::StreamError),

    #[error("Failed to decode PNG: {0}")]
    PngDecodeErr(#[from] png::DecodingError),

    #[error("ANSI compatibility check failed. Please run in a terminal.")]
    ColorCheckErr,

    #[error("Failed to load GUI")]
    GUILoadErr,

    #[error("Failed to load Sound Effects")]
    SFXErr,

}