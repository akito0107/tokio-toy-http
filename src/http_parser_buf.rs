use bytes::{Bytes, BytesMut, BufMut, Buf};
use std::io::{Error, ErrorKind, Cursor};

macro_rules! advance{
    ($buf:ident, $size:expr) => {
        unsafe {
            $buf.advance($size);
        }
    }
}

pub fn parse_request(bytes: &mut BytesMut) {
    let mut buf = Cursor::new(bytes);
    parse_method(&mut buf);
}

fn parse_method(buf: &mut Cursor<&mut BytesMut>) {
    match buf.get_u8() as char {
        'G' => {
            advance!(buf, 2);
        }
        _ => {
            panic!("not implemented");
        }
    }
    advance!(buf, 1);
}

fn parse_path(buf: &mut Cursor<&mut BytesMut>) {}
