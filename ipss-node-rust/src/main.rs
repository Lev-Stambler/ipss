#[macro_use]
extern crate log;

mod dht;
mod server;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

fn main() {
    // lazy_static! {
    //     static ref dht: dht::DHTNode = dht::DHTNode::init();
    // }
    let dht = dht::DHTNode::init();
    let dht_mut = Mutex::new(dht);
    let mut server = server::Server::new(dht_mut);
    std::thread::spawn(move || {
        server.start();
    });
    loop {}
}
