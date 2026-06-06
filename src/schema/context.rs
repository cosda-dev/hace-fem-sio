use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioContext {
    pub slots: Vec<String>,
    pub variables: Vec<(String, String)>,
    pub state: String,
}

impl Default for SioContext {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            variables: Vec::new(),
            state: String::new(),
        }
    }
}