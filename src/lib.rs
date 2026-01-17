use core::error;
use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::cid::{CidTrait, CidV1};

pub mod cid;

pub struct Dag<P, C: CidTrait>(pub HashMap<C, Node<P, C>>);

pub struct Node<P, C>
where
    C: CidTrait,
{
    pub content: NodeContent<P>,
    pub cid: C,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeContent<P> {
    pub payload: P,
    pub timestamp: u64,
    pub dependencies: BinaryHeap<CidV1>,
}

impl<P> NodeContent<P>
where
    P: Serialize,
{
    // WARNING: This is a temporary addition and will be replaced later
    pub fn generate_cid(&self) -> Result<CidV1, Box<dyn error::Error>> {
        let mut buffer = Vec::new();
        ciborium::into_writer(&self, &mut buffer)?;

        let mut hasher = Keccak256::new();
        hasher.update(buffer);

        Ok(CidV1(hasher.finalize().into()))
    }
}
