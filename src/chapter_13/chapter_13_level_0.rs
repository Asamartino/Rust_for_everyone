fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Closures                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Ex 1: Rewrite the following function to a closure with as minimum syntax as possible
    fn add_two(x: u32) -> u32 {
        x + 2
    }
    // uncomment the below line and complete it
    // let add_two_minimal = todo!();

    // Ex 2: Rewrite the following function to a closure with as minimum syntax as possible
    fn navi_being_navi(x: u32, increment: u32) -> u32 {
        println!("Look, x is there {}", x);
        let new_x = x + increment;
        println!("Watch out, x is here{}", new_x);
        new_x
    }
    // uncomment the below line and complete it
    // let navi_being_navi_minimal = todo!();

    // Ex 3: Rewrite the below closure with fully type annotation
    // let multiply = |x, y| x * y;
    // uncomment the below line and complete it
    // let multiply_full = todo!();

    // Ex 4: Create a closure that returns 42
    // uncomment the below line and complete it
    // let return_forty_two = todo!();

    // Ex 5: Create a closure that prints Hello
    // uncomment the below line and complete it
    // let print_hello = todo!();

    // Ex 6: Create a closure that takes one argument name and print to the terminal "Hello name"
    // e.g. print_hello_name("John") -> "Hello John"
    // uncomment the below line and complete it
    // let print_hello_name = todo!();

    // Ex 7: Create a closure that takes two arguments and add them together
    // uncomment the below line and complete it
    // let adding = todo!();

    // Ex 8: Create a closure that captures the variable four and multiply it by another number provided by the user.
    let four = 4;
    // uncomment the below line and complete it
    // let four_multiplied_by = todo!();

    // Ex 9: Write a closure that will increment index by an amount specified by the user
    let mut index = 0;
    // let adding_to_index = todo!()

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Iterators                                     //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Ex 10: Create a simple iterator from the below vector using the iter method
    let vec_0_5 = vec![0, 1, 2, 3, 4, 5];

    // Ex 11:Create a simple iterator from the below array using the iter method
    let array_0_5 = [0, 1, 2, 3, 4, 5];

    // The common methods to create an iterators are (learn more here: https://doc.rust-lang.org/std/iter/#implementing-iterator):
    // - iter(), which iterates over &T.
    // - iter_mut(), which iterates over &mut T.
    // - into_iter(), which iterates over T.

    // Ex 12: Using a for loop and iter_mut increment each value by 1
    let mut mut_vec = [0, 1, 2, 3, 4, 5];

    // for...

    // Notice that the original vector has been updated
    println!("mut_vec is now: {:?}", mut_vec);

    // Ex 13: Using a for loop and into_iter print each value of the below vector
    let into_vec = [0, 1, 2, 3, 4, 5];

    // for...

    // Note that into_iter takes ownership of the element and consumes them -> there are no longer accessible
    // If uncommented the below line will result into a compilation error
    // println!("into_vec is now: {:?}", into_vec);

    // Ex 14: Using a for loop and iter print for each element "Value is {}"
    // note that Rust’s for loop syntax is actually sugar for iterators (learn more here: https://doc.rust-lang.org/std/iter/#for-loops-and-intoiterator)
    let vec_iter = [0, 1, 2, 3, 4, 5];

    // for...

    // Note that vec_iter is still accessible to us and has not been modified
    // println!("vec_iter is: {:?}", into_vec);

    // Ex 15: You can create your own iterator (learn more here: https://doc.rust-lang.org/std/iter/#implementing-iterator)
    // You first need to define a struct that holds the iterator’s state. Then, you need to implement the Iterator trait for this struct
    // Create an Iterator CountDown that starts at 10 and is decrement by 1 till 1, and then yields None.

    // todo!()

    // Uncomment the below line after your iterator has been created
    // let mut count_down = CountDown::new();
    // assert_eq!(count_down.next(), Some(9));
    // assert_eq!(count_down.next(), Some(8));
    // assert_eq!(count_down.next(), Some(7));
    // assert_eq!(count_down.next(), Some(6));
    // assert_eq!(count_down.next(), Some(5));
    // assert_eq!(count_down.next(), Some(4));
    // assert_eq!(count_down.next(), Some(3));
    // assert_eq!(count_down.next(), Some(2));
    // assert_eq!(count_down.next(), Some(1));
    // assert_eq!(count_down.next(), None);
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
