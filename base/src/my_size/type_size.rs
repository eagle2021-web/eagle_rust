macro_rules! print_size {
    ($($t:ty),+) => {
        $(
            println!("Size of {}: {} bytes", stringify!($t), std::mem::size_of::<$t>());
        )+
    };
}

macro_rules! assert_size {
    ($type:ty, $expected:expr) => {
        println!("Size of {}: {} bytes", stringify!($type), std::mem::size_of::<$type>());
        assert_eq!($expected, std::mem::size_of::<$type>());
    };
}


#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::mem;
    use std::rc::Rc;
    use std::sync::Mutex;

    #[test]
    fn test_write() {
        assert_size!(i32, 4);
        assert_size!(i64, 8);
        assert_size!(i64, 8);
        assert_size!(char, 4);
        assert_size!(&str, 16);
        assert_size!(Cell<i32>, 4);
        assert_size!(Cell<i64>, 8);
        assert_size!(Rc<i32>, 8);
        assert_size!(Rc<i64>, 8);
        assert_size!(Rc<i8>, 8);
        assert_size!(Mutex<i32>, 16);
    }
}
