extern crate futures;

use std::net::SocketAddr;
use futures::Future;
use futures::stream::{ForEach, Stream};
use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, Incoming, TcpStream};
use tokio_core::reactor::{Handle, Core};
use std::io::Error;

pub struct Server;
impl Server {
    pub fn new(handle: Handle, addr: &SocketAddr) -> Box<Future<Item=(), Error=Error>> {
        let socket = TcpListener::bind(&addr, &handle).unwrap();

        println!("listening on {:?}", addr);

        //more or less what tokio-core's echo example is r.n.
        let done = socket.incoming().for_each(move |(socket, addr)| {
            let pair = futures::lazy(|| Ok(socket.split()));
            let amt = pair.and_then(|(reader, writer)| copy(reader, writer));

            handle.spawn(amt.then(move |result| {
                println!("wrote {:?} bytes to {}", result, addr);
                Ok(())
            }));

            Ok(())
        });

        Box::new(done)
    }
}
