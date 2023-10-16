use std::time::Duration;

pub struct EntryPoint {
    pub url: String,
    pub page_frames: Vec<Frame>,
}
pub const ENTRY_POINT: &str = "^about:srcdoc$";

pub struct Frame {
    pub url_matcher: String,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Click {
        selector: String,
    },
    Wait {
        timeout: Duration,
    },
}
