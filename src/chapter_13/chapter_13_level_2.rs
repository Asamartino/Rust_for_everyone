/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Ex Look at the below website. This website contains a list of streets in Geneva.
// Fetch the contain of this website and count how many street contain " du " anywhere in their name
// https://commons.wikimedia.org/wiki/Category:Streets_in_Geneva_by_name

// use reqwest::blocking::get;
// use select::document::Document;
// use select::predicate::Name;
// pub fn count_geneva_street_name_with_du() -> usize {
//     let response = get("https://commons.wikimedia.org/wiki/Category:Streets_in_Geneva_by_name").unwrap();
//     assert!(response.status().is_success());

//     let html = response.text().unwrap();

//     // Parse the HTML document
//     let document = Document::from(html.as_str());

//     // Extract street names from the HTML
//     let streets: Vec<String> = document
//         .find("mw-category-group")
//         .flat_map(|node| node.find(Name("a")).map(|n| n.text().trim().to_string()))
//         .collect();

//     // Count how many streets contain 'du' anywhere in their name
//     let count = streets.iter().filter(|&street| street.contains(" du ")).count();
//     count
    
// }


use std::collections::HashMap;

// Ex You will be provided with a vector SodaSize (as defined below).
// Sum up each instance: compute_soda_size(vec![SodaSize::Small, SodaSize::Medium]) -> {SodaSize::Small: 1, SodaSize::Medium: 1 }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SodaSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

// let sales = [
//     SodaSize::Large,
//     SodaSize::Medium,
//     SodaSize::Medium,
//     SodaSize::ExtraLarge,
//     SodaSize::ExtraLarge,
//     SodaSize::Large,
//     SodaSize::ExtraLarge,
//     SodaSize::ExtraLarge,
//     SodaSize::Large,
// ];
// Sum up the instances of each of these using the reduce method:
// e.g. -> (SodaSize::Large; 3),...
pub fn compute_soda_size(v: Vec<SodaSize>) -> HashMap<SodaSize, usize> {
    let mut hm = HashMap::new();
    v.into_iter().fold(hm, |mut acc, size| {
        match size {
            SodaSize::Small => *acc.entry(SodaSize::Small).or_insert(0) += 1,
            SodaSize::Medium => *acc.entry(SodaSize::Medium).or_insert(0) += 1,
            SodaSize::Large => *acc.entry(SodaSize::Large).or_insert(0) += 1,
            SodaSize::ExtraLarge => *acc.entry(SodaSize::ExtraLarge).or_insert(0) += 1,
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_count_geneva_street_name_with_du() {
    //     assert_eq!(count_geneva_street_name_with_du(), 5);
    // }


    #[test]
    fn test_compute_soda_size() {
        let input = vec![
            SodaSize::Large,
            SodaSize::Medium,
            SodaSize::Medium,
            SodaSize::ExtraLarge,
            SodaSize::ExtraLarge,
            SodaSize::Large,
            SodaSize::ExtraLarge,
            SodaSize::ExtraLarge,
            SodaSize::Large,
        ];
        let mut answer = HashMap::new();
        answer.insert(SodaSize::Large, 3);
        answer.insert(SodaSize::Medium, 2);
        answer.insert(SodaSize::ExtraLarge, 4);

        assert_eq!(compute_soda_size(input), answer);
    }

    #[test]
    fn test_compute_soda_size_empty() {
        let input = Vec::new();
        let answer = HashMap::new();
        assert_eq!(compute_soda_size(input), answer);
    }

    
}
