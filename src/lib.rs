
// SIO - Structured Intent Object / Structured Information Object
// Layer between components for Brain-Soul-RAC communication
// 
// Teleport Support: Resource stays. Rights move.

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
    RRP(String),  // Reality Resource Pointer cho Teleport
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
    pub mobility_mode: MobilityMode,  // Copy or Teleport
}

#[derive(Debug, Clone)]
pub struct SioPayload {
    pub data: Vec<u8>,
    pub format: SioFormat,
    pub resource: Option<ResourcePointer>,  // Teleport payload
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
                resource: None,
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
                mobility_mode: MobilityMode::Copy,
            },
        }
    }
    
    /// Tạo SIO với Teleport mode — chỉ chứa pointer, không chứa data
    pub fn teleport(
        kind: SioKind,
        rrp: impl Into<String>,
        rtype: ResourceType,
        size: u64,
    ) -> Self {
        let mut sio = Self::new(kind, "TELEPORT");
        sio.payload.resource = Some(ResourcePointer::new(rrp, rtype, size));
        sio.metadata.mobility_mode = MobilityMode::Teleport;
        sio
    }
    
    /// Validate ARA trước khi teleport
    pub fn can_teleport(&self, action: RightAction) -> bool {
        self.payload
            .resource
            .as_ref()
            .map(|r| r.can_teleport(action))
            .unwrap_or(false)
    }
    
    /// Attach a resource reference (arena, shared memory, GGUF blob, etc.)
    pub fn add_attachment(&mut self, uri: AttachmentUri) {
        self.attachments.push(SioAttachment { uri });
    }

    /// Transfer ownership qua Teleport
    pub fn transfer_ownership(&mut self, new_owner: String) -> Result<(), &'static str> {
        if let Some(ref mut resource) = self.payload.resource {
            resource.transfer_to(new_owner)
        } else {
            Err("NO_RESOURCE_POINTER")
        }
    }
}

// Re-export reality types với Teleport support
pub mod schema;
pub use schema::reality::*;
pub use schema::header::*;
pub use schema::intent::*;
pub use schema::outcome::*;

pub mod builder;
