use std::{io, str, fmt, slice};
use std::io::{Cursor, SeekFrom, Error, ErrorKind};
use bytes::{BytesMut, BufMut};

macro_rules! next{
    ($itr:ident) => {
        match $itr.next() {
            Some(b) => b,
            None => return Err(Error::new(ErrorKind::Other, "parse error"))
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

macro_rules! next_line{
    ($itr:ident) => {
        loop {
            let b =  next!($itr);
            if b == &13u8 {
                expect_byte!(&10u8, next!($itr));
                break
            }
            if b == &10u8 {
                break
            }
        }
    }
}

macro_rules! empty_headers{
    ($i:expr) => {
        &mut vec![Header{ name: None, value: None}; $i];
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
pub struct Request<'header> {
    pub method: HttpMethod,
    pub path: Option<String>,
    pub minor_version: Option<String>,
    pub headers: &'header mut [Header],
}

#[derive(Clone, Debug)]
pub struct Header {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl<'a> Request<'a> {
    pub fn new(headers: &'a mut [Header]) -> Request<'a> {
        Request {
            method: HttpMethod::OTHER,
            path: None,
            minor_version: None,
            headers: headers,
        }
    }
}

pub fn parse_request<'a>(buf: &mut BytesMut, header_size: u8) -> io::Result<Option<Request<'a>>> {
    let mut itr = buf.iter();
    let method = match parse_method(&mut itr) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    println!("{:?}", method);
    expect_byte!(&32u8, next!(itr)); // ' '
    let path = match parse_path(&mut itr) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    println!("{:?}", path);
    let minor_version = match parse_minor_version(&mut itr) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    println!("{:?}", minor_version);
    next_line!(itr);

    let header_size = match parse_headers(&mut itr, 100) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(None)
}

#[inline]
fn parse_method(itr: &mut slice::Iter<u8>) -> io::Result<HttpMethod> {
    match itr.next().unwrap() {
        &71u8 => {
            // G
            expect_byte!(&69u8, next!(itr)); // E
            expect_byte!(&84u8, next!(itr)); // T
            Ok(HttpMethod::GET)
        }
        _ => Err(Error::new(ErrorKind::Other, "parse error")),
    }
}

#[inline]
fn parse_path(itr: &mut slice::Iter<u8>) -> io::Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity(256);
    loop {
        let b = next!(itr);
        if b == &32u8 {
            break;
        }
        buf.push(*b);
    }
    match String::from_utf8(buf) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, "invalid utf-8 sequence")),
    }
}

#[inline]
fn parse_minor_version(itr: &mut slice::Iter<u8>) -> io::Result<u8> {
    expect_byte!(&72u8, next!(itr)); // 'H'
    expect_byte!(&84u8, next!(itr)); // 'T'
    expect_byte!(&84u8, next!(itr)); // 'T'
    expect_byte!(&80u8, next!(itr)); // 'P'
    expect_byte!(&47u8, next!(itr)); // '/'
    expect_byte!(&49u8, next!(itr)); // '1'
    expect_byte!(&46u8, next!(itr)); // '.'
    match itr.next().unwrap() {
        &49u8 => Ok(1),
        &48u8 => Ok(0),
        _ => Err(Error::new(ErrorKind::Other, "parse error")),
    }
}

#[inline]
fn parse_headers<'a>(itr: &mut slice::Iter<u8>, size: usize) -> io::Result<&'a [Header]> {
    let mut headers: Vec<Header> = Vec::with_capacity(size);
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt > size {
            break;
        }
        match parse_header(itr) {
            Ok(header) => {
                headers.push(*header);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(&headers)
}

#[inline]
fn parse_header<'a>(itr: &mut slice::Iter<u8>) -> io::Result<&'a Header> {
    let mut name_buf: Vec<u8> = Vec::with_capacity(256);
    let mut value_buf: Vec<u8> = Vec::with_capacity(256);
    loop {
        let b = next!(itr);
        if b == &58u8 {
            match next!(itr) {
                &32u8 => break,
                _ => return Err(Error::new(ErrorKind::Other, "parse error")),
            }
        }
        name_buf.push(*b);
    }
    let name = match String::from_utf8(name_buf) {
        Ok(v) => Some(v),
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, "invalid utf-8 sequence")),
    };

    loop {
        let b = next!(itr);
        if b == &139 {
            expect_byte!(&10u8, next!(itr));
            break;
        }
        if b == &10u8 {
            break;
        }
        value_buf.push(*b);
    }
    let value = match String::from_utf8(value_buf) {
        Ok(v) => Some(v),
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, "invalid utf-8 sequence")),
    };

    Ok(&Header {
            name: name,
            value: value,
        })
}
