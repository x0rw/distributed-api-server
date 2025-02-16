use std::{mem, vec};

pub enum AuthType {
    Bearer,
    Basic,
    Digest,
    OAuth,
}
pub enum ContentType {
    JSON,
}
struct Auth {
    auth_type: AuthType,
    value: String,
}
pub struct HttpHeader {
    content_type: ContentType,
    Host: String,
    Authorization: Auth,
}
enum Test {
    point2d { x: u32, y: String },
    point1d { y: String },
}
fn process_vec(vector: &mut Vec<u8>) {
    let tkve = mem::take(vector);
    return tkve;
}
fn main() {
    let p1 = &mut Test::point2d {
        x: 43,
        y: String::from("hello"),
    };
    if let Test::point2d { x, y } = p1 {
        let p2 = Test::point1d { y: mem::take(y) };
        print!("{y}");
    };
}
