fn first_word(s: &str) -> &str { // also first_word(s: &String)

    let in_bytes = s.as_bytes();

    for (i, &item) in in_bytes.iter().enumerate() {
        if item == b' ' {return &s[0..i];} //equivalent to &s[..i]
        }

    &s[..]
}


fn main() {

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("First word is: {word}");

    s.clear(); // this empties the String, making it equal to ""

}