fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    // Ex 1: Recreate a simple example of one thread sending the message "Hello" from one thread to another using std::sync::mpsc
    // and print the message you receive. For simplicity sake use unwrap()

    // Ex 2: Using you solution above make some modification to send multiple messages using a for loop

    // Ex 3: Create two threads that will send multiple messages to the receiver by cloning the transmitting half

    // Ex 4: Create a simple example using a Mutex by creating an Mutex<i32> = 42 and changing its value using lock to

    // Ex 5: Spawn 10 threads that will each increment a shared counter. Use Mutex<T> and Arc<T>.
}

#[cfg(test)]
mod tests {
    mod tests {
        #[test]
        fn test_is_equal() {
            todo!()
        }
        #[test]
        fn test_add_three() {
            todo!()
        }
    }
}
