fn main(){ 
    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); 

    let s1 = String::from("hello🗝️");

    let (s2, len) = calculate_length(s1);
     //the ownership of s1 is moved to the function calculate_length, 
     //so it is not available after this point
    println!("The length of '{s2}' is {len}.");

    let mut s = String::from("hello");

    // If inmutable references exist, we cannot create a mutable reference 
    //although the mutable reference is created after the inmutable references are no longer used.
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) 
    //If we define a string in the fuction and then return it, the ownership
    //of the string will be moved to the calling function, so it will b<e available.
}

fn calculate_lenght_by_reference(s: &String) -> usize { //with reference the function borrows the value 
                                                        //DOES NOT TRANSFER OWNERSHIP
    s.len()

}