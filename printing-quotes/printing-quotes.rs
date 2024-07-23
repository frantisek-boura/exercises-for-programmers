use std::io;
use std::io::Write;

fn main() {
    
    print!("What is the quote? ");
    io::stdout().flush().unwrap();
    
    let mut quote: String = String::new();
    io::stdin().read_line(&mut quote).unwrap();
    quote.pop();
    
    print!("Who said it? ");
    io::stdout().flush().unwrap();

    let mut author: String = String::new();
    io::stdin().read_line(&mut author).unwrap();
    author.pop();

    let mut result: String = String::new();
    result.push_str(&author);
    result.push_str(" says, \"");
    result.push_str(&quote);
    result.push_str("\"");

    println!("{}", result);
    



}
