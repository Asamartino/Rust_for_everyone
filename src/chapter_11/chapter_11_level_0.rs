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
    // Same as before, this time add a custom failure message f.i. "The rectangle is not a square as its width adn length are not equal"

    fn add_one(x: u32) -> u32 {
        x + 1
    }
    // Exercise n°
    // The function add_one above add 1 to the input.
    // create a test that assert that  1+1 == 2 using the assert_eq! macro

    // Exercise n°
    // create another test that assert that  1+1 == 2 using the assert! macro and the == operator.
    // note that assert_eq! is more convenient than asset! as if the assertion fails it will print both value.
    // conversely, assert will only indicate that the test failed.

    // Exercise n°
    // create another test that assert that  1+1 != 1 using the assert_ne! macro

    // Exercise n°
    // create another test that assert that  1+1 != 1 using the assert! macro and the != operator.
    // note that assert_eq! is more convenient than asset! as if the assertion fails it will print both value.
    // conversely, assert will only indicate that the test failed.

    

    // filtering tests 215

    // documentation test -> 204 and 289
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
