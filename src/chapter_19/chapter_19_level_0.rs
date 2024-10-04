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

    // Ex 8: Uncomment the code below and add lifetime annotation so it can compile
    // Note that even to the name of the lifetime you use can be the same the lifetime parameters aren't related
    // struct Context(&str);

    // struct Parser {
    //     context: &Context,
    // }

    // impl Parser{
    //     fn parse(&self) -> Result<(),&str>{
    //         Err(&self.context.0[1..])
    //     }
    // }

    //Ex 9: Uncomment the code below and add lifetime annotation so it can compile. 
    // struct Context(&str);

    // struct Parser {
    //     context: &Context,
    // }

    // impl Parser{
    //     fn parse(&self) -> Result<(),&str>{
    //         Err(&self.context.0[1..])
    //     }
    // }

    // fn parse_context(context: Context) -> Result<(),&str>{
    //      Parser{ context: &context}.parse()
    // }

    // Ex 10: uncomment the below code and add an explicit lifetime bound so that the reference type &'a T does not outlive the data it points at
    // struct Context(&str);

    // struct Parser {
    //     context: &Context,
    // }

    // impl Parser{
    //     fn parse(&self) -> Result<(),&str>{
    //         Err(&self.context.0[1..])
    //     }
    // }

    // fn parse_context(context: Context) -> Result<(),&str>{
    //      Parser{ context: &context}.parse()
    // }

    // Ex 11: recreate the trait Iterator using associated types. Recall that you need to define the type of element being iterated over and the next() function
    // Why is the use of associated types different from using generics?


    // Ex 12: Overload the operator add so that you can add two struct Position together. Uncomment and complete the code below:
    // use std::ops::Add;

    //#[derive(Debug, PartialEq)]
    // struct Position{
    //     x: i32,
    //     y, i32
    // }

    // impl Add for Position{
    //     todo!()
    // }

    // Ex 13: Uncomment and complete the code below so that you can add values in millimeters to values in meters
    // use std::ops::Add;

    // struct Millimeters(u32);
    // struct Meters(u32);

    // impl Add<Meters> for Millimeters{
    //     todo!();
    // }

    // Ex: 14 Uncomment the below code and create the trait Car that is a supertrait of Vehicle
    // Defining a basic trait `Vehicle`
    // trait Vehicle {
    //     fn drive(&self);
    // }

    // struct MyCar;

    // impl Vehicle for MyCar {
    //     fn drive(&self) {
    //         println!("I'm driving my car \o/");
    //     }
    // }

    // impl Car for MyCar {
    //     fn honk(&self) {
    //         println!("My car can honk");
    //     }
    // }

    // let my_car = MyCar;
    // my_car.drive(); 
    // my_car.honk(); 

    Ex 15: Use t

}
