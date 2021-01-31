extern crate kademlia_dht;
extern crate sha3;

use kademlia_dht::{Key, Node as NodeKademlia, NodeData};
use sha3::{Digest, Sha3_256};

pub struct DHTNode {
    node: NodeKademlia,
}

impl DHTNode {
    pub fn init() -> Self {
        DHTNode {
            node: NodeKademlia::new("localhost", "8080", None), //Some(NodeData{addr: "localhost:8080".to_string(), id: get_key("AASAS")}));
        }
    }

    fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Sized + Default + AsMut<[T]>,
        T: Clone,
    {
        let mut a = Default::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
    }

    pub fn get_key(key: &str) -> Key {
        let mut hasher = Sha3_256::default();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        Key(DHTNode::clone_into_array(result.as_slice()))
    }

    pub fn get_val(&mut self, key: Key) -> Option<String> {
        self.node.get(&key)
    }

    pub fn store_val(&mut self, key: &str, value: &str) {
        let key = DHTNode::get_key(key);
        self.node.insert(key, value);
    }
}
