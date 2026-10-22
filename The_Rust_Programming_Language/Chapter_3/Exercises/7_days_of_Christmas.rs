
fn main(){
    println!("\n\n\n");
    let numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh"];
    let lines = [
        "a partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens ",
        "Four calling birds",
        "Seven swans a-swimming",
        "Five golden rings",
        "Six geese a-laying"
    ];
    let mut verse = 0;
    
    for n in numbers {
        verse += 1;
        println!("On the {n} day of Christmas, my true love sent to me");

        for line_index in (0..verse).rev(){

            println!("\t{}", lines[line_index]);
        }
        println!("")

    }


}