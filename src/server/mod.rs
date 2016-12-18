extern crate futures;

use std::net::SocketAddr;
use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use std::io::Error;
use std::io::Read;

pub struct Server;
impl Server {
    pub fn new(core: &Core, addr: &SocketAddr) -> Box<Future<Item=(), Error=Error>> {
        let handle = core.handle();
        let socket = TcpListener::bind(&addr, &handle).unwrap();

        println!("listening on {:?}", addr);

        //more or less what tokio-core's echo example is r.n.
        let done = socket.incoming().for_each(move |(socket, addr)| {
            let pair = futures::lazy(|| Ok(socket));
            //let amt = pair.and_then(|(reader, writer)| copy(reader, writer));
            let amt = pair.and_then(|reader| {
              Self::place(reader)
            });
            handle.spawn(amt.then(move |result| {
                println!("wrote {:?} bytes to {}", result, addr);
                Ok(())
            }));

            Ok(())
        });

        Box::new(done)
    }

    pub fn place<A>(from: A) -> BangRequest<A> where A: Read {
        BangRequest {
            state: State::Reading{
                a: from,
                buf: Vec::new(),
                pos: 0
            }
        }
    }
}

enum State<A, T> {
    Reading {
        a: A,
        buf: T,
        pos: usize,
    },
    Empty,
}

pub struct BangRequest<A> where A: Read {
    state: State<A, Vec<u8>>
}

use futures::{Async, Poll};

impl <A> BangRequest<A> where A: Read {
    fn parse_request(&mut self) {
        match self.state {
            _ => ()
        };
    }
}

impl <A> Future for BangRequest<A> where A: Read {
    type Item = usize;
    type Error = Error;

    fn poll(&mut self) -> Poll<usize, Error> {
        Ok(Async::Ready(4))
    }
}
