

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                   String Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// create a String that returns "hello"
// learn more here: https://doc.rust-lang.org/std/string/struct.String.html#
pub fn return_hello() -> String {
    todo!()
}

// create a mutable String with the value hello and append " world" to it using the push_str method
// learn more here: https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
pub fn return_hello_world() -> String {
    todo!()
}

// create a String slices that returns "hello universe"
// learn more here: https://doc.rust-lang.org/std/primitive.str.html#
// don't worry to much about 'static, will learn more about it in Chapter 10
pub fn return_hello_universe() -> &'static str {
    todo!()
}


/////////////////////////////////////////////////////////////////////////////////////////////////
//////                            References and Borrowing                                 //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// this function should return the length of a String using the .len() method
// learn more here: https://doc.rust-lang.org/std/string/struct.String.html#method.len
// notice that the argument of the function is a reference to a String -> the function do not take ownership of the String
pub fn string_length(s: &String) -> usize {
    todo!()
}


/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                    Slice Type                                       //////
/////////////////////////////////////////////////////////////////////////////////////////////////



#[cfg(test)]
mod tests {
    use super::*;

    // Variables, mutability and shadowing
    #[test]
    fn test_return_hello() {
        assert_eq!("hello", return_hello());
    }

    #[test]
    fn test_return_six_mutability() {
        assert_eq!("hello world", return_hello_world());
    }
}
