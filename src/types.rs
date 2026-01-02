#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InternalId(u32);
pub type KeccakHash = [u8; 32];

impl From<u32> for InternalId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl TryFrom<usize> for InternalId {
    type Error = <u32 as TryFrom<usize>>::Error;

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        Ok(Self(id.try_into()?))
    }
}

#[derive(Debug)]
pub enum DagError {
    MissingDependency(KeccakHash),
}

impl std::fmt::Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DagError::MissingDependency(hash) => {
                write!(f, "Missing dependency with hash: {:x?}", hash)
            }
        }
    }
}

impl std::error::Error for DagError {}
