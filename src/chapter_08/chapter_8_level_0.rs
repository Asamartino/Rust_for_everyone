fn main() {
    // To familiarize you with the syntax, solutions to the following exercises are provided in a separate file.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                        Vectors                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°1
    // create an empty vector called vec that will contain i32 values

    // Exercise n°2
    // shadow the vec variable that will now contain the char 'a', 'b' and 'c' using the vec! macro

    // Exercise n°3
    // create an empty vector called counting_sheep that will contain u32 values. Add the number 1,2,3,4,5 using the push method

    // Exercise n°
    // create a variable called sheep_number_three and assign it the third element of counting_sheep using the indexing syntax

    // Exercise n°
    // shadow the above variable and get the fifth element of counting_sheep using the get method.

    // Exercise n°
    // iterate over all the values of counting_sheep and print the element with the word sheep next to it
    // the following should be printed to your terminal: 1 sheep, 2 sheep, 3 sheep, 4 sheep, 5 sheep,

    // Exercise n°
    // create a mutable vector and initialized it to 50,100,150,200.
    // increment each value by 50 and print the resulting vector.

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                       String                                        //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°
    // create a mutable empty string called john

    // Exercise n°
    // update the value of john to "john" using the to_string method

    // Exercise n°
    // update the value of john to "johnny" using the String::from function

    // Exercise n°
    // add a whitespace at the end of "johnny" using the method push

    // Exercise n°
    // add the word " bravo" to variable john using the push_str method

    // Exercise n°
    // using the format! create the variable eeny_meeny_miny_moe by concatenating the below variables
    let eeny = String::from("Eeny");
    let meeny = "Meeny".to_string();
    let miny = "Miny".to_string();
    let moe = String::from("Moe");

    
    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////                                      Hash Maps                                      //////
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Exercise n°
    // create an empty mutable hashmap called balance

    // Exercise n°
    // insert the key "Alice" (a String) and the value 100 into balance using the insert method. Do the same with the key "Bob" with a value of 50
    
    // Exercise n°
    // create a variable called alice_balance and get the value associated with the key "Alice" in balance using the get method.

    // Exercise n°
    // replace the value associated with the key "Bob" to 75 using the entry and or_insert method.

    // Exercise n°
    // iterate over the hashmap balances using a for loop and print to the terminal "the balance of key is value" for each key and value inside balances.

    // Exercise n°
    // count how many times each word appears in the below variable using a hashmap and print the hashmap in the terminal
    let eeny_meeny_miny_moe = "eeny meeny miny moe catch a tiger by the toe if he hollers let him go eeny meeny miny moe";

}
