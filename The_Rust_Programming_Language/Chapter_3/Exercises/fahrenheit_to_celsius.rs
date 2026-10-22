use std::io;


fn main(){
    println!("Fahrenheit to celsius calculator:");

    loop{

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");
        println!("Introduce a value in ºF: ");

        let num : f64= match input.trim().parse(){
            Ok(n) => n,
            _ => continue,};

        println!("\t{num} ºF = {}", fahr_to_celsius(num))

    }

}


fn fahr_to_celsius(fahr: f64) -> f64{
   (fahr - 32.0) * (5.0/9.0)

}