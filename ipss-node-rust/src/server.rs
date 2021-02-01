extern crate env_logger;
extern crate serde;
extern crate simple_server;

use crate::dht::DHTNode;
use serde::{Deserialize, Serialize};
use serde_json;
use simple_server::{Method, Server as SimpServer, StatusCode};
use std::sync::Mutex;
use substring::Substring;

pub struct Server {
  // dht: &impl DHT,
  simp_server: SimpServer,
}

#[derive(Debug, serde_derive::Deserialize, serde_derive::Serialize)]
struct NewItem {
  key: String,
  value: String,
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
        (&Method::POST, "/") => {
          let (parts, body) = request.into_parts();
          // TODO: add error return
          let body: NewItem = serde_json::from_slice(&body).unwrap();

          dht
            .lock()
            .unwrap()
            .store_val(&*body.key, &*body.value);
          Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
        }
        (&Method::GET, x) => {
          let path  = x.substring(1, x.len());
          Ok(
            response.body(
              dht
                .lock()
                .unwrap()
                .get_val(DHTNode::get_key(&path))
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
