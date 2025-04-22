fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        assert_ne!(5 * 2, 11);
    }
}
