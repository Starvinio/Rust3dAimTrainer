use supports_color::Stream;
use crate::engine::{BLUE, RED, RESET};

pub struct Colors {
    pub blue: &'static str,
    pub red: &'static str,
    pub reset: &'static str,
}
impl Colors {
    pub fn detect_set_colors() -> Self {
        match supports_color::on(Stream::Stdout) {
            Some(lvl) if lvl.has_basic => {
                Self { blue: BLUE, red:RED, reset: RESET,  }
            },
            _ => Self { blue:"", red:"", reset:"" }
        }
    }
}