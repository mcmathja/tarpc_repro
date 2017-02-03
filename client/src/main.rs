extern crate service;
extern crate futures;
extern crate tarpc;
extern crate tokio_core;

use futures::Future;
use tarpc::{client};
use tarpc::client::future::Connect;
use tarpc::util::{FirstSocketAddr};
use tokio_core::reactor;

use service::*;

fn main() {
    let addr = "localhost:10000".first_socket_addr();
    let mut core = reactor::Core::new().unwrap();
    let options = client::Options::default().handle(core.handle());
    core.run(FutureClient::connect(addr, options)
            .map_err(tarpc::Error::from)
            .and_then(|client| client.test("Test".to_string()))
            .map(|resp| println!("{}", resp)))
        .unwrap();
}
