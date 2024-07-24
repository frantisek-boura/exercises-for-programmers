use std::io;
use std::io::Write;

const M_CONVERSION_FACTOR: f64 = 0.09290304;

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn main() {

    print!("What is the length of the room in feet? ");
    io::stdout().flush().unwrap();
    let length: f64;
    match get_input() {
        Ok(v) => {
            match v.trim().parse::<f64>() {
                Ok(n) => {
                    length = n;
                }
                Err(_e) => {
                    println!("Enter a valid number");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Couldn't get the length of the room");
            return;
        }
    }

    print!("What is the length of the room in feet? ");
    io::stdout().flush().unwrap();
    let width: f64;
    match get_input() {
        Ok(v) => {
            match v.trim().parse::<f64>() {
                Ok(n) => {
                    width = n;
                }
                Err(_e) => {
                    println!("Enter a valid number");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Couldn't get the width of the room");
            return;
        }
    }

    println!("You entered dimensions of {length} feet by {width} feet.");
    let area: f64 = width * length;
    let area_m: f64 = area * M_CONVERSION_FACTOR;
    println!("The area is\n{area} square feet\n{area_m} square meters");

}
