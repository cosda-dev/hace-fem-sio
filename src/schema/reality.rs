use alloc::string::String;
use alloc::vec::Vec;

/// SIO Reality — Resource binding với Teleport support
/// Triết lý: "Resource stays. Rights move." (Real Estate Motif)

#[derive(Debug, Clone)]
pub struct SioReality {
    pub vision: Vec<u8>,
    pub audio: Vec<u8>,
    pub video: Vec<u8>,
    pub sensor: Vec<u8>,
    pub document: Vec<u8>,
    
    /// Resource Pointer — thay vì chứa data, chứa pointer
    pub resource: Option<ResourcePointer>,
}

/// Resource Pointer — "Sổ đỏ" cho resource bất động
/// Chỉ chứa metadata + rights, không chứa body
#[derive(Debug, Clone)]
pub struct ResourcePointer {
    /// RRP: Reality Resource Pointer (tương tự URI)
    /// Format: RRP://node/storage/path/to/resource
    pub rrp: String,
    
    /// Resource metadata (không di chuyển)
    pub metadata: ResourceMetadata,
    
    /// Rights di chuyển cùng pointer
    pub rights: ResourceRights,
    
    /// Mobility mode
    pub mobility: MobilityMode,
}

#[derive(Debug, Clone)]
pub struct ResourceMetadata {
    /// Resource type
    pub rtype: ResourceType,
    
    /// Size in bytes
    pub size: u64,
    
    /// Storage location (node/disk path)
    pub location: String,
    
    /// Hash for integrity (BLAKE3)
    pub hash: [u8; 32],
    
    /// Creation timestamp
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Dataset,
    Model,
    Artifact,
    Media,
    Knowledge,
    Binary,
}

#[derive(Debug, Clone)]
pub struct ResourceRights {
    /// Owner actor ID
    pub owner: String,
    
    /// Operator actor ID (có thể khác owner)
    pub operator: Option<String>,
    
    /// ARA (Actor Right Actions) được transfer
    pub ara: Vec<RightAction>,
    
    /// Copyright holder
    pub copyright: Option<String>,
    
    /// License terms
    pub license: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightAction {
    Read,
    View,
    Update,
    Delete,
    Execute,
    Trade,
    Buy,
    Sell,
    Transfer,
    Copy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobilityMode {
    /// Traditional: copy toàn bộ data
    Copy,
    
    /// Teleport: chỉ transfer pointer + rights
    Teleport,
    
    /// Hybrid: copy một phần, teleport phần còn lại
    Hybrid,
}

impl ResourcePointer {
    pub fn new(rrp: impl Into<String>, rtype: ResourceType, size: u64) -> Self {
        Self {
            rrp: rrp.into(),
            metadata: ResourceMetadata {
                rtype,
                size,
                location: String::new(),
                hash: [0u8; 32],
                created_at: 0,
            },
            rights: ResourceRights {
                owner: String::new(),
                operator: None,
                ara: Vec::new(),
                copyright: None,
                license: None,
            },
            mobility: MobilityMode::Teleport,
        }
    }
    
    /// Validate ARA before teleport
    pub fn can_teleport(&self, action: RightAction) -> bool {
        self.rights.ara.contains(&action)
    }
    
    /// Transfer ownership
    pub fn transfer_to(&mut self, new_owner: String) -> Result<(), &'static str> {
        if !self.can_teleport(RightAction::Transfer) {
            return Err("ARA_TRANSFER_DENIED");
        }
        self.rights.owner = new_owner;
        Ok(())
    }
}

impl Default for SioReality {
    fn default() -> Self {
        Self {
            vision: Vec::new(),
            audio: Vec::new(),
            video: Vec::new(),
            sensor: Vec::new(),
            document: Vec::new(),
            resource: None,
        }
    }
}