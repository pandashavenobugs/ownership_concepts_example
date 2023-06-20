use functions::my_module::{barrowing_function, function_that_takes_ownership};

use crate::functions::my_module::barrowing_function_with_mutable_parameter;

mod functions {
    pub mod my_module;
}
fn main() {
    // let mut test_data = String::from("hello");
    // test_function(&mut test_data);

    let ownership_data = String::from("my ownership will be taken");
    function_that_takes_ownership(ownership_data);

    // this println! method throws error bc function_that_takes_ownership function has taken the ownership of the ownership_data
    // println!("{}", ownership_data);

    let data_to_be_barrowed = String::from("my ownership won't be taken");

    barrowing_function(&data_to_be_barrowed);

    // at this point we can print the variable because we give the reference instead of ownership
    println!("{}", data_to_be_barrowed);

    let mut mutable_string = String::from("hello");

    println!("{}", mutable_string);

    barrowing_function_with_mutable_parameter(&mut mutable_string);

    println!("{}", mutable_string);
}
