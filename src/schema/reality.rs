use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioReality {
    pub vision: Vec<u8>,
    pub audio: Vec<u8>,
    pub video: Vec<u8>,
    pub sensor: Vec<u8>,
    pub document: Vec<u8>,
}

impl Default for SioReality {
    fn default() -> Self {
        Self {
            vision: Vec::new(),
            audio: Vec::new(),
            video: Vec::new(),
            sensor: Vec::new(),
            document: Vec::new(),
        }
    }
}