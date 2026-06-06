use alloc::string::String;

#[derive(Debug, Clone)]
pub struct SioHeader {
    pub sio_id: String,
    pub session_id: String,
    pub actor_id: String,
    pub soul_id: String,
    pub profile: String,
    pub timestamp: u64,
}

impl SioHeader {
    pub fn new() -> Self {
        Self {
            sio_id: String::from("sio://default"),
            session_id: String::new(),
            actor_id: String::new(),
            soul_id: String::new(),
            profile: String::from("default"),
            timestamp: 0,
        }
    }
}

impl Default for SioHeader {
    fn default() -> Self {
        Self::new()
    }
}