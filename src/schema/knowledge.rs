use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SioKnowledge {
    pub refs: Vec<String>,
    pub embeddings: Vec<u8>,
    pub memories: Vec<String>,
}

impl Default for SioKnowledge {
    fn default() -> Self {
        Self {
            refs: Vec::new(),
            embeddings: Vec::new(),
            memories: Vec::new(),
        }
    }
}