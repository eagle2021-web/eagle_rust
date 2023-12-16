#![allow(dead_code, unused_imports, unused_macros, unnameable_test_items)]
use std::ops::{Add, Mul, Sub};
// op operation
macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// Implement `add_assign`, `mul_assign`, and `sub_assign` functions.


// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn test_assert_equal_len() {
//         let n = 1;
//         // assert_equal_len!("11", "222", ss, d);
//     }
// }

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);
macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr, $func2:ident) => {

            #[test]
            fn $func2() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);
                    println!("{:?}", &z);
                    assert_eq!(x, z);
                }
            }
        };
    }

// Test `add_assign`, `mul_assign`, and `sub_assign`.

mod test {
    use std::iter;
    test!(add_assign, 1u32, 2u32, 3u32, add_assign222);
    // test!(mul_assign, 2u32, 3u32, 6u32);
    // test!(sub_assign, 3u32, 2u32, 1u32);
    // test!(add_assign, 1u32, 2u32, 3u32);
    #[test]
    fn test_test2() {
        use super::Add;
        println!("{}", stringify!(add_assign));
        println!("{}", stringify!(add_assign));
        add_assign222();
        // let _a = Add::add(1, 2);
    }
}
