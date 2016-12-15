extern crate bangor;
extern crate tokio_core;

use tokio_core::reactor::Core;
use bangor::db::DBIO;
use bangor::server::Server;
use std::net::SocketAddr;

fn main() {

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    
    let future = Server::new(handle, &"127.0.0.1:9001".parse::<SocketAddr>().unwrap());
    event_loop.run(future).unwrap();

    // this won't actually run until the future resolves
    let mut db = DBIO::new();
    let result = db.append("/tmp/test_bangor_file", "hello, world".as_bytes());
    println!("{:?}", result);
}
