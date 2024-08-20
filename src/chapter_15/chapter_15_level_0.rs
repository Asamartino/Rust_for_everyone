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
    //      complete here
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
    //     
    // }


    // Ex 8
    // As you know RefCell<T> lets you have many immutable borrows and only one mutable borrow. Modify the borrow function in the code above
    // and make two mutable references in the same scope (i.e. create two variables each containing a RefMut<T> by using borrow_mut())
    // The code should not compile. Familiarize yourself with the error message and note that this happens at runtime.

    // Ex 9 
    // As you know it is challenging to create memory leak in Rust but not impossible.
    // Create a reference cycle by completing the below example

    // use std::cell::RefCell;
    // use std::rc::Rc;
    //
    // struct Node {
    //     value: i32,
    //     next: Option<Rc<RefCell<Node>>>,
    // }
    //
    // fn main() {
    //     // Create two Rc, RefCell wrapped Node instances
    //     let node1 = Rc::new(RefCell::new(Node {
    //         value: 1,
    //         next: None,
    //     }));
    //     let node2 = todo!();
    //     // Create a reference cycle node1.next() should reference node2 and vice versa. Use the borrow_mut method
    //     node1.borrow_mut().next = Some(Rc::clone(&node2));
    //     node2.borrow_mut().next = todo();

    //     // At this point, the reference count of node1 and node2 is 2 due to the cycle:
    //     println!("node1 strong count: {}", Rc::strong_count(&node1)); 
    //     println!("node2 strong count: {}", Rc::strong_count(&node2));
    // }
    // At this point the reference count of node 1 and node2 is 1 rather than 0 -> so there is memory on the heap that won't be dropped = memory leak

    // Ex 10
    // We can prevent reference cycle by using weak reference
    // A perfect use case for a weak reference is when: a parent node own its children, so if it is dropped so are its children.
    // However, a child doesn't own its parent, so if it is dropped the parent should still exists
    // Recreate the example similar to the one used in the book

    //     use std::cell::RefCell;
    // use std::rc::{Rc, Weak};

    // #[derive(Debug)]
    // struct Node {
    //     value: i32,
    //     parent: todo!(), // use a weak reference
    //     children: todo!(),
    // }

    // fn main() {
    //     // the leaf variable is an Rc wrapped Node
    //     let leaf = todo!(),

    //     println!(
    //         "leaf: strong_count = {}, weak_count = {}",
    //         Rc::strong_count(&leaf),
    //         Rc::weak_count(&leaf),
    //     );

    //     {
    //         // the branch variable is an Rc wrapped Node with its children being leaf
    //         let branch = todo!();
            
    //         // assign branch as the parent of leaf 
            

    //         println!(
    //             "branch: strong_count = {}, weak_count = {}",
    //             Rc::strong_count(&branch),
    //             Rc::weak_count(&branch),
    //         );

    //         println!(
    //             "leaf: strong_count = {}, weak_count = {}",
    //             Rc::strong_count(&leaf),
    //             Rc::weak_count(&leaf),
    //         );
    //     }

    //     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //     println!(
    //         "leaf: strong_count = {}, weak_count = {}",
    //         Rc::strong_count(&leaf),
    //         Rc::weak_count(&leaf),
    //     );
    // }

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
