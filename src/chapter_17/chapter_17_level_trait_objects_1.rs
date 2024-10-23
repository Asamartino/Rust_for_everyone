 // Ex 1 Uncomment the perimeter function in the trait Area and complete the code below so that it compiles by only adding code and not deleting any lines

pub trait Area {
    fn area(&self) -> u32;
    // fn perimeter(&self) -> u32;
}

pub struct Shape {
    pub vec_shapes: Vec<Box<dyn Area>>,
}

impl Shape {
    pub fn calculate_total_area(&self) -> u32 {
        let mut total_area = 0;
        for shapes in self.vec_shapes.iter() {
            total_area += shapes.area();
        }
        total_area
    }
}

pub struct Circle {
    radius: u32,
}
impl Area for Circle {
    fn area(&self) -> u32 {
        self.radius * self.radius * 3
    }
}

pub struct Square {
    side: u32,
}
impl Area for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn main() {
    let shapes = Shape {
        vec_shapes: vec![
            Box::new(Circle { radius: 3 }),
            Box::new(Square { side: 10 }),
        ],
    };
    println!("the total of the area in shapes is: {}", shapes.calculate_total_area());
}

