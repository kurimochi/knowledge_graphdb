use std::collections::BinaryHeap;

use dag::NodeContent;

fn main() {
    let nodecontent = NodeContent {
        payload: 1024u32,
        timestamp: 512,
        dependencies: BinaryHeap::new(),
    };
    let cid = nodecontent.generate_cid().unwrap();
    let mut buffer = Vec::new();
    ciborium::into_writer(&nodecontent, &mut buffer).unwrap();
    println!("{}", hex::encode(buffer));
    println!("{}", cid);
}
