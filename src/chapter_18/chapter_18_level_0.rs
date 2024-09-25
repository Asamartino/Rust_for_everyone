fn main() {
    // complete the below function who depending on the variable should return the following string depending on the below requirements
    // if pet = Some("Dog") and drink = Some("Coffee") -> "You've got coffee! The perfect combo for dog walking!"
    // if pet = Some("Dog") and drink != Some("Coffee") -> "No coffee? Better find some energy for that enthusiastic pup!"
    // if pet = Some("Cat") and drink = Some("Tea") -> "Tea and cat? what a great combo"
    // if pet = Some("Cat") and drink = Some("Coffee") -> "Time to play"
    // if pet = None -> "No pets? What's wrong? Are you secretly a plant lover?"

    // use if let and,else if and elseif let expression
    pub fn pet_drink_advice(pet: Option<&str>, drink: Option<&str>) -> String {
        todo!()
    }

    // reverse the vector provided using while let and the pop function
    pub fn vec_inversion(v: Vec<u32>) -> Vec<u32>{
        todo!()
    }

    // complete the function that creates a new vector containing only the values from even indices of the original vector using a for loop and enumerate()
    pub fn even_indexed_from_previous_vector(v : Vec<u32>) -> Vec<u32>{
        todo!()
    }

    // complete the function that computes the sum of all values at odd indices in a vector using a for loop and enumerate()
    pub fn sum_of_value_at_odd_indices(v : Vec<u32>) -> u32{
        todo!()
    }

    // using match, the | syntax, and the .. syntax the below function should return
    // "Below 0" if the number is smaller than 0
    // "Between 1 and 10" if the number is between 1 and 10 
    // "Exactly 15 or 20" if the number is = 15 or 20
    // "Other" in any other cases
    pub fn classify_number(n: i32) -> &'static str {
        todo!()
    }

    // using only one match and the _ syntax complete the below function so that it returns:
    // "Both numbers are even" if n1%2=0 & n2%2=0
    // "One number is even and the other is uneven" if n1%2=0 & n2%2!=0 or n1%2!=0 & n2%2=0
    // "Both numbers are uneven" in all other cases
    pub fn even_uneven(n1: u32, n2: u32) -> &'static str{
        todo!()
    }

    // using the .. syntax and a match arm that returns the addition of the first and last element
    pub fn adding_first_and_last(arr: [u32;1000]) -> u32{
        todo!()
    }

    // using match and ref mut the below option should change the value contain by the option to 4
    pub fn replace_by_4(op: &mut Option<u32>) -> Option<u32>{
        todo!()
    }

    // using match guards the following function should return:
    // "Contains two negative number" if both number are <0.
    // "Contains one negative number" if one number < 0
    // "Both large numbers" if both values >= 1000
    // "Other pair" in all the other cases 
    pub fn check_number(pair: (i32,i32)) -> &'static str{
        todo!()
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
