mod grammer;

use grammer::Grammer;

fn main() {
    let grammer = Grammer::from_file(String::from("input"));
    if grammer.try_word() {
        println!("The word was accepted!");
    } else {
        println!("The word was rejected!");
    }
}
