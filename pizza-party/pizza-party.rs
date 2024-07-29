use std::io;
use std::io::Write;


fn get_input() -> Result<String, io::Error> {    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn convert_to_int(input: String) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = input.trim().parse::<i32>()?;

    Ok(num)
}



fn main() {

    print!("How many people? ");
    io::stdout().flush().unwrap();

    let people_count: i32;
    match get_input() {
        Ok(v) => {
            match convert_to_int(v) {
                Ok(n) => {
                    people_count = n;
                }
                Err(_e) => {
                    println!("Entered value is not a valid number.");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Couldn't get your input.");
            return;
        }
    }

    print!("How many pizzas do you have? ");
    io::stdout().flush().unwrap();

    let pizza_count: i32;
    match get_input() {
        Ok(v) => {
            match convert_to_int(v) {
                Ok(n) => {
                    pizza_count = n;
                }
                Err(_e) => {
                    println!("Entered value is not a valid number.");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Couldn't get your input.");
            return;
        }
    }

    const SLICES: i32 = 8;
    let slices_per_person: i32 = pizza_count * SLICES / people_count;
    let leftover_slices: i32 = pizza_count * SLICES % people_count;

    println!("{people_count} people with {pizza_count} pizzas.");
    println!("Each person gets {slices_per_person} slices of pizza");
    println!("There are {leftover_slices} leftover slices");
    

     
}
