#[cfg(test)]

mod types;
use types::MyType;

mod tests {
    #[test]
    fn it_works() {
        println!("Mytype: {:?}", MyType::new(42));
    }
}
