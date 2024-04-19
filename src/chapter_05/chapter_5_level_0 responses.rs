fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.
    
    // Exercise n°1
    // create a struct named User with the following fields: username of type String, email of type String, active of type bool, and sign_in_counter of type u32
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_counter: u32,
    }

    // Exercise n°2
    // create an instance of the structure User and initialized with the value you want.
    let user1 = User {
        username: String::from("Ben_Dover"),
        email: String::from("ben.dover@fakemail.com"),
        active: true,
        sign_in_counter: 32,
    };

    // Exercise n°3
    // create a mutable instance of this struct, initialized with the value you want. Modify the field active from true to false or vice versa
    let mut user2 = User {
        username: String::from("Chris_P_Bacon"),
        email: String::from("chris.p.bacon@fakemail.com"),
        active: true,
        sign_in_counter: 10,
    };
    user2.active = !user2.active;

    // Exercise n°4
    // complete the below function that returns a User from the provided username and email
    // default value for active is true and sign_in_counter is 1
    fn build_user(username: String, email: String) -> User {
        User {
            username: username,
            email: email,
            active: true,
            sign_in_counter: 1,
        }
    }

    //Exercise n°5
    //complete the below function that returns a User using the field init shorthand syntax
    //default value for active is true and sign_in_counter is 1
    fn build_user_shorthand(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_counter: 1,
        }
    }

    // Exercise n°5 create an instance of User and initiate some fields from it from one of the previous instance you created
    let user3 = User {
        username: String::from("Pepe_Roni"),
        email: String::from("pepe.roni@fakemail.com"),
        ..user2
    };

    // Exercise n°6 create struct Rectangle3d with the fields length, width, and height. Those three field are of type u32
    struct Rectangle3d {
        length: u32,
        width: u32,
        height: u32,
    }

    // Exercise n°7 create the function rectangle_3d_volume that takes a Rectangle3d and return the volume of this rectangle (volume = length*width*height)
    fn rectangle_3d_volume(rect: Rectangle3d) -> u32 {
        rect.length * rect.width * rect.height
    }

    // Exercise n°8 implement the volume function as a method of the struct Rectangle_3D
    impl Rectangle3d {
        fn volume(&self) -> u32 {
            self.length * self.width * self.height
        }
    }

    // Exercise n°9 implement another method called is_cube will which return true if the rectangle has the same height, width and height value
    impl Rectangle3d {
        fn is_cube(&self) -> bool {
            self.length == self.width && self.width == self.height
        }
    }

    // Exercise n°10 implement another method. This method called is_bigger which will compare the first rectangle volume with the volume of a second instance.
    // this function should return true only if the first rectangle volume is bigger thant the second
    // and false if they have the same volume or if the second volume is bigger than the first one
    // Tip: why not use a method you created before?

    impl Rectangle3d {
        fn is_bigger(&self, other_rect: &Rectangle3d) -> bool {
            self.volume() > other_rect.volume()
        }
    }
}
