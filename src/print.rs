pub fn print_it(){
    // Create modifiable string
    let mut s = String::from("Welcome");
    s.push_str(" to calculater");

    {
        // Pass a reference to a function
        let split = split_by_spaces(&s);
        for i in &split {
            println!("{}", i);
        }
    }
    
    // Continue to modify string, this needs to be after split is out of scope because
    // modifying a string while references to its contents exist is not allowed
    s.push_str("!");
    println!("{}", s);
}

fn split_by_spaces(s: &String) -> Vec<&str>{
    let mut split_words: Vec<&str> = Vec::new();
    let mut start = 0;

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || i == s.len()-1 {
            split_words.push( &s[start..i+1]);
            start = i+1;
        }
    }
    split_words
}