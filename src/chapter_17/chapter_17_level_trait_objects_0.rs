 // Ex 1 Look at the code below, and uncomment it.Complete the struct Shape using a trait object
 // Ex 2 Create the struct Rectangle and add it to the vector created in the main() function. Notice that you will get a compilation error as Rectangle does not implement the trait Area.
 // Ex 3 Implement the trait Area for Rectangle so that this program compiles

// pub trait Area {
//     fn area(&self) -> u32;
// }

// pub struct Shape {
//     todo!(),
// }

// impl Shape {
//     pub fn calculate_total_area(&self) -> u32 {
//         let mut total_area = 0;
//         for shapes in self.vec_shapes.iter() {
//             total_area += shapes.area();
//         }
//         total_area
//     }
// }

// pub struct Circle {
//     radius: u32,
// }
// impl Area for Circle {
//     fn area(&self) -> u32 {
//         self.radius * self.radius * 3
//     }
// }

// pub struct Square {
//     side: u32,
// }
// impl Area for Square {
//     fn area(&self) -> u32 {
//         self.side * self.side
//     }
// }


// fn main() {
//     let shapes = Shape {
//         vec_shapes: vec![
//             Box::new(Circle { radius: 3 }),
//             Box::new(Square { side: 10 }),
//             // add Box::new(Rectangle {...}), here 
//         ],
//     };
//     println!("the total of the area in shapes is: {}", shapes.calculate_total_area());
// }

