extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;
use tokio_service::Service;

#[macro_use]
mod http_parser_buf;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        // let headers = empty_headers!(10);
        // let mut req = http_parser::Request::new(headers);
        http_parser_buf::parse_request(buf);
        Ok(None)
        //if let Some(i) = buf.iter().position(|&b| b == b'\n') {
        //    let line = buf.split_to(i);
        //    buf.split_to(1);

        //    match str::from_utf8(&line) {
        //        Ok(s) => Ok(Some(s.to_string())),
        //        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
        //    }
        //} else {
        //    Ok(None)
        //}
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars().rev().collect();
        future::ok(rev).boxed()
    }
}
pub struct LineProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);
    server.serve(|| Ok(EchoRev));
}
