fn main(){

    // Create the struct MyOwnBox<T>(T). Then, define the new function similar to the new function described on Box<T>. 
    // Finally, implement the Deref trait on MyOwnBox<T> by implementing the deref method. 
    // The below code should compile after you uncomment it:
    
    // let a = 42;
    // let b = MyOwnBox::new(a);

    // assert_eq!(a, 42);
    // assert_eq!(*b, 42);


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_positive_number_false() {
        let input = [0, -2, -999, -43, -3, -832, -456, -134, -549, -450];
        assert_eq!(any_positive_number(input), false);
    }

}
