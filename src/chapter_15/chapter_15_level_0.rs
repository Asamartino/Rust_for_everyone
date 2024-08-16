use std::arch::x86_64::_MM_PERM_BABA;

fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    // Ex 1: Create an enum for a cons list. This list should store the value 4,5 and 6. Don't use Box<T> and try to recreate
    // the error "recursive type has infinite size"

    // Ex 2: Fix the code you created above by using Box<T>

    // Ex 3: Make the below code compile by adding one character
    // let a = 42;
    // let b = &a;

    // assert_eq(a, 42);
    // assert_eq(b, 42);

    // Ex 4: Create the struct MyBox<T> similar to what was introduced in the book. Implement the Deref trait and the deref method.
    // Find a way to show that *a is the same behind the scene as *(a.deref()) (you could use println!, assert_eq! or any other way).

    // Ex 5: The below code shows you deref coercion at work. Explain what steps that are taken and each type conversion
    // fn say_hi(name: &str){
    //      println!("hi {}", name);
    // }

    // fn main(){
    //      let box_georges = Box::new(String::from(Georges));
    //      say_hi(&box_georges);
    // }

    // Ex 6:
    // Complete the code in the main function below using Rc<T>, create a cons list that conceptually looks like the figure below

    //         +-------+
    //   b ->  | 1 |   |
    //         +-------+
    //                 ↘ +-------+    +-------+      +-----+
    //             a  -> | 2 |   | -> | 22 |  |  ->  | Nil |
    //                   +-------+    +-------+      +-----+
    //                 ↗
    //         +-------+
    //   c ->  | 3 |   |
    //         +------\+
    //

    //use List::{Cons, Nil};
    // enum List {
    //     Cons(i32, Rc<List>),
    //     Nil,
    // }
    // use std::rc::Rc;
    //
    // fn main() {
    // complete here
    // }

    // Ex 7
    // Suppose you have the trait Library that defines the function signatures borrow. You want to be able to increase the borrowed_counter of Book
    // every time the function borrow has been called. Note that:
    //      - you can't modify a book instance because the send method takes an immutable reference.
    //      - you can't change the borrow function signature
    // -> This is a perfect use case for RefCell<T>.
    // Modify the type of borrowed_counter of Book to RefCell<u32> and complete the borrow function so that the below code compiles
    //
    // use std::cell::RefCell;
    // pub trait Library {
    //      fn borrow(&self);
    //   }
    //
    // struct Book {
    //     title: String,
    //     borrowed_counter: u32,
    // }
    //
    // impl Library for Book {
    // fn borrow(&self) {
    //         //to complete using borrow_mut() (learn more here: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow_mut)
    //    }
    // }
    //
    // fn main() {
    //     let book = Book {
    //         title: String::from("I love Rust"),
    //         borrowed_counter: RefCell::new(0),
    //     };
    //     book.borrow();
    //     book.borrow();
    // }


    // Ex 8
    // As you know RefCell<T> lets you have many immutable borrows and only one mutable borrow. Modify the borrow function in the code above
    // and make two mutable references in the same scope (i.e. create two variables each containing a RefMut<T> by using borrow_mut())
    // The code should not compile. Familiarize yourself with the error message and note that this happens at runtime.
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
