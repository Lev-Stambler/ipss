
extern crate kademlia_dht;
extern crate sha3;

use kademlia_dht::{Key, Node, NodeData};
use sha3::{Digest, Sha3_256};
use std::thread;
use std::time::Duration;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

fn get_key(key: &str) -> Key {
    let mut hasher = Sha3_256::default();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    Key(clone_into_array(result.as_slice()))
}

pub fn init_dht() {
    let mut node = Node::new("localhost", "8081", Some(NodeData{addr: "localhost:8080".to_string(), id: get_key("AASAS")}));

    let key = get_key("Hello");
    let value = "MAKAKAKAKAK";

    node.insert(key, value);

    // TODO: make this an async operation
    // inserting is asynchronous, so sleep for a second
    thread::sleep(Duration::from_millis(1000));

    assert_eq!(node.get(&key).unwrap(), value);
    println!("Got value of {}", node.get(&key).unwrap());
    loop {};
}