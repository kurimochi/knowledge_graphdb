use serde::{Deserialize, Serialize};

pub enum CidVersion {
    V1,
}

// Trait common to all versions
pub trait CidTrait {
    fn version(&self) -> CidVersion;
    fn to_vec(&self) -> Vec<u8>;
}

// CID v1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct CidV1(pub [u8; 32]);
pub const CIDV1_PREFIX: [u8; 2] = [0x00, 0x01]; // CID v1 Prefix: 0x0001

impl CidTrait for CidV1 {
    fn version(&self) -> CidVersion {
        CidVersion::V1
    }
    fn to_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.0.len() + CIDV1_PREFIX.len());
        result.extend_from_slice(&CIDV1_PREFIX);
        result.extend_from_slice(&self.0);
        result
    }
}
