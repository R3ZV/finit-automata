mod regex;
use crate::regex::processor::RegexProcessor;

fn info() {
    let art = r" ____            _____      
|  _ \ ___  __ _| ____|_  __
| |_) / _ \/ _` |  _| \ \/ /
|  _ <  __/ (_| | |___ >  < 
|_| \_\___|\__, |_____/_/\_\
           |___/             ";

    println!("{}", art);
    println!();
    println!("Parenthesis: ( )");
    println!("Closure: *");
    println!("Concatenation .");
    println!("Union: |");
    println!("Symbols: lowercase letters [a-z]");
    println!("Lambda: L");
    println!();
    println!("IMPORTANT!");
    println!("You are expected to put the proper parenthesis!");
    println!();
}
fn main() {
    info();
    // Reference: http://cgosorio.es/Seshat/thompson?expr=%28%28a+%7C+b%29+%7C+c%29

    println!("Your regex: ");
    let mut regex = String::new();
    std::io::stdin()
        .read_line(&mut regex)
        .expect("Couldn't read from stdin");
    regex = regex.trim().to_string();

    let processor = RegexProcessor::new(regex);
    let lambda_nfa = processor.generate();

    dbg!(lambda_nfa);
}
