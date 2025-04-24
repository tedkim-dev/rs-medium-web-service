pub fn hello() -> &'static str {
    "Hello, world! from libs"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_works() {
        let result = hello();
        assert_eq!(result, "Hello, world! from libs");
    }
}
