// error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
// --> base\src\.\log_type\mod.rs:4:24
// |
// 4 |         println!("{}", std::intrinsics::type_name::<T>());
// |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// |
// = help: add `#![feature(core_intrinsics)]` to the crate attributes to enable
// #![feature(core_intrinsics)]
// fn print_type_name<T>(_arg: &T) {
//     unsafe {
//         println!("{}", std::intrinsics::type_name::<T>());
//         }
// }
fn type_id(_: ()) {}

#[cfg(test)]
mod test{
    use super::type_id;
    // use super::print_type_name;
    #[test]
    fn test_panic(){
        let ref x = 5_i32;
        // 实际参数的类型肯定不是 unit,此处必定有编译错误,通过编译错误,我们可以看到实参的具体类型
        // type_id(x);
        let ref x = 5_i32;
        // print_type_name(&x);
    }
}