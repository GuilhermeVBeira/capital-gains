use capital_gains::serializer::converter_raw_json;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result = converter_raw_json(&input);
    if result.is_err() {
        println!("There was an error in the input");
    } else {
        println!("{}", result.unwrap());
    }
}
