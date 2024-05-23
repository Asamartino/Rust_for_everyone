/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                      Traits                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// assert macro with true and false
// assert negate

// assert_eq
// asssert_ne

// add custom message

// should panic
// should panic with message to make it more precise

// test with flag  test-threads=1

// showing the output of pinrtln!

// exo combining --test-threads=1 with --nocapture

// panic with expected

// Exercise n°
// Order the test you created as you please adn run them all. As you might see they run in parallel.
// Use the test-threads=1 option to run them sequentially.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_pop() {}
}
