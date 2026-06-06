use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioRuleset {
    pub policies: Vec<String>,
    pub permissions: Vec<String>,
    pub restrictions: Vec<String>,
}

impl Default for SioRuleset {
    fn default() -> Self {
        Self {
            policies: Vec::new(),
            permissions: Vec::new(),
            restrictions: Vec::new(),
        }
    }
}