#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
/// 

mod my_struct;

use crate::my_struct::MyStruct;


pub fn hello_from_bin() -> String {
    let a = MyStruct{a: 4};
    "Hello from flyin-pyrussdsdtsdasdsds!".to_string()
}


#[cfg(feature = "extension-module")]
#[pymodule]
mod _core {
    use pyo3::prelude::*;


    #[pyfunction]
    fn hello_from_bin() -> String {
        "Hello from flyin-pyrust!".to_string()
    }
}
