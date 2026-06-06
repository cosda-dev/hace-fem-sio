
// SIO - Structured Intent Object / Structured Information Object
// Layer between components for Brain-Soul-RAC communication

#[derive(Debug, Clone, PartialEq)]
pub enum SioKind {
    Intent,
    Inference,
    Information,
    Command,
    Event,
    Response,
    Memory,
    Artifact,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SioFormat {
    Json,
    Yaml,
    Ail,
    Skb,
    Rcb,
    Gguf,
    Onnx,
    Binary,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachmentUri {
    Arena(String),
    SharedMemory(String),
    HostCall(String),
    Gguf(String),
    Onnx(String),
    Pl(String),
    Hc(String),
}

#[derive(Debug, Clone)]
pub struct SioAttachment {
    pub uri: AttachmentUri,
}

#[derive(Debug, Clone)]
pub struct SioId {
    pub kind: SioKind,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct SioMetadata {
    pub timestamp: u64,
    pub source: String,
    pub target: String,
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub struct SioPayload {
    pub data: Vec<u8>,
    pub format: SioFormat,
}

#[derive(Debug, Clone)]
pub struct SioObject {
    pub id: SioId,
    pub kind: SioKind,
    pub schema: String,
    pub payload: SioPayload,
    pub attachments: Vec<SioAttachment>,
    pub metadata: SioMetadata,
}

impl SioObject {
    pub fn new(kind: SioKind, schema: impl Into<String>) -> Self {
        Self {
            id: SioId {
                kind: kind.clone(),
                id: uuid::Uuid::new_v4().to_string(),
            },
            kind,
            schema: schema.into(),
            payload: SioPayload {
                data: Vec::new(),
                format: SioFormat::Json,
            },
            attachments: Vec::new(),
            metadata: SioMetadata {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                source: "unknown".to_string(),
                target: "unknown".to_string(),
                priority: 0,
            },
        }
    }

    pub fn add_attachment(&mut self, uri: AttachmentUri) {
        self.attachments.push(SioAttachment { uri });
    }
}

