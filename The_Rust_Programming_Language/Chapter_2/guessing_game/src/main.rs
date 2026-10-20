use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("\nGuess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    // println!("The Secret number is: {secret_number}");

    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failedd to read line");

        // let guess: u32 = guess.trim().parse() //Convert a string to an int
        //     .expect("Please type a number!"); 

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            _ => continue,
            //Also Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return ();
            },     
        }

    } // loop
}