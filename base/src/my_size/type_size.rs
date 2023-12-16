// #[repr(C)]
struct A{
    i: i8,
    a: i16,

    b: i8,
    c: i8,
    d: i32,
}
#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::mem;
    use std::mem::size_of;
    use std::rc::Rc;
    use std::sync::Mutex;
    use crate::assert_size;
    use super::A;

    #[test]
    fn test_print_a_ptr(){
        let a = A {
            i: 1,
            a: 2,
            b: 3,
            c: 4,
            d:5
        };
        let ptr_a = &a as *const A;

        unsafe {
            println!("Address of i: {:p}", &(*ptr_a).i);
            println!("Address of a: {:p}", &(*ptr_a).a);
            println!("Address of b: {:p}", &(*ptr_a).b);
            println!("Address of c: {:p}", &(*ptr_a).c);
            println!("Address of d: {:p}", &(*ptr_a).d);
        }
        assert_size!(A, 6);
    }
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
        assert_size!(Option<isize>, 16);
        assert_size!(Option<i8>, 2);
        assert_size!(Option<i16>, 4);
        assert_size!(Option<i32>, 8);
        assert_size!(Option<i64>, 16);
        assert_size!(Option<i128>, 24);
        assert_size!(&isize, 8);
        assert_size!(Option<&isize>, 8);
        assert_size!(Box<isize>, 8);
        assert_size!(Box<&isize>, 8);
        assert_size!(*const isize, 8);
        assert_size!(Option<(*const isize)>, 16);

    }
    /// 对不起，发生了错误，让我重新开始。
    ///
    /// 这个结果看起来是在一个则 NULL优化不可能应用的系统上进行的(例如一些嵌入式系统或者某些操作系统)。在这样的系统上，每一个可能的指针值都是合法的，没有一个“保留的”值可以表示Option<T>的None变体。
    ///
    /// 我原本的解释是基于在许多系统中，例如普通的PC或者业务服务器，都会有的NULL优化现象。在这样的系统上，可能有一个或者更多的指针值是不合法的，并且因此可以被用来表示None变体。你提供的输出表明你的系统可能不支持这种优化，所以Option<isize> 和 Option<*const isize> 都需要16个字节的存储空间，即8个字节用于存储具体的值，另外8个字节用于表示Option是否为None。
    ///
    /// 让我们来解释一下各种类型的大小：
    ///
    /// - `isize`， `&isize`, `Box<isize>`, 以及 `*const isize` ：这些类型在一个64位系统上都需要8个字节的存储空间。对于 `isize`类型，8个字节可以存储一个64位整数。而对于 `&isize`, `Box<isize>`, 和 `*const isize`，8个字节被用于存储一个64位的内存地址。
    ///
    /// - `Option<isize>`： 在你的系统上，这种类型需要16个字节的存储空间，8个字节用于存储 `isize` 的值，而另外8个字节用于表示 Option 是否为 None。这是因为在你的系统上，没有“保留的”内存地址可以用于表示 None。
    ///
    /// - `Option<&isize>`, `Option<Box<isize>>`: 这两种类型在你的系统上只需要8个字节的存储空间。当 Option 是 Some 变体时，8个字节的存储空间用于存储内存地址。当 Option 是 None 变体时，Rust 使用特殊的内存地址表示.
    ///
    /// - `Option<*const isize>`: 在你的系统上，这种类型需要16个字节的存储空间，因为没有“保留的”内存地址可以用于表示 None。 因此，这个类型需要额外的8个字节来表示是否为None。
    ///
    #[test]
    fn test_option_size() {

        // assert_size!(A, 6);
        assert_size!(Option<isize>, 16);
        assert_size!(isize, 8);
        assert_size!(Option<i8>, 2);
        assert_size!(Option<i16>, 4);
        assert_size!(Option<i32>, 8);
        assert_size!(Option<i64>, 16);
        assert_size!(Option<i128>, 24);
        assert_size!(&isize, 8);
        assert_size!(Option<&isize>, 8);
        assert_size!(Box<isize>, 8);
        assert_size!(Box<i8>, 8);
        assert_size!(Box<&isize>, 8);
        assert_size!(*const isize, 8);
        assert_size!(*const i8, 8);
        assert_size!(Option<(*const isize)>, 16);
    }
}
