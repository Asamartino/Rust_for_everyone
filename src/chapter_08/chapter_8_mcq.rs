//! # Multiple Choice Questions
//!
//! Replace the todo!() macros with your answer e.g.:
//!
//! Question 0
//!
//! Do you want to learn Rust?
//!
//! - a) yes
//! - b) no
//! - c) maybe
//! - d) I don't know
//!
//! ```notest
//! pub fn answer_0(){
//!  'a'
//! }
//! ```
//!
//! There is only one right answer per question
//!
//!
//! The below command will open the docs in a browser after building them
//! ```sh
//! cargo doc --open
//! ```

/// ## Question 1
///
/// Is the below sentence correct?
/// Vector, String and Hashmap are structures that are stored on the heap. The amount of data does not need to be known at compile time and can adjust its size dynamically as the program runs.
///
/// - a) Yes
/// - b) No, they are stored on the stack.
/// - c) No, the amount of data needs to be known at compile time in order for the program to run.
/// - d) No, the size is adjusted statically.
pub fn answer_01() -> char {
    todo!()
}

// there are other collections see https://doc.rust-lang.org/std/collections/index.html

/// ## Question 2
///
/// Is the below sentence correct?
/// Vector can only store values of the same type.
///
/// - a) No,
/// - b) No, Vector do not store value
/// - c) No, they can be of different type
/// - d) Yes
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Consider the below code:
/// ```notest
/// let counting_sheep = vec![1,2,3,4,5,6,7,8,9];
/// let sheep_number_ten = &counting_sheep[10];
/// ```
/// Will the program compile and if yes what value does sheep_number_ten contains?
///
/// - a) Yes, sheep_number_ten = 9
/// - b) No, the program will panic because we try to access a nonexistent element
/// - c) No, the program will panic because the syntax is incorrect, it should be: let sheep_number_ten = counting_sheep(10)
/// - d) Yes, sheep_number_ten = Some(9)
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Consider the below code:
/// ```notest
/// let counting_sheep = vec![0,1,2,3,4,5,6,7,8,9];
/// let sheep_number_ten = counting_sheep.get(10);
/// ```
/// Will the program compile and if yes what value does sheep_number_ten contains?
///
/// - a) Yes, sheep_number_ten = None
/// - b) Yes, sheep_number_ten = Some(9)
/// - c) Yes, sheep_number_ten = 9
/// - d) No, the program will panic because we try to access a nonexistent element
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Consider the below code:
/// ```notest
/// enum DataTypes{
///     U32(u32),
///     I32(i32),
///     Float(f32),
///     Text(String),
///     Char(char),
/// }
///
/// let vec_datatype = vec![
///     DataTypes::U32(3),
///     DataTypes::I32(-5),
///     DataTypes::Text(String::from("abc")),
///     DataTypes::Float(3.14),
///     DataTypes::Char('a'),
/// ];
/// ```
/// Is the above code valid?
///
/// - a) Yes.
/// - b) No, because a vector can only hold value of the same type. Here you have multiples type u32,i32, float, string and char.
/// - c) No, because in the vector the order has been reversed. The float variable should come before the text variable.
/// - d) No, because a vector can not hold enum values
pub fn answer_05() -> char {
    todo!()
}

/// ## Question
///
/// Consider the below code:
///
/// ```notest
/// let johnny = String::from("Johnny");
/// let bravo = " Bravo".to_string();
/// let johnny_bravo = johnny + &bravo;
///  ```
/// Which variable can no longer be used after the third line?
///
/// - a) Zero, they can all be used.
/// - b) johnny as it has been moved into johnny_bravo
/// - c) johnny and bravo, as both of them have been moved into johnny_bravo
/// - d) johnny, as johnny_bravo shadows it.
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
///
/// Consider the below code:
///
/// ```notest
/// let eeny = String::from("Eeny");
/// let meeny = "Meeny".to_string();
/// let miny = "Miny".to_string();
/// let moe = String::from("Moe");
/// let counting = format!("{} {} {} {}", eeny, meeny, miny, moe);
///  ```
/// Which variable can no longer be used after counting has been defined?
///
/// - a) None, they can all be used as the format! macro does not take ownership of any of its parameters
/// - b) meeny and miny, because they have been defined using the to_string method
/// - c) eeny and moe, because they have been defined using the function String::from
/// - d) eeny, meeny, miny and moe because the format! macro takes ownership of them
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// From Rust point of view, which is not a valid way to look at strings?
///
/// - a) grapheme clusters
/// - b) base characters
/// - c) bytes
/// - d) scalar values
pub fn answer_08() -> char {
    todo!()
}

// Strings are complicated pg 141-142

/// ## Question 9
///
/// is this sentence correct?
/// 
/// Hashmaps are homogeneous: all of their keys and values must have the same type
///
/// - a) No, Hashmaps are heterogenous, the keys can contain different type.
/// - b) No, all the keys must have the same type and all the value must have the same type. The type of key and value can be different.
/// - c) No, Hashmaps are heterogenous, the values can contain different type.
/// - d) Yes
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Consider the below code:
///
/// ```notest
/// let alice = String::from("Alice");
/// let balance: i32 = 10;
/// let mut balances = HashMap::new();
/// balances.insert(alice,balance);
/// println!("the account of {} has been inserted and has a balance of {}", alice, balance)
///  ```
/// does the above code compile?
/// - a) Yes
/// - b) No, because alice is a type String that does not implement the Copy trait. Hence it is moved into balance and can't be used afterwards.
/// - c) No, because balance does not implement the Copy trait. Hence it is moved into balance and can be used afterwards.
/// - d) No, because alice and balance should have the same type.
pub fn answer_10() -> char {
    todo!()
}
/// ## Question 11
///
/// Does the below code compile and what What would be the output?
///
/// ```notest
/// let mut balances = HashMap::new();
/// let alice = String::from("Alice");
/// let balance: i32 = 10;
/// balances.insert(alice,balance);
/// balances.entry(alice).or_insert(25);
/// balances.entry(String::from("Bob")).or_insert(25);
/// println!("{:?}", balances);
///  ```
///
/// - a) Yes, with the output {"Alice": 25, "Bob": 25}.
/// - b) Yes, with the output {"Alice": 10, "Bob": 25}.
/// - c) No, because the variable alice has been moved and thus can no longer be used after being inserted into balances.
/// - d) No, because you can't use the entry function with a non existing key.
pub fn answer_11() -> char {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sanity_check(f: &dyn Fn() -> char) {
        assert!(
            "abcd".contains(f()),
            "{}",
            "You have not returned an answer a, b, c, d."
        )
    }

    #[test]
    fn answer_01_sanity_check() {
        sanity_check(&answer_01)
    }

    #[test]
    fn answer_02_sanity_check() {
        sanity_check(&answer_02)
    }

    #[test]
    fn answer_03_sanity_check() {
        sanity_check(&answer_03)
    }

    #[test]
    fn answer_04_sanity_check() {
        sanity_check(&answer_04)
    }

    #[test]
    fn answer_05_sanity_check() {
        sanity_check(&answer_05)
    }

    #[test]
    fn answer_06_sanity_check() {
        sanity_check(&answer_06)
    }

    #[test]
    fn answer_07_sanity_check() {
        sanity_check(&answer_07)
    }

    #[test]
    fn answer_08_sanity_check() {
        sanity_check(&answer_08)
    }

    #[test]
    fn answer_09_sanity_check() {
        sanity_check(&answer_09)
    }

    #[test]
    fn answer_10_sanity_check() {
        sanity_check(&answer_10)
    }

    #[test]
    fn answer_11_sanity_check() {
        sanity_check(&answer_10)
    }
}
