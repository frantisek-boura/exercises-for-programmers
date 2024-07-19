use std::io;
use std::io::Write;

fn main() {
    let mut input: String = String::new();

    loop {
        print!("What is the input string? ");
        io::stdout().flush().unwrap();


        match io::stdin().read_line(&mut input) {
            Ok(_v) => {
                input = input.trim().to_string();
                let count: usize = input.chars().count();

                if count == 0 {
                    continue;
                } else {
                    println!("'{input}' has {count} characters.");
                    break;
                }
            }
            Err(_e) => {
                println!("Can't get your input");
                break;
            }
        }
    }
}
