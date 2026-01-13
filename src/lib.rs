use std::collections::BinaryHeap;

use crate::cid::{CidTrait, CidV1};

pub mod cid;

pub struct Node<P, C>
where
    C: CidTrait,
{
    pub content: NodeContent<P>,
    pub cid: C,
}

pub struct NodeContent<P> {
    pub payload: P,
    pub timestamp: u64,
    pub dependencies: BinaryHeap<CidV1>,
}
