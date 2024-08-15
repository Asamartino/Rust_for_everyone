fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    // Ex 1: Create an enum for a cons list. This list should store the value 4,5 and 6. Don't use Box<T> and try to recreate
    // the error "recursive type has infinite size"

    // Ex 2: Fix the code you created above by using Box<T>

    // Ex 3: Make the below code compile by adding one character
    // let a = 42;
    // let b = &a;

    // assert_eq(a, 42);
    // assert_eq(b, 42);

    // Ex 4: Create the struct MyBox<T> similar to what was introduced in the book. Implement the Deref trait and the deref method.
    // Find a way to show that *a is the same behind the scene as *(a.deref()) (you could use println!, assert_eq! or any other way).

    // Ex 5: The below code shows you deref coercion at work. Explain what steps that are taken and each type convertion
    // fn say_hi(name: &str){
    //      println!("hi {}", name);
    // }

    // fn main(){
    //      let box_georges = Box::new(String::from(Georges));
    //      say_hi(&box_georges);
    // }


    // Ex 6: 
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
