fn main() {
    /// To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        pub fn is_square(&self) -> bool {
            self.length == self.width
        }
    }

    // Exercise n°
    // The function above return true if the Rectangle is a square
    // Create a test in which you create a Rectangle with the same value for width and length and use the is_square function inside the assert! macro.

    // Exercise n°
    // Same exercise as the precedent. Except this time create a Rectangle with different value for width and length.
    // Negate the result of the is_square function inside the assert! macro

    // Exercise n°
    // Rerun the above test without negating the result just to see the failure message.

    // Exercise n°
    // Same as before, this time add a custom failure message in the assert! macro, f.i. "The rectangle is not a square as its width and length are not equal"

    // Exercise n°
    // add the attribute should_panic on the above test to make it passe

    fn add_one(x: u32) -> u32 {
        x + 1
    }
    // Exercise n°
    // The function add_one above add 1 to the input.
    // create a test that assert that  1+1 = 2 using the assert_eq! macro

    // Exercise n°
    // create another test that assert that  1+1 = 2 using the assert! macro and the == operator.
    // note that assert_eq! is more convenient than asset! as if the assertion fails it will print both value.
    // conversely, assert will only indicate that the test failed.

    // Exercise n°
    // create another test that assert that  1+1 != 1 using the assert_ne! macro

    // Exercise n°
    // create another test that assert that  1+1 != 1 using the assert! macro and the != operator.
    // note that assert_ne! is more convenient than asset! as if the assertion fails it will print both value.
    // conversely, assert will only indicate that the test failed.

    fn panic_if_not_42(x: u32) {
        if x < 42 {
            panic!("The value provided is lower than 42");
        }
        if x > 42 {
            panic!("The value provided is higher than 42");
        }
    }

    // Exercise n°
    // create a test that will ensure that the function panic using the should_panic attribute

    // Exercise n°
    // using the expected parameter specify the expected message.
    // The expected parameter is a substring of the panic message. So you can define the level of precision of your test.
    // try providing a value higher than 42 and with two level of precision f.i. "the value provided" and "the value provided is higher than 42"

    fn say_hello() -> bool {
        println!("hello");
        true
    }
    // Exercise n°
    // create a passing test for the above function in order to see the passing message
    // you will have disable the output capture by using the --no-capture flag

    // filtering tests 215

    // documentation test -> 204 and 289

    // create an integration test
}

#[cfg(test)]
mod tests {
    mod tests {
        #[test]
        fn test_is_equal() {
            todo!()
        }
        #[test]
        fn test_add_three() {
            todo!()
        }
    }
}
