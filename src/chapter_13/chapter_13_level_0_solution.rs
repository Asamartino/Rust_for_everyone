fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Closures                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // rewrite the following closure with as minimum syntax as possible
    let add_one = |x: u32| -> u32 { x + 1 };
    // let add_one_minimal = todo!();

    // rewrite the following closure with as minimum syntax as possible sur
    // closure sur plusieurs ligne

    // create a closure that prints Hello

    // create a closure that takes one argument name and print to the terminal "hello name"
    // e.g.

    // create a closure that will take two argument and add them together
    // the {} brackets are optional

    // create a closure that captures the variable num and multiply it by another number provided by the user.
    let four = 4;
    // let four_multiplied_by = todo!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Iterators                                     //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // complete the below function that turns the array received as input into an iterator
    // pub fn convert_arr_to_iter(arr: [u32; 5]) -> impl Iterator<Item = u32> {
    //     arr.iter()
    // }

    // complete the below function that sum up every element togethers using iterators
    pub fn total(num: Vec<u32>) -> u32 {
        num.iter().sum()
    }


    // complete the below function that filters the vector to a vector of only even numbers using iterators
    pub fn even(num: Vec<i32>) -> Vec<i32> {
        num.iter().filter(|&x| x % 2 == 0).collect()
    }

    // complete the below function that find the maximum value using the maximum method of iterator.
    fn find_max(num: Vec<u32>) -> Option<&u32> {
        num.iter().max()
    }

    // complete the below function that multiply each element of the iterators by 2 using the map method
    fn iter_multiplied_by_two(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
        iter.map(|x| x * 2)
    }
    // complete the below function that return the sum of the squares using iterators
    pub fn sum_of_squares(num: Vec<u32>) -> u32 {
        num.iter().map(|x| x*x).sum()
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
