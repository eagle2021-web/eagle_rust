#[cfg(test)]
mod test {
    use std::mem;
    #[test]
    fn test_write() {
        println!("Size of i32: {} bytes", mem::size_of::<i32>());
        println!("Size of f64: {} bytes", mem::size_of::<f64>());
        println!("Size of char: {} bytes", mem::size_of::<char>());
        println!("Size of &str: {} bytes", mem::size_of::<&str>());
        // let s: str = "sdfdsafsadf";
        // the trait `Sized` is not implemented for `str`
        // println!("Size of &str: {} bytes", mem::size_of::<str>());
    }
}
