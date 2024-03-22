/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                        Structs                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Exercise n°1
// create a struct named MyUser with the following fields: username of type String, email of type String, active of type bool, and sign_in_counter of type u32

// Exercise n°2
// create an instance of the structure MyUser and initialized with the value you want.

// Exercise n°3
// create a mutable instance of this struct, initialized with the value you want. Modify the field active from true to false or vice versa

// Exercise n°4
// complete the below function that returns a User (User and MyUser have the same filed)
// default value for active is true and sign_in_counter is 1
fn build_user(username: String, email: String) -> User {
    todo!()
}

//Exercise n°5
//complete the below function that returns a User (User and MyUser have the same filed) using the field init shorthand syntax 
//default value for active is true and sign_in_counter is 1
fn build_user_shorthand(username: String, email: String) -> User {
    todo!()
}

// Exercise n°5 create an instance of User and use this instance to initate some fields in a second instance using the syntax ..


// Exercise n°6 create struct Rectangle_3D with the fields length, width, and height. Those three field are of type u32

// Exercise n°7 create the function rectangle_3D_volume that takes a Rectangle_3D and return the volume of this rectangle (= length*width*height)

// Exercise n°8 implement the volume function as a method of the struct Rectangle_3D
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

    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_counter: u32,
    }

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
