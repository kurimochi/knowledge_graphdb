use rkyv::{
    Archive, Deserialize, Serialize, api::high::HighSerializer, ser::allocator::ArenaHandle,
    util::AlignedVec,
};
use std::{collections::HashMap, error::Error, fmt::Debug};

use crate::types::{DagError, InternalId, KeccakHash};

pub mod types;

fn keccak256(data: &[u8]) -> KeccakHash {
    use tiny_keccak::{Hasher, Keccak};

    let mut hasher = Keccak::v256();
    hasher.update(data);

    let mut output: KeccakHash = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct NodeContent<P> {
    pub payload: P,
    pub dependencies: Vec<KeccakHash>,
}

pub struct Node<P> {
    pub hash: KeccakHash,
    pub inner: NodeContent<P>,
}

pub struct Dag<P> {
    pub nodes: Vec<Node<P>>,
    pub id_map: HashMap<KeccakHash, InternalId>,
}

impl<P> Dag<P>
where
    for<'a> P: Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>,
{
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id_map: HashMap::new(),
        }
    }

    pub fn import(nodes: Vec<Node<P>>) -> Self {
        let mut id_map = HashMap::new();
        for (idx, node) in nodes.iter().enumerate() {
            id_map.insert(node.hash.clone(), InternalId::from(idx as u32));
        }
        Self { nodes, id_map }
    }

    pub fn add_node(
        mut self,
        payload: P,
        dependencies: Vec<KeccakHash>,
    ) -> Result<(Self, InternalId), Box<dyn Error>> {
        for dep in &dependencies {
            if !self.id_map.contains_key(dep) {
                return Err(DagError::MissingDependency(*dep).into());
            }
        }

        let node = Node::new(payload, dependencies);
        let internal_id = self.nodes.len().try_into()?;

        self.id_map.insert(node.hash.clone(), internal_id);
        self.nodes.push(node);
        Ok((self, internal_id))
    }
}

impl<P> Node<P>
where
    for<'a> P: Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>,
{
    fn new(payload: P, dependencies: Vec<[u8; 32]>) -> Self {
        let inner = NodeContent {
            payload,
            dependencies,
        };
        Self {
            hash: keccak256(&rkyv::to_bytes(&inner).unwrap()),
            inner,
        }
    }
}
