
use std::io;
use std::io::Write;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {

    print!("What is the first number? ");
    io::stdout().flush().unwrap();
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();

    let num_a: i32;
    match input_a.trim().parse::<i32>() {
        Ok(v) => {
            if v < 0 {
                println!("No negative numbers");
                return;
            }

            num_a = v;
        }
        Err(_e) => {
            println!("First number is not a valid number");
            return;
        }
    }

    print!("What is the second number? ");
    io::stdout().flush().unwrap();
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();

    let num_b: i32;
    match input_b.trim().parse::<i32>() {
        Ok(v) => {
            if v < 0 {
                println!("No negative numbers");
                return;
            }

            if v == 0 {
                println!("Second number cannot be 0");
                return;
            }

            num_b = v;
        }
        Err(_e) => {
            println!("Second number is not a valid number");
            return;
        }
    }

    let addition = add(num_a, num_b);
    let subtraction = subtract(num_a, num_b);
    let multiplication = multiply(num_a, num_b);
    let division = divide(num_a, num_b);

    println!("{num_a} + {num_b} = {addition}\n{num_a} - {num_b} = {subtraction}\n{num_a} * {num_b} = {multiplication}\n{num_a} / {num_b} = {division}"); 
}
