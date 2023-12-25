trait Foo where Self: Sized {
    fn foo(&self);
}

impl Foo for i32 {
    fn foo(&self) {
        println!("{}", self);
    }
}

trait Foo2 {
    fn foo1(&self);
    fn foo2(&self) where Self:Sized;
}

impl Foo2 for i32 {
    fn foo1(&self) {
        println!("foo1 {}", self);
    }

    fn foo2(&self) where Self: Sized {
        println!("foo2 {}", self);
    }
}


fn main() {
    println!("Hello, world!");
    let x = 1_i32;
    x.foo();

    // let p = &x as &dyn Foo; // `Foo` cannot be made into an object
    // p.foo(); //调用失败
    // | trait Foo where Self: Sized {
    //     |       ---             ^^^^^ ...because it requires `Self: Sized`
    //     |       |
    //     |       this trait cannot be made into an object...

    println!("Hello2, world!");
    let x = 1_i32;
    x.foo1();
    x.foo2(); // ok

    let p = Box::new(&x as &dyn Foo2);
    p.foo1();
    // p.foo2(); // err
    // error: the `foo2` method cannot be invoked on a trait object
    // --> e_trait\trait_safe\src\report:46:7
    //     |
    //     13 |     fn foo2(&self) where Self:Sized;
    // |                               ----- this has a `Sized` requirement
    //     ...
    //     46 |     p.foo2(); // err
    // |       ^^^^                            ----- this has a `Sized` requirement

    // 在您提供的代码中，首先定义了一个 `Foo2` 特征（trait），并为 `i32` 类型实现了 `foo1` 和 `foo2` 两个方法。然后创建了一个 `Box` 指针 `p`，将 `&x` 强制转换为 `&dyn Foo2` 来进行动态分发。最后调用了 `p.foo1()`，但注释掉了 `p.foo2()`。
    // 在这种情况下，`foo1` 和 `foo2` 方法的区别主要体现在静态分发和动态分发的差异上：
    // 1. `foo1(&self)` 是一个正常的特征方法，可以通过静态分发和动态分发调用。静态分发的方式是在编译时根据类型进行调用，可以直接使用具体类型的方法实现，效率较高。动态分发则是在运行时通过特征对象来调用，根据对象的动态类型来确定使用哪个具体类型的方法实现。
    // 2. `foo2(&self) where Self: Sized` 方法使用了一个 `where` 子句约束，要求 `Self` 是具有固定大小的类型。这个限定对于静态分发的调用没有影响，因为编译器可以在编译时确定 `Self` 的大小。然而，对于动态分发，特征对象需要被存储在指针或引用中，而不是直接用具体类型来调用方法。由于 `Self` 被限制为具有固定大小的类型，因此无法使用动态分发来调用 `foo2` 方法。
    // 因此，在您的代码中，当您尝试通过 `dyn Foo2` 特征对象来调用 `foo2` 方法时，会引发编译错误。这是由于 `Self: Sized` 的约束，它将限制了动态分发的能力。
    // 总结起来，`foo1` 和 `foo2` 方法的主要区别在于它们是否可以通过动态分发来调用。`foo1` 可以通过静态分发和动态分发进行调用，而 `foo2` 只能通过静态分发调用，因为它带有 `Self: Sized` 的约束。
}
