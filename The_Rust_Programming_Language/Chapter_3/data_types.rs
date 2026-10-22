fn main() {


    let _this_is_a_bool : bool = true;
    let _heart_eyed_cat = '😻'; //Chars supports UTF-8 (may occup 4 bytes)
    let _number : u32 = "42".parse().expect("Not a number!");
    let _x = 2.0; // f64
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let r = tup.0;
    let a [i32; 5] = [1, 2, 3, 4, 5]; //fix lenght
    //[3; 5]; 

    println!("The value of y-r is: {y}-{r}");
    println!("a.1 = {}", a[1])
   

   
}