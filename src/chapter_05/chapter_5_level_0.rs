fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    // Exercise n°1
    // create a struct named User with the following fields: username of type String, email of type String, active of type bool, and sign_in_counter of type u32

    // Exercise n°2
    // create an instance of the structure User and initialized with the value you want.

    // Exercise n°3
    // create a mutable instance of this struct, initialized with the value you want. Modify the field active from true to false or vice versa

    // Exercise n°4
    // complete the below function that returns a User from the provided username and email
    // default value for active is true and sign_in_counter is 1
    fn build_user(username: String, email: String) -> User {
        todo!()
    }

    //Exercise n°5
    //complete the below function that returns a User using the field init shorthand syntax
    //default value for active is true and sign_in_counter is 1
    fn build_user_shorthand(username: String, email: String) -> User {
        todo!()
    }

    // Exercise n°5 create an instance of User and use this instance to initate some fields in a second instance using the syntax ..

    // Exercise n°6 create struct Rectangle_3D with the fields length, width, and height. Those three field are of type u32

    // Exercise n°7 create the function rectangle_3D_volume that takes a Rectangle_3D and return the volume of this rectangle (volume = length*width*height)

    // Exercise n°8 implement the volume function as a method of the struct Rectangle_3D

    // Exercise n°9 implement another method called is_cube will which return true if the rectangle has the same height, width and height value

    // Exercise n°10 implement another method. This method called is_bigger which will compare the first rectangle volume with the volume of a second instance.
    // this function should return true only if the first rectangle volume is bigger thant the second
    // and false if they have the same volume or if the second volume is bigger than the first one
    // Tip: why not use a method you created before?
}
