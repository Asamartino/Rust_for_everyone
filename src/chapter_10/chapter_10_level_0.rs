fn main() {
    /// To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.
    
    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                      Generics                                       //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°2
    // create a generic struct Coordinates with two fields x and y of both the same type.

    // Exercise n°3
    // create a generic struct Position with two fields x and y of both the same type. This time use a another letter than <T>

    // Exercise n°4
    // create a generic struct Measurement with two fields age and height that can have different types

    // Exercise n°5    
    // define a generic method called self_x on Coordinates that returns the value of x

    // Exercise n°6
    // define a method total_distance method on Coordinates for the type u32
    // it should return the addition of the two fields x+y

    // Exercise n°7
    // create a method fusion for the Mutant struct define below.
    // This method, should take another Mutant as parameter which might have different type than the self Mutant we're calling fusion on. 
    // It will create a new Mutant with the x value from the self Mutant and the y value from the passed-in Mutant.
    struct Mutant<T,U>{
        first_half: T,
        second_half: U,
    }
    

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                      Traits                                         //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°8
    // consider the below code and implement the Summarize trait for Cat. It should return: "This cat name is {} and have {} number of live(s)"
    pub struct Cat{
        name: String,
        number_of_lives: u32,
    }
    pub struct Duck{
        name: String,
        can_fly: bool,
    }

    pub struct Goose{
        name: String,
        can_fly: bool,
        golden_eggs: bool,
    }
    pub trait Summary{
        fn summarize(&self) -> String;
    }
    
    // Exercise n°9
    // implement the Summary trait for Duck. It should return: "This duck name is {}"


    // Exercise n°10
    // create a Trait Fly with method flying that use a default implementation.
    // It should return a string: "Fly, so high... direction sky"
    // Implement this trait for duck

    // Exercise n°11
    // implement the trait Fly for Goose
    // Override the default implementation it should return the following string: "what an untitled goose"


    // Exercise n°12
    // Consider the below code: 
    pub trait Rich{
        fn too_rich(&self) -> String{
            String::from("I'm sorry, is this some peasant joke that I'm too rich to understand?")
        }
    }
    impl Rich for Goose {}
    // Create a generic function that will accept any type that implement Fly and Rich. 
    // This function should return the following string: "it's flying and it's rich... probably a canadian goose"


    // Scrooge McDuck

   
    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                     Lifetimes                                       //////
    /////////////////////////////////////////////////////////////////////////////////////////////////
}
