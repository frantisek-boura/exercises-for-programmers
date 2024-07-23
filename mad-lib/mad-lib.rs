use std::io::Write;

fn main() {

    print!("Enter a noun: ");
    std::io::stdout().flush().unwrap();
    let mut noun: String = String::new();
    std::io::stdin().read_line(&mut noun).unwrap();
    noun.pop();
    
    print!("Enter a verb: ");
    std::io::stdout().flush().unwrap();
    let mut verb: String = String::new();
    std::io::stdin().read_line(&mut verb).unwrap();
    verb.pop();

    print!("Enter an adjective: ");
    std::io::stdout().flush().unwrap();
    let mut adjective: String = String::new();
    std::io::stdin().read_line(&mut adjective).unwrap();
    adjective.pop();
    
    print!("Enter an adverb: ");
    std::io::stdout().flush().unwrap();
    let mut adverb: String = String::new();
    std::io::stdin().read_line(&mut adverb).unwrap();
    adverb.pop();

    println!("Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!");

}
