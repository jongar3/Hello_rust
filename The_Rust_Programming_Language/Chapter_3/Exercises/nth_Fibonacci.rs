use std::io;

fn main(){    
    
    loop{
        let mut input = String::new();
        println!("Introduce the n-th Fibonacci number: ");
        io::stdin().read_line(&mut input).expect("Failed to read");
        let n : u32= input.trim().parse().expect("The input must be a positive integer...");
        println!("the {n}th Fibonacci number is {}", fibonnacci(n));
    
    }


}

fn fibonnacci(n : u32) -> u64 {
    if n <= 1{
        return 0;
    }else if n <= 3{
        return 1;
    }else{
        return fibonnacci(n-1) + fibonnacci(n-2);
    }

}