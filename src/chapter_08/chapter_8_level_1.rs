// have a look at the doc of the different methods of vector, before or while solving the below exercises : https://doc.rust-lang.org/std/vec/struct.Vec.html

// complete the below function that should remove the last element from a vector and returns it, or return None if the vector is empty
pub fn vec_pop(vec: &mut Vec<u32>) -> Option<u32> {
    todo!()
}

// complete the below function which should return the length of a vector
pub fn vec_length(vec: &Vec<u32>) -> usize {
    todo!()
}

// complete the below function that should return the sum of the element of a vector
pub fn sum_of_vec(vec: &Vec<u32>) -> u32 {
    todo!()
}

// complete the below function that should return the mean of a vector.
// TIP: you could use the result of vec_sum and divide it by the number of element of a vector
pub fn mean_of_vec(vec: &Vec<u32>) -> u32 {
    todo!()
}

// complete the below function that should append the content of vec2 to vec1 and leaving vec2 empty.
pub fn vec_append(vec1: &mut Vec<u32>, vec2: &mut Vec<u32>) -> Vec<u32> {
    todo!()
}

// complete the below function that should append the content of vec2 to vec1 and not let vec2 empty.
pub fn vec_append_not_empty(vec1: &mut Vec<u32>, vec2: &mut Vec<u32>) -> Vec<u32> {
    todo!()
}

// have a look at the doc of the different methods of String, before or while solving the below exercises: https://doc.rust-lang.org/std/string/struct.String.html

// complete the below function that should convert the input string into lowercase. Assume that the input only contains latin characters
pub fn string_to_lowercase(string: String) -> String {
    todo!()
}

// reversed the string provided as input, e.g. string_reversed("hello") -> "olleh" . Assume that the input only contains latin characters
pub fn string_reversed(string: &str) -> String {
    todo!()
}

// Count the number of vowel (i.e. a,e,i,o,u ) in a string provided as input. Assume that the input only contains latin characters (that can also be uppercase)
fn count_vowels(input: &String) -> usize {
    todo!()
}

// Have a look at the different methods of hashmap before or while solving the below exercises: https://doc.rust-lang.org/std/collections/struct.HashMap.html

// returns the sum of the values of a hashmap
fn hashmap_values_sum(hm: &HashMap<String,u32>) ->u32{

}

// returns the sum of the keys of a hashmap
fn hashmap_keys_sum(hm: &HashMap<u32,u32>) -> u32{

}


#[cfg(test)]
mod tests {
    use super::*;

    enum Shape {
        Rectangle { base: u32, height: u32 },
        Square(u32),
        Circle(u32),
    }
    enum Color {
        Red,
        Yellow,
        Blue,
    }

    struct ShapeInfo {
        shape: Shape,
        color: Color,
    }

    #[test]
    fn test_get_grade_failed() {
        let score_0 = 0;
        let score_59 = 59;
        let failed = String::from("Failed");

        assert_eq!(failed, get_grade(score_0));
        assert_eq!(failed, get_grade(score_59));
    }
    #[test]
    fn test_get_grade_d() {
        let score_60 = 60;
        let score_69 = 69;
        let grade_d = String::from("D");

        assert_eq!(grade_d, get_grade(score_60));
        assert_eq!(grade_d, get_grade(score_69));
    }
    #[test]
    fn test_get_grade_c() {
        let score_70 = 70;
        let score_79 = 79;
        let grade_c = String::from("C");

        assert_eq!(grade_c, get_grade(score_70));
        assert_eq!(grade_c, get_grade(score_79));
    }
    #[test]
    fn test_get_grade_b() {
        let score_80 = 80;
        let score_89 = 89;
        let grade_b = String::from("B");

        assert_eq!(grade_b, get_grade(score_80));
        assert_eq!(grade_b, get_grade(score_89));
    }
    #[test]
    fn test_get_grade_a() {
        let score_90 = 90;
        let score_100 = 100;
        let grade_a = String::from("A");

        assert_eq!(grade_a, get_grade(score_90));
        assert_eq!(grade_a, get_grade(score_100));
    }
    #[test]
    fn test_get_grade_invalid() {
        let invalid_score = 12345;

        assert_eq!(String::from("invalid"), get_grade(invalid_score));
    }

    #[test]
    fn test_power_level_baby() {
        let level = 4;

        assert_eq!(String::from("Probably a baby"), power_level(level));
    }

    #[test]
    fn test_power_level_average() {
        let level = 10;

        assert_eq!(String::from("Average level"), power_level(level));
    }
    #[test]
    fn test_power_level_nothing() {
        let eleven = 11;
        let nine_thousand_one = 9001;

        assert_eq!(String::from(""), power_level(eleven));
        assert_eq!(String::from(""), power_level(nine_thousand_one));
    }
    #[test]
    fn test_power_level_nine_thousand() {
        let nine_thousand = 900;

        assert_eq!(
            String::from("What?! 9000?! There's no way that can be right!"),
            power_level(nine_thousand)
        );
    }
}
