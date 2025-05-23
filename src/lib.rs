pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn hello_from_lib() {
    println!("Hello from the library!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
