#[cfg(test)]
mod test {
    // fn cal<'a, F>(var: &'a i32, f: F) -> i32
    //     where F: for<'f> Fn(&'f i32) -> i32 {
    //     let local = *var;
    //     f(&local)
    // }
    fn cal<F>(var: &i32, f: F) -> i32
        where F: for<'f> Fn(&'f i32) -> i32 {
        let local = *var;
        f(&local)
    }
    #[test]
    fn test_superior_lifetime() {
        let a = 31i32;
        let b = move |x: &i32| *x * 2;
        let a = cal(&a, b);
        assert_eq!(a, 62);
        let a = 31i32;
        let b = |x: &i32| *x * 2;
        let a = cal(&a, b);
        assert_eq!(a, 62);
        let a = 31i32;
        fn b(x: &i32) ->i32 {
            *x * 2
        }
        let a = cal(&a, b);
        assert_eq!(a, 62);
    }
}