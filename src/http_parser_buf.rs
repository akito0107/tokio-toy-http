use bytes::{Bytes, BytesMut, BufMut, Buf};
use std::io::{Error, ErrorKind, Cursor, Result};
use std::io::prelude::*;

macro_rules! advance{
    ($buf:ident, $size:expr) => {
        unsafe {
            $buf.advance($size);
        }
    }
}

macro_rules! expect_char{
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

#[derive(Copy, Clone, Debug)]
pub enum HttpVersion {
    Version10,
    Version11,
}

#[derive(Debug)]
pub struct BytesWrapper<'a> {
    bytes: &'a Bytes,
    position: usize,
    len: usize,
}

impl<'a> BytesWrapper<'a> {
    #[inline]
    pub fn new(bytes: &'a Bytes) -> BytesWrapper<'a> {
        BytesWrapper {
            bytes: bytes,
            position: 0,
            len: bytes.len(),
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.position
    }

    #[inline]
    pub fn slice(&mut self, head: usize, tail: usize) -> Result<Bytes> {
        if tail < head || self.len < tail {
            Error::new(ErrorKind::Other, "invalid range")
        }
        Ok(self.bytes.slice(head, tail))
    }

    #[inline]
    fn next(&mut self) -> Option<u8> {
        let b = unsafe { *self.bytes.get_unchecked(self.position) };
        self.position += 1;
        Some(b)
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

pub fn parse_request(bytesMut: &mut BytesMut) -> Result<()> {
    let mut bytes = bytesMut.clone().freeze();
    let mut wrapper = BytesWrapper::new(&bytes);
    let method = try!(parse_method(&mut wrapper));
    wrapper.advance(1);
    let path = try!(parse_token(&mut wrapper));
    println!("{:?}", method);
    println!("{:?}", path);
    Ok(())
}

#[inline]
fn parse_method(bytes: &mut BytesWrapper) -> Result<HttpMethod> {
    let b = bytes.next().unwrap();
    match b as char {
        'G' => {
            bytes.advance(2); //ET
            Ok(HttpMethod::GET)
        },
        //'H' => {
        //    bytes.advance(3);
        //    Ok(HttpMethod::HEAD)
        //},
        //'D' => {
        //    bytes.advance(5);
        //    Ok(HttpMethod::DELETE)
        //}
        //'O' => Ok(HttpMethod::OPTIONS),
        //'P' => {
        //    match bytes.next().unwrap() as char {
        //    }
        //}
        _ => Ok(HttpMethod::OTHER),
    }
}

#[inline]
fn parse_token(bytesWrapper: &mut BytesWrapper) -> Result<Bytes> {
    let pos = bytesWrapper.pos();
    loop {
        let b = bytesWrapper.next().unwrap();
        if b as char == ' ' {
            break;
        }
    }
    let current = bytesWrapper.pos();
    println!("{:?}", pos);
    let u = bytesWrapper.slice(pos, current - 1);
    Ok(u)
}

fn parse_minor_version(bytesWrapper: &mut BytesWrapper) -> Result<()> {

    Ok(())
}
