
mod my_struct;

use crate::my_struct::MyStruct;
use _core::hello_from_bin;

fn main() {
    let a = MyStruct{a: 2};
    println!("{}", hello_from_bin());
}
