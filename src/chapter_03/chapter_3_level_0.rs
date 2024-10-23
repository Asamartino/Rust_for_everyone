/////////////////////////////////////////////////////////////////////////////////////////////////
//////                         Variables, mutability and shadowing                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// complete the below function by shadowing the x variable. This function should return the value 6.
pub fn return_six_shadowing() -> u32 {
    let x = 5;
    // add your 1-liner code here
    x
}

// complete the below code. This function should return the value 6.
pub fn return_six_mutability() -> u32 {
    todo!()
    //uncoment the below lines
    // let mut x = 5;
    // add your 1-liner code here
    // x
}

// complete the below code. This function should return the char '6' by shadowing x
// note that you can't achieve this behavior if the variable x is declared as mutable
pub fn return_six_char() -> char {
    let x = 5;
    todo!() // remove the todo!()
            // add your 1-liner code here and uncomment the line below
            // x
}
/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// This function should return a tuple with the value (42, 2.0, 'c')
pub fn return_tuple() -> (u32, f32, char) {
    // 1. create a tuple with the value (42, 2.0, 'c')
    // 2. return the tuple
    todo!()
}

// This function should return the u32 value of a tuple (u64, f64, char, u32, f64, i64)
pub fn return_u32_tuple(tup: (u64, f64, char, u32, f64, i64)) -> u32 {
    todo!()
}

// This function creates and and return the following array [1,2,3,4,5]
pub fn return_array_12345() -> [u32; 5] {
    todo!()
}

// This function return an array created by taking the first, third, and fifth element of an array passed to this function
pub fn return_array_1_3_5(my_array: [u32; 5]) -> [u32; 3] {
    // get the first element, third element and fifth element
    // create a new array and return it
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// This function return true if the number is even and false otherwise
// Complete this function using else if conditions
pub fn is_even(number: u32) -> bool {
    // to determine if a number is even you can use % (the modulo operator), e.g. 4 % 2 = 0 while  5 % 2 != 0
    todo!()
}

// this function returns the maximum values between the two numbers provided. If the numbers are equal this function return the number
// Complete this function using else if conditions
pub fn return_maximum(n1: u32, n2: u32) -> u32 {
    todo!()
}

// this function returns the maximum values between the two numbers provided. If the numbers are equal this function return the number
// This time complete this function using if on the right side of a let statement
pub fn return_maximum_if_let(n1: u32, n2: u32) -> u32 {
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// In rust there are three types of loops:
//      - loop: creates an infinite loop that can be stopped by using a break statement
//      - while: will run till the condition is true
//      - for: is a more concise and safe way than a while loop

// Complete the function below by incrementing the number by 1 during each iteration of the loop. You should use a loop that  exits when the number reaches 100.
pub fn return_hundred_loop() -> u32 {
    todo!()
    // uncomment the code below and complete the loop
    // let mut number = 0;
    // loop {
    //     // complete this part of the loop
    //     // add an if clause that return break if the number 100 is reached
    // }
    // number
}

// Complete the function below by incrementing the number by 1 during each iteration of the loop. You should use a loop that  exits when the number reaches 100.
pub fn return_hundred_while() -> u32 {
    todo!()
    // uncomment the code below and complete
    // let mut number = 0;
    // add a while loop
    // number
}

// Complete the function below by incrementing the number by 1 during each iteration of the loop. You should use a loop that  exits when the number reaches 100.
pub fn return_hundred_for() -> u32 {
    todo!()
    // uncomment the code below and complete
    // let mut number = 0;
    // add a for loop
    // number
}

#[cfg(test)]
mod tests {
    use super::*;

    // Variables, mutability and shadowing
    #[test]
    fn test_return_six_shadowing() {
        assert_eq!(6, return_six_shadowing());
    }

    #[test]
    fn test_return_six_mutability() {
        assert_eq!(6, return_six_mutability());
    }

    #[test]
    fn test_return_six_char() {
        assert_eq!('6', return_six_char());
    }

    // Data Types
    #[test]
    fn test_return_tuple() {
        assert_eq!((42, 2.0, 'c'), return_tuple());
    }

    #[test]
    fn test_return_u32_tuple() {
        let answer: u32 = 2;
        let tup = (1, 1.0, 'a', answer, 3.0, -3);
        assert_eq!(answer, return_u32_tuple(tup));
    }

    #[test]
    fn test_return_array_12345() {
        let answer = [1, 2, 3, 4, 5];
        assert_eq!(answer, return_array_12345());
    }

    #[test]
    fn test_return_array_1_3_5() {
        let arr = [1, 2, 3, 4, 5];
        let answer = [1, 3, 5];
        assert_eq!(answer, return_array_1_3_5(arr));
    }

    // Else if
    #[test]
    fn test_is_even() {
        let num: u32 = 4;
        assert_eq!(num % 2 == 0, is_even(num));
    }

    #[test]
    fn test_is_not_even() {
        let num: u32 = 5;
        assert_eq!(num % 2 != 0, is_even(num));
    }

    #[test]
    fn test_return_maximum_1() {
        let n1: u32 = 10;
        let n2: u32 = 100;
        assert_eq!(n2, return_maximum(n1, n2));
    }

    #[test]
    fn test_return_maximum_2() {
        let n1: u32 = 10;
        let n2: u32 = 100;
        assert_eq!(n2, return_maximum(n2, n1));
    }

    #[test]
    fn test_return_maximum_same_number() {
        let n1: u32 = 10;
        assert_eq!(n1, return_maximum(n1, n1));
    }

    #[test]
    fn test_return_maximum_if_let_1() {
        let n1: u32 = 10;
        let n2: u32 = 100;
        assert_eq!(n2, return_maximum_if_let(n1, n2));
    }

    #[test]
    fn test_return_maximum_if_let_2() {
        let n1: u32 = 10;
        let n2: u32 = 100;
        assert_eq!(n2, return_maximum_if_let(n2, n1));
    }

    #[test]
    fn test_return_maximum_if_let_same_number() {
        let n1: u32 = 10;
        assert_eq!(n1, return_maximum_if_let(n1, n1));
    }

    // Loops
    #[test]
    fn test_return_hundred_loop() {
        let answer: u32 = 100;
        assert_eq!(answer, return_hundred_loop());
    }

    #[test]
    fn test_return_hundred_while() {
        let answer: u32 = 100;
        assert_eq!(answer, return_hundred_while());
    }

    #[test]
    fn test_return_hundred_for() {
        let answer: u32 = 100;
        assert_eq!(answer, return_hundred_for());
    }

}
