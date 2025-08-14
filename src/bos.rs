use std::{fmt,vec::Vec};

impl fmt::Debug for PlatformCapabilityDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PlatformCapabilityDescriptor").field("uuid", &self.uuid)
                                             .field("vendor_code", &self.vendor_code).finish()
    }
}

/// Represents a Microsoft OS 2.0 Platform Capability Descriptor
#[derive(Clone)]
pub struct PlatformCapabilityDescriptor {
    /// OS version UUID
    pub uuid: [u8; 16],
    /// Vendor code
    pub vendor_code: u8,
    /// Reserved
    pub reserved: u8,
}

/// Represents a BOS descriptor containing platform capabilities
#[derive(Clone)]
pub struct BosDescriptor {
    /// BOS capabilities
    pub capabilities: Vec<PlatformCapabilityDescriptor>,
} 

impl fmt::Debug for BosDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BosDescriptor").field("capabilities", &self.capabilities).finish()
    }
}

impl BosDescriptor {
    /// Serializes the BOS descriptor into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let total_length = 5 + self.capabilities.len() * 0x1C;
        let mut desc_bytes = vec![
            0x05, // length of BOS descriptor
            0x0F, // Descriptor type (BOS)
            (total_length & 0xFF) as u8,
            (total_length >> 8) as u8,
            self.capabilities.len() as u8,
        ];

        for cap in &self.capabilities {
            desc_bytes.extend_from_slice(&[
                0x1C, // Length of capability descriptor
                0x10, // Devic compatibility type
                0x05, // Platform compatibility type
            ]);
            desc_bytes.extend_from_slice(&cap.uuid);
            desc_bytes.push(cap.vendor_code);
            desc_bytes.push(cap.reserved);
        }
        desc_bytes
    }
}