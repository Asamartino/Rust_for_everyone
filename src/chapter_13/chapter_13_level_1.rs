// exercise with move

// iterator over hashmap

// pub fn convert_arr_to_iter(arr: [u32; 5]) -> impl Iterator<Item = u32> {
//     arr.into_iter()
// }

// use function enumarate


// complete the below function that sum up every element togethers using iterators
// hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
pub fn total(arr: [u32; 10]) -> u32 {
    todo!()
}

// complete the below function that will return true if any element of this array is > 0
// hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
pub fn any_positive_number(arr: [i32; 10]) -> bool {
    todo!()
}

// complete the below function that filters the vector to a vector of only even numbers using iterators
// it returns an empty vector if the the vector has no even numbers
pub fn even(num: Vec<u32>) -> Vec<u32> {
    todo!()
}

// complete the below function that find the maximum value using the maximum method of iterator.
// it returns None if the vector is empty
pub fn find_max(num: Vec<u32>) -> Option<u32> {
    todo!()
}

// complete the below function that multiply each element of the iterators by 2 using the map method
pub fn iter_multiplied_by_two(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    // comment the below line. We had to put it instead of the usual todo!() otherwise the compiler complains
    0..2
}
// complete the below function that return the sum of the squares using iterators
// returns 0 if the vector is empty
pub fn sum_of_squares(num: Vec<u32>) -> u32 {
    todo!()
}

// complete the below function that returns true if max is the maximum value in this vector.
// use the max method of iterator.
// return false is the vector is empty
pub fn is_max(vec: Vec<u32>, max: u32) -> bool {
    todo!()
}

///////////////////////////////////////////////////////////////////////////////
// The following exercises assumes that you will receive
// an array of SwissMountain as defined below:
// Maybe go with vector ???

#[derive(Debug, PartialEq, Clone)]
pub struct SwissMountain {
    name: String,
    height: u32,
}

// let aletschhorn = SwissMountain {
//     name: String::from("Aletschhorn"),
//     height: 4194,
// };
// let alphubel = SwissMountain {
//     name: String::from("Alphubel"),
//     height: 4206,
// };
// let breithorn = SwissMountain {
//     name: String::from("Breithorn"),
//     height: 4160,
// };
// let dent_blanche = SwissMountain {
//     name: String::from("Dent Blanche"),
//     height: 4357,
// };
// let dent_dherens = SwissMountain {
//     name: String::from("Dent d'Hérens"),
//     height: 4173,
// };
// let dom = SwissMountain {
//     name: String::from("Dom"),
//     height: 4546,
// };
// let finsteraarhorn = SwissMountain {
//     name: String::from("Finsteraarhorn"),
//     height: 4274,
// };
// let grand_combin = SwissMountain {
//     name: String::from("Grand Combin"),
//     height: 4309,
// };
// let gross_fiescherhorn = SwissMountain {
//     name: String::from("Gross Fiescherhorn"),
//     height: 4049,
// };
// let jungfrau = SwissMountain {
//     name: String::from("Jungfrau"),
//     height: 4158,
// };
// let lyskamm = SwissMountain {
//     name: String::from("Lyskamm"),
//     height: 4532,
// };
// let matterhorn = SwissMountain {
//     name: String::from("Matterhorn"),
//     height: 4478,
// };
// let monch = SwissMountain {
//     name: String::from("Mönch"),
//     height: 4110,
// };
// let monte_rosa = SwissMountain {
//     name: String::from("Monte Rosa"),
//     height: 4634,
// };
// let ober_gabelhorn = SwissMountain {
//     name: String::from("Ober Gabelhorn"),
//     height: 4063,
// };
// let rimpfischhorn = SwissMountain {
//     name: String::from("Rimpfischhorn"),
//     height: 4199,
// };
// let schreckhorn = SwissMountain {
//     name: String::from("Schreckhorn"),
//     height: 4078,
// };
// let strahlhorn = SwissMountain {
//     name: String::from("Strahlhorn"),
//     height: 4190,
// };
// let weisshorn = SwissMountain {
//     name: String::from("Weisshorn"),
//     height: 4505,
// };
// let zinalrothorn = SwissMountain {
//     name: String::from("Zinalrothorn"),
//     height: 4221,
// };

// let heighest_swiss_mountains = [
//     aletschhorn,
//     alphubel,
//     breithorn,
//     dent_blanche,
//     dent_dherens,
//     dom,
//     finsteraarhorn,
//     grand_combin,
//     gross_fiescherhorn,
//     jungfrau,
//     lyskamm,
//     matterhorn,
//     monch,
//     monte_rosa,
//     ober_gabelhorn,
//     rimpfischhorn,
//     schreckhorn,
//     strahlhorn,
//     weisshorn,
//     zinalrothorn,
// ];

////////////////////////////////////////////////////////////////////////////////

// filter the array and return an array with only the mountains with a height higher than 4250
// hint: use the methods: into_iter filter and collect
pub fn swiss_mountains_higher_than_4250(sm: [SwissMountain; 20]) -> Vec<SwissMountain> {
    sm.into_iter().filter(|sm| sm.height > 4250).collect()
}

// sort the mountains by height from smaller to tallest
// hint: you could convert the array into a vector using into_iter and collect and then use the sort_by method
pub fn swiss_mountains_sort_by_height(sm: [SwissMountain; 20]) -> Vec<SwissMountain> {
    let mut sm_vec: Vec<SwissMountain> = sm.into_iter().collect();
    sm_vec.sort_by(|a, b| a.height.cmp(&b.height));
    sm_vec
}

// 1 exercie on map

// compute the total height of all the mountains in highest_swiss_mountains using the iterator method for_each
pub fn swiss_mountains_total_height_for_each(sm: [SwissMountain; 20]) -> u32 {
    let mut total_height = 0;
    sm.iter().for_each(|x| total_height += x.height);
    total_height
}

// compute the total height of all the mountains in highest_swiss_mountains using the iterator method fold
pub fn swiss_mountains_total_height_fold(sm: [SwissMountain; 20]) -> u32 {
    let total_height: u32 = sm.iter().fold(0, |acc, e| acc + e.height);
    total_height
}

// find and return the SwissMountain with a height of 4505 using iter, find and cloned method
pub fn swiss_mountains_4505(sm: [SwissMountain; 20]) -> Option<SwissMountain> {
    let sm_4505 = sm.iter().find(|&x| x.height == 4505).clone();
    sm_4505.cloned()
}

// return a vector of swissmountain without the mountain having a height of 4110
pub fn swiss_mountains_wo_4110(sm: [SwissMountain; 20]) -> Vec<SwissMountain> {
    sm.iter().filter(|x| x.height != 4110).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_10() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let answer = 55;
        assert_eq!(total(input), answer);
    }

    #[test]
    fn test_any_positive_number_false() {
        let input = [0, -2, -999, -43, -3, -832, -456, -134, -549, -450];
        assert_eq!(any_positive_number(input), false);
    }
    #[test]
    fn test_any_positive_number_true() {
        let input = [0, 2, -999, -43, -3, -832, -456, -134, -549, -450];
        assert_eq!(any_positive_number(input), true);
    }

    #[test]
    fn test_even_empty() {
        let input = vec![1, 3, 5, 7, 9];
        assert_eq!(even(input), Vec::new());
    }

    #[test]
    fn test_even_2_4() {
        let input = vec![0, 1, 3, 5, 7, 9, 2, 4];
        let answer = vec![2, 4];
        assert_eq!(even(input), answer);
    }

    #[test]
    fn test_find_max_empty() {
        assert_eq!(find_max(Vec::new()), None);
    }
    #[test]
    fn test_find_max_nine() {
        let input = vec![0, 1, 3, 5, 7, 9];
        assert_eq!(find_max(input), Some(9));
    }

    #[test]
    fn test_iter_multiplied_by_two() {
        let input = [1, 2, 3, 4].into_iter();
        let answer = vec![2, 4, 6, 8];
        assert_eq!(iter_multiplied_by_two(input).collect::<Vec<_>>(), answer);
    }

    #[test]
    fn test_sum_of_squares_empty() {
        assert_eq!(sum_of_squares(Vec::new()), 0);
    }

    #[test]
    fn test_sum_of_squares() {
        let input: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let answer = 285;
        assert_eq!(sum_of_squares(input), answer);
    }

    #[test]
    fn test_is_max_empty() {
        assert_eq!(is_max(Vec::new(), 10), false);
    }

    #[test]
    fn test_sum_of_squares_false() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let max = 10;
        let answer = false;
        assert_eq!(is_max(input, max), answer);
    }

    #[test]
    fn test_sum_of_squares_true() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let max = 9;
        let answer = true;
        assert_eq!(is_max(input, max), answer);
    }
    #[test]
    fn test_swiss_mountains_higher_than_4250() {
        let aletschhorn = SwissMountain {
            name: String::from("Aletschhorn"),
            height: 4194,
        };
        let alphubel = SwissMountain {
            name: String::from("Alphubel"),
            height: 4206,
        };
        let breithorn = SwissMountain {
            name: String::from("Breithorn"),
            height: 4160,
        };
        let dent_blanche = SwissMountain {
            name: String::from("Dent Blanche"),
            height: 4357,
        };
        let dent_dherens = SwissMountain {
            name: String::from("Dent d'Hérens"),
            height: 4173,
        };
        let dom = SwissMountain {
            name: String::from("Dom"),
            height: 4546,
        };
        let finsteraarhorn = SwissMountain {
            name: String::from("Finsteraarhorn"),
            height: 4274,
        };
        let grand_combin = SwissMountain {
            name: String::from("Grand Combin"),
            height: 4309,
        };
        let gross_fiescherhorn = SwissMountain {
            name: String::from("Gross Fiescherhorn"),
            height: 4049,
        };
        let jungfrau = SwissMountain {
            name: String::from("Jungfrau"),
            height: 4158,
        };
        let lyskamm = SwissMountain {
            name: String::from("Lyskamm"),
            height: 4532,
        };
        let matterhorn = SwissMountain {
            name: String::from("Matterhorn"),
            height: 4478,
        };
        let monch = SwissMountain {
            name: String::from("Mönch"),
            height: 4110,
        };
        let monte_rosa = SwissMountain {
            name: String::from("Monte Rosa"),
            height: 4634,
        };
        let ober_gabelhorn = SwissMountain {
            name: String::from("Ober Gabelhorn"),
            height: 4063,
        };
        let rimpfischhorn = SwissMountain {
            name: String::from("Rimpfischhorn"),
            height: 4199,
        };
        let schreckhorn = SwissMountain {
            name: String::from("Schreckhorn"),
            height: 4078,
        };
        let strahlhorn = SwissMountain {
            name: String::from("Strahlhorn"),
            height: 4190,
        };
        let weisshorn = SwissMountain {
            name: String::from("Weisshorn"),
            height: 4505,
        };
        let zinalrothorn = SwissMountain {
            name: String::from("Zinalrothorn"),
            height: 4221,
        };

        let input = [
            aletschhorn.clone(),
            alphubel.clone(),
            breithorn.clone(),
            dent_blanche.clone(),
            dent_dherens.clone(),
            dom.clone(),
            finsteraarhorn.clone(),
            grand_combin.clone(),
            gross_fiescherhorn.clone(),
            jungfrau.clone(),
            lyskamm.clone(),
            matterhorn.clone(),
            monch.clone(),
            monte_rosa.clone(),
            ober_gabelhorn.clone(),
            rimpfischhorn.clone(),
            schreckhorn.clone(),
            strahlhorn.clone(),
            weisshorn.clone(),
            zinalrothorn.clone(),
        ];
        let answer = vec![
            dent_blanche,
            dom,
            finsteraarhorn,
            grand_combin,
            lyskamm,
            matterhorn,
            monte_rosa,
            weisshorn,
        ];
        assert_eq!(swiss_mountains_higher_than_4250(input), answer);
    }

    #[test]
    fn test_swiss_mountains_sort_by_height() {
        let aletschhorn = SwissMountain {
            name: String::from("Aletschhorn"),
            height: 4194,
        };
        let alphubel = SwissMountain {
            name: String::from("Alphubel"),
            height: 4206,
        };
        let breithorn = SwissMountain {
            name: String::from("Breithorn"),
            height: 4160,
        };
        let dent_blanche = SwissMountain {
            name: String::from("Dent Blanche"),
            height: 4357,
        };
        let dent_dherens = SwissMountain {
            name: String::from("Dent d'Hérens"),
            height: 4173,
        };
        let dom = SwissMountain {
            name: String::from("Dom"),
            height: 4546,
        };
        let finsteraarhorn = SwissMountain {
            name: String::from("Finsteraarhorn"),
            height: 4274,
        };
        let grand_combin = SwissMountain {
            name: String::from("Grand Combin"),
            height: 4309,
        };
        let gross_fiescherhorn = SwissMountain {
            name: String::from("Gross Fiescherhorn"),
            height: 4049,
        };
        let jungfrau = SwissMountain {
            name: String::from("Jungfrau"),
            height: 4158,
        };
        let lyskamm = SwissMountain {
            name: String::from("Lyskamm"),
            height: 4532,
        };
        let matterhorn = SwissMountain {
            name: String::from("Matterhorn"),
            height: 4478,
        };
        let monch = SwissMountain {
            name: String::from("Mönch"),
            height: 4110,
        };
        let monte_rosa = SwissMountain {
            name: String::from("Monte Rosa"),
            height: 4634,
        };
        let ober_gabelhorn = SwissMountain {
            name: String::from("Ober Gabelhorn"),
            height: 4063,
        };
        let rimpfischhorn = SwissMountain {
            name: String::from("Rimpfischhorn"),
            height: 4199,
        };
        let schreckhorn = SwissMountain {
            name: String::from("Schreckhorn"),
            height: 4078,
        };
        let strahlhorn = SwissMountain {
            name: String::from("Strahlhorn"),
            height: 4190,
        };
        let weisshorn = SwissMountain {
            name: String::from("Weisshorn"),
            height: 4505,
        };
        let zinalrothorn = SwissMountain {
            name: String::from("Zinalrothorn"),
            height: 4221,
        };

        let input = [
            aletschhorn.clone(),
            alphubel.clone(),
            breithorn.clone(),
            dent_blanche.clone(),
            dent_dherens.clone(),
            dom.clone(),
            finsteraarhorn.clone(),
            grand_combin.clone(),
            gross_fiescherhorn.clone(),
            jungfrau.clone(),
            lyskamm.clone(),
            matterhorn.clone(),
            monch.clone(),
            monte_rosa.clone(),
            ober_gabelhorn.clone(),
            rimpfischhorn.clone(),
            schreckhorn.clone(),
            strahlhorn.clone(),
            weisshorn.clone(),
            zinalrothorn.clone(),
        ];
        let answer = vec![
            gross_fiescherhorn.clone(),
            ober_gabelhorn.clone(),
            schreckhorn.clone(),
            monch.clone(),
            jungfrau.clone(),
            breithorn.clone(),
            dent_dherens.clone(),
            strahlhorn.clone(),
            aletschhorn.clone(),
            rimpfischhorn.clone(),
            alphubel.clone(),
            zinalrothorn.clone(),
            finsteraarhorn.clone(),
            grand_combin.clone(),
            dent_blanche.clone(),
            matterhorn.clone(),
            weisshorn.clone(),
            lyskamm.clone(),
            dom.clone(),
            monte_rosa.clone(),
        ];
        assert_eq!(swiss_mountains_sort_by_height(input), answer);
    }

    #[test]
    fn test_swiss_mountains_total_height_for_each() {
        let aletschhorn = SwissMountain {
            name: String::from("Aletschhorn"),
            height: 4194,
        };
        let alphubel = SwissMountain {
            name: String::from("Alphubel"),
            height: 4206,
        };
        let breithorn = SwissMountain {
            name: String::from("Breithorn"),
            height: 4160,
        };
        let dent_blanche = SwissMountain {
            name: String::from("Dent Blanche"),
            height: 4357,
        };
        let dent_dherens = SwissMountain {
            name: String::from("Dent d'Hérens"),
            height: 4173,
        };
        let dom = SwissMountain {
            name: String::from("Dom"),
            height: 4546,
        };
        let finsteraarhorn = SwissMountain {
            name: String::from("Finsteraarhorn"),
            height: 4274,
        };
        let grand_combin = SwissMountain {
            name: String::from("Grand Combin"),
            height: 4309,
        };
        let gross_fiescherhorn = SwissMountain {
            name: String::from("Gross Fiescherhorn"),
            height: 4049,
        };
        let jungfrau = SwissMountain {
            name: String::from("Jungfrau"),
            height: 4158,
        };
        let lyskamm = SwissMountain {
            name: String::from("Lyskamm"),
            height: 4532,
        };
        let matterhorn = SwissMountain {
            name: String::from("Matterhorn"),
            height: 4478,
        };
        let monch = SwissMountain {
            name: String::from("Mönch"),
            height: 4110,
        };
        let monte_rosa = SwissMountain {
            name: String::from("Monte Rosa"),
            height: 4634,
        };
        let ober_gabelhorn = SwissMountain {
            name: String::from("Ober Gabelhorn"),
            height: 4063,
        };
        let rimpfischhorn = SwissMountain {
            name: String::from("Rimpfischhorn"),
            height: 4199,
        };
        let schreckhorn = SwissMountain {
            name: String::from("Schreckhorn"),
            height: 4078,
        };
        let strahlhorn = SwissMountain {
            name: String::from("Strahlhorn"),
            height: 4190,
        };
        let weisshorn = SwissMountain {
            name: String::from("Weisshorn"),
            height: 4505,
        };
        let zinalrothorn = SwissMountain {
            name: String::from("Zinalrothorn"),
            height: 4221,
        };

        let input = [
            aletschhorn,
            alphubel,
            breithorn,
            dent_blanche,
            dent_dherens,
            dom,
            finsteraarhorn,
            grand_combin,
            gross_fiescherhorn,
            jungfrau,
            lyskamm,
            matterhorn,
            monch,
            monte_rosa,
            ober_gabelhorn,
            rimpfischhorn,
            schreckhorn,
            strahlhorn,
            weisshorn,
            zinalrothorn,
        ];
        let answer = 85436;
        assert_eq!(swiss_mountains_total_height_for_each(input), answer);
    }

    #[test]
    fn test_swiss_mountains_total_height_fold() {
        let aletschhorn = SwissMountain {
            name: String::from("Aletschhorn"),
            height: 4194,
        };
        let alphubel = SwissMountain {
            name: String::from("Alphubel"),
            height: 4206,
        };
        let breithorn = SwissMountain {
            name: String::from("Breithorn"),
            height: 4160,
        };
        let dent_blanche = SwissMountain {
            name: String::from("Dent Blanche"),
            height: 4357,
        };
        let dent_dherens = SwissMountain {
            name: String::from("Dent d'Hérens"),
            height: 4173,
        };
        let dom = SwissMountain {
            name: String::from("Dom"),
            height: 4546,
        };
        let finsteraarhorn = SwissMountain {
            name: String::from("Finsteraarhorn"),
            height: 4274,
        };
        let grand_combin = SwissMountain {
            name: String::from("Grand Combin"),
            height: 4309,
        };
        let gross_fiescherhorn = SwissMountain {
            name: String::from("Gross Fiescherhorn"),
            height: 4049,
        };
        let jungfrau = SwissMountain {
            name: String::from("Jungfrau"),
            height: 4158,
        };
        let lyskamm = SwissMountain {
            name: String::from("Lyskamm"),
            height: 4532,
        };
        let matterhorn = SwissMountain {
            name: String::from("Matterhorn"),
            height: 4478,
        };
        let monch = SwissMountain {
            name: String::from("Mönch"),
            height: 4110,
        };
        let monte_rosa = SwissMountain {
            name: String::from("Monte Rosa"),
            height: 4634,
        };
        let ober_gabelhorn = SwissMountain {
            name: String::from("Ober Gabelhorn"),
            height: 4063,
        };
        let rimpfischhorn = SwissMountain {
            name: String::from("Rimpfischhorn"),
            height: 4199,
        };
        let schreckhorn = SwissMountain {
            name: String::from("Schreckhorn"),
            height: 4078,
        };
        let strahlhorn = SwissMountain {
            name: String::from("Strahlhorn"),
            height: 4190,
        };
        let weisshorn = SwissMountain {
            name: String::from("Weisshorn"),
            height: 4505,
        };
        let zinalrothorn = SwissMountain {
            name: String::from("Zinalrothorn"),
            height: 4221,
        };

        let input = [
            aletschhorn,
            alphubel,
            breithorn,
            dent_blanche,
            dent_dherens,
            dom,
            finsteraarhorn,
            grand_combin,
            gross_fiescherhorn,
            jungfrau,
            lyskamm,
            matterhorn,
            monch,
            monte_rosa,
            ober_gabelhorn,
            rimpfischhorn,
            schreckhorn,
            strahlhorn,
            weisshorn,
            zinalrothorn,
        ];
        let answer = 85436;
        assert_eq!(swiss_mountains_total_height_fold(input), answer);
    }

    #[test]
    fn test_swiss_mountains_wo_4110() {
        let aletschhorn = SwissMountain {
            name: String::from("Aletschhorn"),
            height: 4194,
        };
        let alphubel = SwissMountain {
            name: String::from("Alphubel"),
            height: 4206,
        };
        let breithorn = SwissMountain {
            name: String::from("Breithorn"),
            height: 4160,
        };
        let dent_blanche = SwissMountain {
            name: String::from("Dent Blanche"),
            height: 4357,
        };
        let dent_dherens = SwissMountain {
            name: String::from("Dent d'Hérens"),
            height: 4173,
        };
        let dom = SwissMountain {
            name: String::from("Dom"),
            height: 4546,
        };
        let finsteraarhorn = SwissMountain {
            name: String::from("Finsteraarhorn"),
            height: 4274,
        };
        let grand_combin = SwissMountain {
            name: String::from("Grand Combin"),
            height: 4309,
        };
        let gross_fiescherhorn = SwissMountain {
            name: String::from("Gross Fiescherhorn"),
            height: 4049,
        };
        let jungfrau = SwissMountain {
            name: String::from("Jungfrau"),
            height: 4158,
        };
        let lyskamm = SwissMountain {
            name: String::from("Lyskamm"),
            height: 4532,
        };
        let matterhorn = SwissMountain {
            name: String::from("Matterhorn"),
            height: 4478,
        };
        let monch = SwissMountain {
            name: String::from("Mönch"),
            height: 4110,
        };
        let monte_rosa = SwissMountain {
            name: String::from("Monte Rosa"),
            height: 4634,
        };
        let ober_gabelhorn = SwissMountain {
            name: String::from("Ober Gabelhorn"),
            height: 4063,
        };
        let rimpfischhorn = SwissMountain {
            name: String::from("Rimpfischhorn"),
            height: 4199,
        };
        let schreckhorn = SwissMountain {
            name: String::from("Schreckhorn"),
            height: 4078,
        };
        let strahlhorn = SwissMountain {
            name: String::from("Strahlhorn"),
            height: 4190,
        };
        let weisshorn = SwissMountain {
            name: String::from("Weisshorn"),
            height: 4505,
        };
        let zinalrothorn = SwissMountain {
            name: String::from("Zinalrothorn"),
            height: 4221,
        };

        let input = [
            aletschhorn.clone(),
            alphubel.clone(),
            breithorn.clone(),
            dent_blanche.clone(),
            dent_dherens.clone(),
            dom.clone(),
            finsteraarhorn.clone(),
            grand_combin.clone(),
            gross_fiescherhorn.clone(),
            jungfrau.clone(),
            lyskamm.clone(),
            matterhorn.clone(),
            monch.clone(),
            monte_rosa.clone(),
            ober_gabelhorn.clone(),
            rimpfischhorn.clone(),
            schreckhorn.clone(),
            strahlhorn.clone(),
            weisshorn.clone(),
            zinalrothorn.clone(),
        ];

        let answer = vec![
            aletschhorn.clone(),
            alphubel.clone(),
            breithorn.clone(),
            dent_blanche.clone(),
            dent_dherens.clone(),
            dom.clone(),
            finsteraarhorn.clone(),
            grand_combin.clone(),
            gross_fiescherhorn.clone(),
            jungfrau.clone(),
            lyskamm.clone(),
            matterhorn.clone(),
            monte_rosa.clone(),
            ober_gabelhorn.clone(),
            rimpfischhorn.clone(),
            schreckhorn.clone(),
            strahlhorn.clone(),
            weisshorn.clone(),
            zinalrothorn.clone(),
        ];
        assert_eq!(swiss_mountains_wo_4110(input), answer);
    }
}
