extern crate proc_macro_lib;

use proc_macro_lib::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}