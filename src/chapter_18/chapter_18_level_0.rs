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
