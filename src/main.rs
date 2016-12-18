extern crate bangor;
extern crate tokio_core;

use tokio_core::reactor::Core;
use bangor::db::DBIO;
use bangor::server::Server;
use std::net::SocketAddr;


fn append() {
    let mut db = DBIO::new();
    let result = db.append("/tmp/test_bangor_file", "hello, world".as_bytes());
    println!("{:?}", result);
}

//for the server
fn start() {
    let mut event_loop = Core::new().unwrap();
    let future = Server::new(&event_loop, &"127.0.0.1:9001".parse::<SocketAddr>().unwrap());
    event_loop.run(future).unwrap();
}


// for the client
//
extern crate futures;
use futures::future::Future;
use tokio_core::net::TcpStream;
use std::io::Read;

fn client_connect() {
    let mut event_loop = Core::new().unwrap();
    let addr = &"127.0.0.1:9001".parse::<SocketAddr>().unwrap();
    let handle = event_loop.handle();
    let future = TcpStream::connect(&addr, &handle).map(|mut stream|{
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf);
        println!("{:?}", buf);
    });
    event_loop.run(future).unwrap();
}

use std::thread;

fn main() {

    let handle = thread::spawn(||start());

    client_connect();

    handle.join();


}
