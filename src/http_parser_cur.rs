use std::{io, str, fmt, slice};
use std::io::{Cursor, Seek, SeekFrom, Error, ErrorKind, Read};
use bytes::{BytesMut, BufMut};

macro_rules! skip_cursor{
    ($cur:ident, $n:ident) => {
        match $cur.seek(SeekFrom::Current(n)) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }
    }
}

macro_rules! read_next{
    ($cur:ident, $buf:ident) => {
        match $cur.read_exact($buf) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }
    }
}

macro_rules! expect_byte{
    ($expect:expr, $actual:expr) => {
        if $expect != $actual {
            return Err(Error::new(ErrorKind::Other, "parse error"))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    PUT,
    DELETE,
    OPTIONS,
    POST,
    PATCH,
    OTHER,
}

#[derive(Debug)]
pub struct Request<'a, 'b: 'a> {
    pub method: HttpMethod,
    pub path: &'b str,
    pub headers: &'a mut [Header<'b>],
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn new(headers: &'a mut [Header<'b>]) -> Request<'a, 'b> {
        Request {
            method: HttpMethod::OTHER,
            path: "",
            headers: headers,
        }
    }
}

#[derive(Debug)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub fn parse_request<'a, 'b>(buf: &mut BytesMut) -> io::Result<Option<Request<'a, 'b>>> {
    let mut cur = Cursor::new(buf);
    Ok(None)
}

//fn parse_method(&mut cur: Cursor<&mut BytesMut>) -> io::Result<HttpMethod> {
//    let buf: &mut [u8] = &mut [0; 1];
//    read_next!(cur, buf);
//    Err(Error::new(ErrorKind::Other, "parse error"))
//}
