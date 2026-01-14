use std::fmt;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Serialize for CidV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_vec().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CidV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<u8> = Deserialize::deserialize(deserializer)?;

        if vec.len() < 34 {
            return Err(serde::de::Error::custom(
                "CidV1 must be at least 34 bytes (2 prefix + 32 data)",
            ));
        }

        if &vec[0..2] != &CIDV1_PREFIX {
            return Err(serde::de::Error::custom("Invalid CidV1 prefix"));
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&vec[2..34]);
        Ok(CidV1(arr))
    }
}
