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
}

fn take_rc_str(s: Rc<str>) -> usize {
    s.len()
}

fn take_and_back(s: Rc<str>) -> Rc<str> {
    s
}
