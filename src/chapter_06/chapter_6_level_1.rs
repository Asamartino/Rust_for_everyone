/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°1                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Exercise n°1
// create an enum Shape with three variants (Rectangle, Square, Circle).
// associate an anonymous struct to Rectangle {base: u32, height: u32}, a u32 to Square (that will represent its side) and a u32 to Circle (that will represent its radius)
// create an enum Color with three variants (Red, Yellow, Blue)
// create a struct ShapeInfo with two fields shape of type Shape and color of type Color.

// Exercise n°2
// complete the below function  get_grade() that takes a u32 and return the following depending on the score:
// from 0 till 59 "Failed", from 60 to 69 "D", from 70 to 79 "C", from 80 to 89 "B", from 90 to 100 "A" and "Invalid" score for all other possible values
// use the match operator, the special pattern _ and the range literal ..
pub fn get_grade(score: u32) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Shape {
        Rectangle {
            base: Option<f32>,
            height: Option<f32>,
        },
        Square(Option<f32>),
        Circle(Option<f32>),
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

        assert_eq!(String::from("Failed"), get_grade(score_0));
        assert_eq!(String::from("Failed"), get_grade(score_59));
    }
    #[test]
    fn test_get_grade_d() {
        let score_60 = 60;
        let score_69 = 69;

        assert_eq!(String::from("D"), get_grade(score_60));
        assert_eq!(String::from("D"), get_grade(score_69));
    }
    #[test]
    fn test_get_grade_c() {
        let score_70 = 70;
        let score_79 = 79;

        assert_eq!(String::from("C"), get_grade(score_70));
        assert_eq!(String::from("C"), get_grade(score_79));
    }
    #[test]
    fn test_get_grade_b() {
        let score_80 = 80;
        let score_89 = 89;

        assert_eq!(String::from("B"), get_grade(score_80));
        assert_eq!(String::from("B"), get_grade(score_89));
    }
    #[test]
    fn test_get_grade_a() {
        let score_90 = 90;
        let score_100 = 100;

        assert_eq!(String::from("A"), get_grade(score_90));
        assert_eq!(String::from("A"), get_grade(score_100));
    }
    #[test]
    fn test_get_grade_invalid() {
        let invalid_score = 12345;

        assert_eq!(String::from("invalid"), get_grade(invalid_score));
    }
}
