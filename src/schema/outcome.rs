use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioOutcome {
    pub answer: String,
    pub confidence: f32,
    pub reasoning_hash: String,
    pub evidence: Vec<String>,
    pub recommendations: Vec<String>,
    pub commands: Vec<String>,
}

impl Default for SioOutcome {
    fn default() -> Self {
        Self {
            answer: String::new(),
            confidence: 0.0,
            reasoning_hash: String::new(),
            evidence: Vec::new(),
            recommendations: Vec::new(),
            commands: Vec::new(),
        }
    }
}