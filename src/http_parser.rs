use std::{io, str, fmt, slice};
use std::io::{Error, ErrorKind};
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
    let mut itr = buf.iter();
    let method = match parse_method(&mut itr) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    expect_byte!(&32u8, next!(itr)); // ' '
    expect_byte!(&47u8, next!(itr)); // '/'
    expect_byte!(&32u8, next!(itr)); // ' '
    let minor_version = match parse_minor_version(&mut itr) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    println!("{:?}", method);
    println!("{:?}", minor_version);
    next_line!(itr);

    Ok(None)
}

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

fn parse_path(itr: &mut slice::Iter<u8>) -> io::Result<str> {
    let buf: Vec<u8> = vec![];
    loop {
        let b = next!(itr);
        if b == &32u8 {
            break;
        }
        buf.push(b);
    }
    match str::from_utf8(buf) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, "invalid utf-8 sequence")),
    }
}

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

fn parse_header(itr: &mut slice::Iter<u8>, &mut headers: [Header]) -> io::Result<u8> {}

// fn parse_method(buf: &mut BytesMut) -> io::Result<Option<HttpMethod>> {}
