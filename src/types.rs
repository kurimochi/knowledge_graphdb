use std::fmt::Display;

// Keccak-256 Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeccakHash([u8; 32]);

impl Display for KeccakHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0));
        Ok(())
    }
}

// DAG Error
#[derive(Debug)]
pub enum DagError {
    MissingDependency(KeccakHash),
}

impl Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DagError::MissingDependency(hash) => {
                write!(f, "Missing dependency with hash: 0x{}", hash)
            }
        }
    }
}

impl std::error::Error for DagError {}
