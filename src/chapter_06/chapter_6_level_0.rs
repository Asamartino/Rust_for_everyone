fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    // Exercise n°1
    // create an enum Season with four possible variants (Winter, Spring, Summer, Autumn)

    // Exercise n°2
    // create an enum Month with twelve possible values (January, February, March, April, May, June, July, August, September, October, November, December)
    // create an variable called four_month of the instance month with the variant April
    // create a variable called eight_month of the instance month with the variant August

    // Exercise n°3
    // create an enum Shape with three variants (Rectangle, Square, Circle,).
    // associate an anonymous struct to Rectangle {base: u32, height, u32}, a u32 to Square (that will represent its side) and a u32 to Circle (that will represent its radius)


    // Exercise n°4
    // create an enum TrafficLight with three variants (Green, Yellow, Red).
    // use a match operator to match each possible variant and print the following to the terminal:
    // Red: "Stop!", Yellow: "Prepare to stop!", Green: "Go!"

    //Exercise n°5
    // create an enum Direction with four variants: Up, Down, Left, and Right, each associated with an u32 (representing the distance).
    // use a match operator that match the Direction enum and depending on the variants print to the console the direction and the associated distance.
    // for instance if the variable is Direction::Up(5) it would print "the direction is Up and the distance is 5".

    // Exercise n°6
    // create a method area for the enum Shape (define before) that will return the area of the shape as a u32
    // area for rectangle = base * height, for the square = side * side and for the circle = 3 * radius * radius
    // using a match statement

    // Exercise n°6
    // using the if let syntax your code should print to the terminal "four", if x = 4.

    // Exercise n°7
    // using the if let syntax your code should print to the terminal "It is a rectangle" if the variable shape is a rectangle and "Not a rectangle" in the other cases
    
}
