/// This function takes the ownership of the parameter in function
///
/// # Arguments
///
/// * `data` - String type data.
///
/// # Returns
/// * usize  the length of the data.
///
pub fn function_that_takes_ownership(data: String) -> usize {
    return data.len();
}

/// This function gets the reference of the parameter. It won't take the ownership of the parameter
///
///
/// # Arguments
///
/// * `data` - reference of a String type data.
///
/// # Returns
///
/// * usize  the length of the data.

pub fn barrowing_function(data: &String) -> usize {
    return data.len();
}

/// This function gets the reference of the parameter. It won't take the ownership of the parameter
/// * this function adds a word the world end of the data
///
/// # Arguments
///
/// * `data` - reference of a mutable String type data.
///

pub fn barrowing_function_with_mutable_parameter(data: &mut String) {
    data.push_str(" world");
}
