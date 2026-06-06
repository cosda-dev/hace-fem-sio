use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioIntent {
    pub objective: String,
    pub actions: Vec<String>,
    pub constraints: Vec<String>,
    pub priority: u32,
}

impl Default for SioIntent {
    fn default() -> Self {
        Self {
            objective: String::new(),
            actions: Vec::new(),
            constraints: Vec::new(),
            priority: 0,
        }
    }
}