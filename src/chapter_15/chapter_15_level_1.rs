// create the struct MyOwnBox<T>(T) and add the deref trait on it




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_positive_number_false() {
        let input = [0, -2, -999, -43, -3, -832, -456, -134, -549, -450];
        assert_eq!(any_positive_number(input), false);
    }

}
