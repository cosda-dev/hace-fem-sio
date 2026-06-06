use alloc::string::String;

#[derive(Debug, Clone)]
pub struct PromptContext {
    pub system: String,
    pub user: String,
    pub history: Vec<String>,
}

impl Default for PromptContext {
    fn default() -> Self {
        Self {
            system: String::new(),
            user: String::new(),
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SioRuntime {
    pub provider: String,
    pub model: String,
    pub quantization: String,
    pub device: String,
    pub execution_mode: String,
    pub prompt: PromptContext,
    pub tokenizer_id: String,
    pub kv_session_id: String,
}

impl Default for SioRuntime {
    fn default() -> Self {
        Self {
            provider: String::from("candle"),
            model: String::new(),
            quantization: String::from("q4_k_m"),
            device: String::from("cpu"),
            execution_mode: String::from("default"),
            prompt: PromptContext::default(),
            tokenizer_id: String::new(),
            kv_session_id: String::new(),
        }
    }
}