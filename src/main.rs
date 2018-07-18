use std::io;
mod print;
fn main() {
    print::print_it();

    let mut current_val = 0.0;
    let mut operator = "+";

    loop {
        // Creates a new string reference
        let mut input = String::new();
        // Allocates memory and populates input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Sets input to the substring of the read in input without newlines/spaces
        let input = input.trim();

        // Check if input was a new operator
        if input == "+" {
            operator = "+";
            continue;
        } else if input == "-" {
            operator = "-";
            continue;
        }
        
        // Check if value is number
        let number = input.parse::<f32>();
        if number.is_err() {
            println!("Invalid input");
            continue;
        }

        // depending on the last operator modify the anser
        let number = number.unwrap();
        if operator == "+" {
            current_val += number;
        }else if operator == "-" {
            current_val -= number;
        }else if operator == " " {
            current_val = number;
        }
        operator = " ";
        
        // Output result
        println!("Ans: {}", current_val);
    }
}