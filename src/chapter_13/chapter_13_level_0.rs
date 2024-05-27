fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Closures                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // rewrite the following closure with as minimum syntax as possible
    let add_two = |x: u32| -> u32 { x + 2 };
    // uncomment the below line and complete it
    // let add_two_minimal = todo!();

    // create a closure that returns 42
    // uncomment the below lin and complete it
    let return_forty_two = || 42;

    // create a closure that prints Hello
    // uncomment the below line and complete it
    // let print_hello = todo!();

    // create a closure that takes one argument name and print to the terminal "Hello name"
    // e.g. print_hello_name("John") -> "Hello John"
    // uncomment the below line and complete it
    // let print_hello_name = todo!();

    // create a closure that takes two arguments and add them together
    // uncomment the below line and complete it
    // let adding = todo!();

    // create a closure that captures the variable four and multiply it by another number provided by the user.
    let four = 4;
    // uncomment the below line and complete it
    // let four_multiplied_by = todo!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Iterators                                     //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // create a simple iterator over the value 0,1,2,3,4 from a vector
    // using a for loop print for each element "Value is {}"

    // same exercise as above except this time iterate over an array

    // complete the below function that sum up every element togethers using iterators
    // hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
    pub fn total(vec: Vec<u32>) -> u32 {
        todo!()
    }

    // complete the below function that will return true if any element of this vector is > 0
    // hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
    pub fn any_positive_number(vec: Vec<i32>) -> bool {
        todo!()
    }
    // complete the below function that filters the vector to a vector of only even numbers using iterators
    pub fn even(numbers: Vec<i32>) -> Vec<i32> {
        todo!()
    }

    // complete the below function that find the maximum value using the maximum method of iterator.
    fn find_max(num: Vec<u32>) -> Option<u32> {
        todo!()
    }

    // complete the below function that multiply each element of the iterators by 2 using the map method
    fn iter_multiplied_by_two(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
        todo!()
    }
    // complete the below function that return the sum of the squares using iterators
    pub fn sum_of_squares(num: Vec<u32>) -> u32 {
        todo!()
    }

    // using filter takes only the student that have a score higher thant 50
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
