fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.
    // to check Option is borrowed when it goes in function or not?

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                        Enums                                        //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°1
    // create an enum Season with four possible variants (Winter, Spring, Summer, Autumn)

    // Exercise n°2
    // create an enum Month with twelve possible values (January, February, March, April, May, June, July, August, September, October, November, December)
    // create a variable called four_month of the instance Month with the variant April.
    // create a variable called october of the instance Month with the variant August.

    // Exercise n°3
    // create an enum Shape with three variants (Rectangle, Square, Circle).
    // associate an anonymous struct to Rectangle {base: u32, height, u32}, a u32 to Square (that will represent its side) and a u32 to Circle (that will represent its radius)


    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       Option                                        //////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    
    // Reminder: Option is an enum defined as:
    // enum Option<T> {}
    //     Some(T),
    //     None,
    // }
    // We will learn more about generics in Chapter 10. For now think of <T> in Option<T> as representing any type. 
    // It serves as a stand-in suggesting its capability to accommodate any type. 
    // Once the type is clearly specified, it will anticipate an option of that particular type
    // e.g. pub fn option_is_some(op: Option<u32>) -> bool{...} expects an Option<u32>

    // Exercise n°
    // create the variable some_char that is an Option<char> containing the character 'b'.

    // Exercise n°
    // create the variable no_char of type Option<char> containing None.

    // Exercise n°
    // create the variable number_of_beer that is an Option<u32> containing the value 3.

    // Exercise n°
    // create a mutable variable score that is an Option<i32> containing the value None. 
    // This could represent a scenario where the score change depending on your performance in the game. The None value could indicate that the game has not started yet


    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                        Match                                        //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°
    // create an enum TrafficLight with three variants (Green, Yellow, Red).
    // create a mutable variable called current_traffic_light_color of the variant Yellow.
    // Use a match statement with the variable current_traffic_light_color that print the following to the terminal depending of the variant:
    // Red: "Stop!", Yellow: "Prepare to stop!", Green: "Go!"

    //Exercise n°
    // create an enum Direction with four variants: Up, Down, Left, and Right, each associated with an u32 (representing the distance).
    // create a function named direction_value that takes a variable of type Direction and returns the direction contained by each variant.
    // if the variant is Down it should also print to the terminal "I'm waiting on the good times now".

    // Exercise n°
    // create a method perimeter for the enum Shape that will return the perimeter of the shape as a u32
    // i.e. for rectangle = 2*(base + height), for the square = 4*side and for the circle = 2 * 3* radius
    // use a match statement

    // Exercise n°
    // create a function that will take as input an enum Shape and returns the area of the shape as a u32
    // i.e. for rectangle = base * height, for the square = side * side and for the circle = 3 * radius * radius
    // use a match statement

    // Exercise n°
    // create the function one_more that takes an Option<u32> and returns an Option<u32>.
    // This function increment the value by one or returns None.
    // shadowed the variable number_of_beer (created previously) by using the this function with number_of_beer as argument.

    // Exercise n°
    // create a match expression that takes the variable number_of_beer. Using the match statement 
    // if it contains the value 4 it should print to the terminal "pretty unlucky number, should take one more" and do nothing otherwise.


    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       if let                                        //////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    
    // Exercise n°
    // create a match expression that takes the variable number_of_beer. 
    // If it contains the value 4 it should print to the terminal "pretty unlucky number, should take one more" and do nothing otherwise.
    // This time use the if let syntax. 

    // Exercise n°
    // using the if let syntax your code should print to the terminal "It is a rectangle" if the variable shape is a rectangle and "Not a rectangle" in the other cases
    
}
