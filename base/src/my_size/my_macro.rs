#[macro_export]
macro_rules! print_size {
    ($($t:ty),+) => {
        $(
            println!("Size of {}: {} bytes", stringify!($t), std::mem::size_of::<$t>());
        )+
    };
}
#[macro_export]
macro_rules! assert_size {
    ($type:ty, $expected:expr) => {
        println!("Size of {}: {} bytes", stringify!($type), std::mem::size_of::<$type>());
        assert_eq!(std::mem::size_of::<$type>(), $expected);
    };
}