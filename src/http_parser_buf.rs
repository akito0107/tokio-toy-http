use bytes::{Bytes, BytesMut, BufMut, Buf};
use std::io::{Error, ErrorKind, Cursor, Result};
use std::io::prelude::*;

macro_rules! expect_char{
    ($expect:expr, $actual:expr) => {
        if $expect != $actual as char {
            return Err(Error::new(ErrorKind::Other, "parse error"))
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    path: &'a Bytes,
    version: HttpVersion,
    headers: [HttpHeader<'a>],
}

#[derive(Copy, Clone, Debug)]
pub struct HttpHeader<'a> {
    pub name: &'a Bytes,
    pub value: &'a Bytes,
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
    pub fn slice(&mut self, head: usize, tail: usize) -> Bytes {
        self.bytes.slice(head, tail)
    }

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if self.position >= self.len {
            None
        } else {
            let b = unsafe { *self.bytes.get_unchecked(self.position) };
            self.position += 1;
            Some(b)
        }
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
    println!("method: {:?}", method);
    wrapper.advance(1);
    let path = try!(parse_token(&mut wrapper));
    println!("path: {:?}", path);
    let version = try!(parse_minor_version(&mut wrapper));
    println!("version: {:?}", version);
    skip_line(&mut wrapper);

    Ok(())
}

#[inline]
fn parse_method(bytes: &mut BytesWrapper) -> Result<HttpMethod> {
    let b = bytes.next().unwrap();
    match b as char {
        'G' => {
            expect_char!('E', bytes.next().unwrap());
            expect_char!('T', bytes.next().unwrap());
            Ok(HttpMethod::GET)
        }
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
        _ => Err(Error::new(ErrorKind::Other, "not implemented")),
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
    let u = bytesWrapper.slice(pos, current - 1);
    Ok(u)
}

fn parse_minor_version(bytesWrapper: &mut BytesWrapper) -> Result<HttpVersion> {
    expect_char!('H', bytesWrapper.next().unwrap());
    expect_char!('T', bytesWrapper.next().unwrap());
    expect_char!('T', bytesWrapper.next().unwrap());
    expect_char!('P', bytesWrapper.next().unwrap());
    expect_char!('/', bytesWrapper.next().unwrap());
    expect_char!('1', bytesWrapper.next().unwrap());
    expect_char!('.', bytesWrapper.next().unwrap());

    match bytesWrapper.next().unwrap() as char {
        '0' => Ok(HttpVersion::Version10),
        '1' => Ok(HttpVersion::Version11),
        _ => Err(Error::new(ErrorKind::Other, "parse error")),
    }
}

fn parse_headers<'a>(bytes: &'a mut BytesWrapper) -> Result<[HttpHeader<'a>]> {
    let mut headers = Vec::with_capacity(100);
    loop {
        let header = match parse_header(bytes) {
            Ok(res) => res,
            Err(_) => break
        };
        headers.push(header);
    }

    Ok(headers)
}

fn parse_header<'a>(bytes: &'a mut BytesWrapper) -> Result<HttpHeader<'a>> {
    let pos = bytes.pos();
    loop {
        let b = bytes.next().unwrap();
        if b as char == ':' {
            expect_char!(' ', bytes.next().unwrap());
            break;
        }
    }
    let current = bytes.pos();
    let name = bytes.slice(pos, current - 2);
    let val = try!(read_line(bytes));
    Ok(HttpHeader { name: name, value: val })
}

fn read_line(bytes: &mut BytesWrapper) -> Result<Bytes> {
    let pos = bytes.pos();
    let end = 0;
    loop {
        let b = bytes.next().unwrap();
        if b as char == '\n' {
            end = bytes.pos();
            break;
        }
        if b as char == '\r' {
            end = bytes.pos();
            expect_char!('\n', bytes.next().unwrap());
            break;
        }
    }
    let line = bytes.slice(pos, end - 1);
    Ok(line)
}

fn skip_line(bytes: &mut BytesWrapper) -> Result<()> {
    loop {
        let b = bytes.next().unwrap();
        if b as char == '\n' {
            break;
        }
        if b as char == '\r' {
            expect_char!('\n', bytes.next().unwrap());
            break;
        }
    }
    Ok(())
}
