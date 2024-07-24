use std::io;
use std::io::Write;
use std::time::SystemTime;

fn read_line_from_stdin() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn main() {

    print!("What is your current age? ");
    io::stdout().flush().unwrap();
    let current: i32;
    match read_line_from_stdin() {
        Ok(v) => {
            match v.trim().parse::<i32>() {
                Ok(n) => {
                    if n <= 0 {
                        println!("Impossible");
                        return;
                    }

                    current = n
                }
                Err(_e) => {
                    println!("This is not a valid number");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Cannot get your input");
            return;
        }
    }

    print!("At what age would you like to retire? ");
    io::stdout().flush().unwrap();
    let retirement: i32;
    match read_line_from_stdin() {
        Ok(v) => {
            match v.trim().parse::<i32>() {
                Ok(n) => {
                    if n <= 0 {
                        println!("Impossible");
                        return;
                    }

                    if n <= current {
                        println!("You can already retire");
                        return;
                    }

                    retirement = n
                }
                Err(_e) => {
                    println!("This is not a valid number");
                    return;
                }
            }
        }
        Err(_e) => {
            println!("Cannot get your input");
            return;
        }
    }

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            let current_year: i32 = 1970 + ((n.as_secs() / (31536000))) as i32;
            let diff: i32 = retirement - current;
            let retirement_year: i32 = current_year + diff;
            println!("You have {diff} years left until you can retire.");
            println!("It's {current_year}, so you can retire in {retirement_year}.");
        }
        Err(_) => {
            println!("SystemTime before UNIX EPOCH!");
        }
    }
}
