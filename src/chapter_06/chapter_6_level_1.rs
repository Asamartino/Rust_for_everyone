/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°1                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Exercise n°
// create an enum Shape with three variants (Rectangle, Square, Circle).
// associate an anonymous struct to Rectangle {base: u32, height: u32}, a u32 to Square (that will represent its side) and a u32 to Circle (that will represent its radius)
// create an enum Color with three variants (Red, Yellow, Blue)
// create a struct ShapeInfo with two fields shape of type Shape and color of type Color.

// Exercise n°
// shadow the variable number_of_beer that is an Option<u32> it should now contains the value 5.

/////////////////////////////////////////////////////////////////////////////////////////////////
//////     To complete the exercise below you should read the Pattern syntax part of Chapter 18. read more about match guards.
/////////////////////////////////////////////////////////////////////////////////////////////////

// Exercise n°
// complete the below function get_grade() that takes a u32 and return the following String depending on the score:
// from 0 till 59 "Failed", from 60 to 69 "D", from 70 to 79 "C", from 80 to 89 "B", from 90 to 100 "A" and "Invalid" score for all other possible values
// use the match operator, the special pattern _ and the range literal ..
pub fn get_grade(score: u32) -> String {
    todo!();
}

// Exercise n°
// complete the below function that returns the following String depending on the input
// "Probably a baby" if the value is between 0 and 4
// "Average level" if the value is between 5 and 10
//  "" for all other values expect 9000 in which case it should return "What?! 9000?! There's no way that can be right!"
pub fn power_level(level: u32) -> String {
    todo!();
}

// Exercise n°
// complete the below function using a match and match guards
// returns Lucky if the input is 3,7, or 8
// returns VeryLucky if the input is 777 or 888
// returns Unlucky if the input is 4,9 or 13
// returns VeryUnlucky if the input is 666
// returns Normal in all other cases
pub enum NumberType {
    Lucky,
    VeryLucky,
    Normal,
    Unlucky,
    VeryUnlucky,
}

pub fn luck_number(number: u32) -> NumberType {
    todo!()
}

// Exercise n°
// Complete the below function that should return the following String:
// if temperature is equal or below -10: "It's as cold as your ex's heart"
// if temperature is between -9 and 0: "it's pretty cold, let's make some coco"
// if temperature is between 1 and 10: "It is cold outside"
// if temperature is between 11 and 15: "It is cool outside"
// if temperature is between 16 and 25: "It is warm outside"
// if temperature is between 26 and 40: "Let's go have some ice cream"
// if temperature is higher than 40. "Time to buy an AC..."
pub fn temperature_description(temperature: i32) -> String {
    todo!()
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
