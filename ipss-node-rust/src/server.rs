extern crate env_logger;

extern crate simple_server;

use crate::dht::{DHTNode};
use simple_server::{Method, Server as SimpServer, StatusCode};
use std::sync::Mutex;

pub struct Server {
  // dht: &impl DHT,
  simp_server: SimpServer,
}

impl Server {
  pub fn new(dht: Mutex<DHTNode>) -> Self {
    Server {
      // dht,
      simp_server: Self::get_simp_server(dht),
    }
  }

  pub fn start(&mut self) {
    let host = "127.0.0.1";
    let port = "7878";
    self.simp_server.listen(host, port);
  }

  fn get_simp_server(dht: Mutex<DHTNode>) -> SimpServer {
    let server = SimpServer::new(move |request, mut response| {
      info!("Request received. {} {}", request.method(), request.uri());

      match (request.method(), request.uri().path()) {
        (&Method::GET, "/hello") => {
          Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
        }
        (&Method::GET, x) => {
          // Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
          println!("Path of {}", x);
          Ok(
            response.body(
              dht
                .lock()
                .unwrap()
                .get_val(DHTNode::get_key(x))
                .unwrap_or("404 Not found".to_string())
                .as_bytes()
                .to_vec(),
            )?,
          )
        }
        (_, _) => {
          response.status(StatusCode::NOT_FOUND);
          Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
        }
      }
    });
    return server;
  }
}
