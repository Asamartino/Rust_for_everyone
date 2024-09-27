fn main() {
    // Ex 1: Create two raw pointers one immutable and one mutable. They should both point to the same value
    // Note that this is only possible with raw pointers

    // Ex 2: Dereference the two raw pointers created previously by printing the value they point to in the terminal
    // As you know you can only dereference a raw pointer in an unsafe block

    // Ex 3: Call/execute the following function
    unsafe fn danger() {
        println!("You can't be in danger if you are the danger")
    }

    // Ex 4: Complete the below function so that it compiles
    pub fn split_at_mut(slice: &mut [i32], ind: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        todo!()
    }

    // Ex 5: Call/execute the above function

    // Ex 6: Create a static variable and print its value in the terminal

    // Ex 7: Create a mutable static variable and update its value. Ensure that the value is updated by printing it in the terminal.
}
