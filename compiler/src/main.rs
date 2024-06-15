mod parser;
mod pp;
mod renamer;
mod simplifier;

fn main() {
    const FILENAME: &str = "example/fibonacci.oath";
    let contents = std::fs::read_to_string(FILENAME).expect("Could not read file");

    let syntax = crate::parser::parse(&contents).expect("Could not parse file");
    let renamed = crate::renamer::rename(&syntax);

    println!("{:#?}", renamed)
}
