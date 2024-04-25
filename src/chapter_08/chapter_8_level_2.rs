/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// complete the below function that should returns a vector of uniques values only:
fn deduplicate_vector(vec: Vec<u32>) -> Vec<u32> {
  todo!()
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
}

