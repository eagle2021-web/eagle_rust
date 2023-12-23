use std::fmt::Display;

fn abc<'a, T: 'a + Sized + Display + AsRef<str>>(s: &'a T) -> &'a str {
    s.as_ref()
}

#[cfg(test)]
mod test {
    use super::{abc};
    use std::path::PathBuf;

    #[test]
    fn test_() {
        let mut buf = PathBuf::from("/");
        let _b = buf.as_os_str();
        buf.set_file_name("bar");
        // error[E0502]: cannot borrow `buf` as mutable because it is also borrowed as immutable
        // 13 |         let b = buf.as_os_str();
        //    |                 --- immutable borrow occurs here
        // 14 |         buf.set_file_name("bar");
        //    |         ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
        // 15 |         println!("{:?}", b);
        //    |                          - immutable borrow later used here
        // println!("{:?}", b);

        if let Some(s) = buf.to_str() {
            println!("{}", s);
        } else {
            println!("invalid path");
        }
    }

    #[test]
    fn test_str_is_sized() {
        let a = "dafdsf";
        let b: Option<&str>;
        {
            b = Some(abc(&a));
        }
        assert!(b.is_some());
        assert_eq!(a, b.unwrap());
        let b = "dsf";
        let c: &str = b.as_ref();
        let d: &str = c.as_ref();
    }
}
