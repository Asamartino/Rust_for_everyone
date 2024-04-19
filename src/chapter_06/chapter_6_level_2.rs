/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Exercise n°1
// create an enum Shape with three variants (Rectangle, Square, Circle).
// associate an anonymous struct to Rectangle {base: Option<f32>, height: Option<f32>}, a Option<f32> to Square (that will represent its side) and a Option<f32> to Circle (that will represent its radius)
pub enum Shape{
    // complete this enum
    Rectangle {
        base: Option<f32>,
        height: Option<f32>,
    },
    Square(Option<f32>),
    Circle(Option<f32>),
}

// Exercise n°2
// complete the below function area that compute the area of a shape using a match. In the case that the Shape variant contains None it returns 0.
pub fn area(shape: Shape) -> f32 {
    // todo!();

    match shape{
        Shape::Rectangle { base, height } => match (base, height) {
            (Some(base), Some(height)) => base * height,
            _ => 0.0,
        },
        Shape::Square(x) => {
            match x {
                Some(side) => side * side,
                None => 0.0,
            }
        },
        Shape::Circle(x) => 
            match x {
                Some(r) => 3.14 * r * r,
                None => 0.0,
            },
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_rectangle() {
        let base =10.0 ;
        let height = 3.0;
        let rectangle = Shape::Rectangle{base: Some(base), height: Some(height)};

        assert_eq!(base*height, area(rectangle));
    }
    #[test]
    fn test_area_rectangle_null() {
        let base =5.0 ;
        let rectangle_none = Shape::Rectangle{base: Some(base), height: None};

        assert_eq!(0.0, area(rectangle_none));
    }

    #[test]
    fn test_area_square() {
        let side = 5.0;
        let square = Shape::Square(Some(side));

        assert_eq!(side*side, area(square));
    }
    #[test]
    fn test_area_square_null() {
        let square_none = Shape::Square(None);

        assert_eq!(0.0, area(square_none));
    }

    #[test]
    fn test_area_circle() {
        let radius = 10.0;
        let circle = Shape::Circle(Some(radius));

        assert_eq!(3.14*radius*radius, area(circle));
    }
    #[test]
    fn test_area_circle_null() {
        let circle_none = Shape::Circle(None);

        assert_eq!(0.0, area(circle_none));
    }
}

