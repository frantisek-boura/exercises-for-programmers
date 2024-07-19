use std::io;
use std::io::Write;

fn main() {
    print!("What is your name? ");

    // Calling the flush() method forces the stdout buffer to flush - to send the buffer's contents
    // to stdout immediately
    // Unwrap is called afterwards because flush() returns a Result object, the value of which
    // can be ignored in this instance
    io::stdout().flush().unwrap();

    let mut name: String = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_n) => {
            name.pop();
            println!("Hello, {name}, nice to meet you!");
        }
        Err(_e) => {
            println!("Can't get your name");
        }
    }
}
