use std::ffi::CStr;
use std::io::prelude::*;
use std::rc::Rc;

fn main() {
    let a: Rc<str> = Rc::from("hello");
    take_rc_str(a.clone());
    dbg!(Rc::strong_count(&a));
    let b = take_and_back(a.clone());

    dbg!(Rc::strong_count(&a));
    dbg!(Rc::strong_count(&b));
    drop(a);
    dbg!(Rc::strong_count(&b));

    let _ = b.len();

    drop(b);

    let s = "hello\0world";
    let mut reader = s.as_bytes();
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    let x = CStr::from_bytes_until_nul(buf.as_slice()).unwrap();
    assert_eq!(x.to_string_lossy(), "hello");
}

fn take_rc_str(s: Rc<str>) -> usize {
    s.len()
}

fn take_and_back(s: Rc<str>) -> Rc<str> {
    s
}
