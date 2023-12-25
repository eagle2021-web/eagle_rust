use std::mem;
trait Bird {
    fn fly(&self);
    fn chirp(&self);
}

struct Duck;
struct Swan;

impl Drop for Duck {
    fn drop(&mut self){
        println!("dddd");
    }
}
impl Bird for Duck {
    fn fly(&self) {
        println!("duck = {}", "fly");
    }
    fn chirp(&self){
        println!("duck = {}", "chirp");
    }
}
impl Bird for Swan {
    fn fly(&self) {
        println!("swan = {}", "fly");
    }
    fn chirp(&self){
        let _a = 112;
        let _b = vec![1212, 12,12213];
        println!("swan = {}", "chirp11111111111111111111111111");
    }

}
// trait object

/// Trait Object: &dyn Trait
///   ├─ data_ptr: 指向实际数据/对象
///   └─ vtable_ptr: 指向虚函数表（vtable）
///
/// 虚函数表 (vtable):
///   ├─ destructor: 用于析构trait对象的函数指针
///   ├─ size: 对象大小
///   ├─ align: 对象对齐要求
///   └─ 方法指针:
///       ├─ method_1: 指向trait要求的第一个方法的实现
///       ├─ method_2: 指向trait要求的第二个方法的实现
///       └─ ...
///
fn print_trait_object(p: &dyn Bird) {
    let (data, vtable): (usize, *const usize) = unsafe {mem::transmute(p)};
    println!("TraitObject[data:{}, vtable:{:p}]", data, vtable);

    unsafe {
        println!("data in vtable [{}, {}, {}, {}, {}]",
        *vtable, *vtable.offset(1),
                 *vtable.offset(2), *vtable.offset(3),  *vtable.offset(4))
    }
}

fn main() {
    println!("Hello, world!");
    let duck = Duck;
    println!("duck size = {}", mem::size_of::<Duck>());

    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    // println!("{:p}", p_duck.drop);
    println!("ptr of duck {:p}, ptr of bird {:p}", p_duck, p_bird);
    println!("Size of p_duck {}, Size of p_bird {}", mem::size_of_val(&p_duck), mem::size_of_val(&p_bird));
    let duck_fly : usize = Duck::fly as usize;
    let swan_fly : usize = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);
    println!("======");
    print_trait_object(p_bird);
    print_trait_object(p_duck);
    print_trait_object(&duck);
    println!("======");
    let swan = Swan;
    print_trait_object(&swan as &dyn Bird);

}
