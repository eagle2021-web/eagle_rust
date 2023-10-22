#[cfg(test)]
mod test{
    #[test]
    fn test_a(){
        let v = vec![12];
        let v2 = &v[..];
        let x = 5_i32; // i32
        let x = &5_i32; // &i32
        let ref x = 5_i32; // ???
        let ref x = &5_i32; // ???
    }
}