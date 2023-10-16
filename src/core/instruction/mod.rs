use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryPoint {
    pub url: String,
    pub page_frames: Vec<Frame>,
}
pub const ENTRY_POINT: &str = "^about:srcdoc$";
#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub url_matcher: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Instruction {
    Click { selector: String },
    Wait { timeout: Duration },
}
