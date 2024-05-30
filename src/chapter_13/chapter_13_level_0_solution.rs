fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Closures                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // rewrite the following function to a closure with as minimum syntax as possible
    fn add_two (x: u32) -> u32 { x + 2 };
    // uncomment the below line and complete it
    // let add_two_minimal = |x| x + 2;

    // rewrite the below closure with fully type annotation
    // let multiply = |x, y| x * y;

    // create a closure that returns 42
    // uncomment the below line and complete it
    let return_forty_two = || 42;

    // create a closure that prints Hello
    // uncomment the below line and complete it
    let print_hello = || println!("Hello");

    // create a closure that takes one argument name and print to the terminal "Hello name"
    // e.g. print_hello_name("John") -> "Hello John"
    // uncomment the below line and complete it
    let print_hello_name = |name| println!("Hello {}", name);

    // create a closure that takes two arguments and add them together
    // uncomment the below line and complete it
    let adding = |x, y| x + y;

    // create a closure that captures the variable num and multiply it by another number provided by the user.
    let four = 4;
    let four_multiplied_by = |x| x * four;

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Iterators                                     //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // create a simple iterator over the value 0,1,2,3,4 from a vector
    // using a for loop print for each element "Value is {}"
    let v = vec![0, 1, 2, 3, 4];
    for val in v.iter() {
        println!("Value is {}", val);
    }

    // same exercise as above except this time iterate over an array
    let a = [0, 1, 2, 3, 4];
    for val in a.iter() {
        println!("Value is {}", val);
    }

    // complete the below function that sum up every element togethers using iterators
    // hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
    pub fn total(vec: Vec<u32>) -> u32 {
        vec.iter().sum()
    }

    // complete the below function that will return true if any element of this vector is > 0
    // hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
    pub fn any_positive_number(vec: Vec<i32>) -> bool {
        vec.iter().any(|&x| x > 0)
    }

    // // complete the below function that multiply each element of the iterators by 2
    // // hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
    // fn iter_multiplied_by_two(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    //     iter.map(|x| x * 2)
    // }

    // complete the below function that filters the vector to a vector of only even numbers using iterators
    pub fn even(num: Vec<i32>) -> impl Iterator<Item = i32> {
        num.iter().filter(|x| **x % 2 == 0)
    }

    // complete the below function that return the sum of the squares using iterators
    // use the map and the sum method
    pub fn sum_of_squares(vec: Vec<u32>) -> u32 {
        vec.iter().map(|x| x * x).sum()
    }
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
