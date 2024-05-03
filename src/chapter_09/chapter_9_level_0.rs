fn main() {
    /// To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /// Reminder: Result is an enum:
    /// enum Result<T, E> {
    ///     Ok(T),
    ///     Err(E),
    /// }
    /// We will learn more about the <T,E> syntax in chapter 10. For now, think of it as a placeholder for any type that you will define in your code.
    /// Once defined Rust will will ensure adherence to this type.
    /// So Ok(T) will hold a type T wrapped into Ok().

    // Exercise n°1
    // Create a vector and try to access an element that is out of bond.
    // look at the compiler error


    // Exercise n°2
    // Same as before but this time get a backtrace by setting the RUST_BACKTRACE environment variable to any value other than zero.
    // On windows: before executing cargo run in the command line execute the following: set RUST_BACKTRACE=1


    // Exercise n°3
    // Observe the below functions (we will learn more about 'static in the following chapters)
    // if feeling is >=10 this functions returns the value of feeling wrapped into Ok. Otherwise it returns an str wrapped into Err.
    // we use an implicit return
    pub fn annie_are_you_okay(feeling: u32) -> Result<u32, &'static str> {
        if feeling >= 10 {
            return Ok(feeling);
        }
        Err("Annie is not ok")
    }
    // complete the below function. It is similar to the above function, except that it returns Ok(()) if feeling >= 10
    // () type is also called unit: https://doc.rust-lang.org/std/primitive.unit.html
    pub fn so_annie_are_you_okay(feeling: u32) -> Result<(), &'static str> {
        todo!()
    }


    // Exercise n°4
    // complete the below function. It is similar to the annie_are_you_okay function, except that:
    // the value of feeling is wrapped by Ok if feeling >=10 or Err in the other case
    // e.g. are_you_okay_annie(9) -> Err(9)
    pub fn are_you_okay_annie(feeling: u32) -> Result<u32, u32> {
        todo!()
    }


    // Exercise n°5    
    // complete the below function that should return the number wrapped into an Ok
    // if the number is 404 it return Error(404)
    fn not_found(n: u32) -> Result<u32,u32>{
        todo!();
    }


    // Exercise n°6
    // complete the below function that returns the Result of the division of num by den. It should return the following error if den equal zero: "division by 0"
    fn safe_division(num: i32, den: i32) -> Result<i32, &'static str> {
        todo!()
    }


    // Exercise n°7
    // create the variable fourty_two by parsing "42" to an u32 using the parse method: https://doc.rust-lang.org/std/primitive.str.html#method.parse the turbofish syntax and the unwrap method
    // Note: It is an appropriate use of unwrap as we can ensure that Result will have an Ok value. Learn more here: https://blog.burntsushi.net/unwrap/


    // Exercise n°8
    // using the expect method to unwrap the value of the variable unexpected into a variable called expect_the_unexpected
    // You can use qed as an error message: https://en.wikipedia.org/wiki/Q.E.D.
    // Note: It is an appropriate use of expect as, we are sure that unexpected is an Ok value
    let unexpected: Result<&str, f64> = Ok("the unexpected");

    
    // Exercise n°9
    // complete the below function that will divide a by b and multiply the result by c
    // use the safe_division function and the ? operator
    // note that in order to use ? the return of divide_and_multiply must be Result
    fn divide_and_multiply(a: i32, b: i32, c: i32) -> Result<i32, String> {
        todo!()
    }
}
