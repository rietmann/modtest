#[derive(Debug)]
pub struct MyType {
    x: u32,
}

impl MyType {
    pub fn new(x_: u32) -> MyType {
        MyType { x: x_ }
    }
}
