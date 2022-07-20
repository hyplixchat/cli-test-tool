use std::io; // package
use text_io; // package
use std::io::Write; // package

pub fn read_line(question: &str) -> String{
    print!("{}", question); // print a line without a /n
    io::stdout().flush().unwrap(); // forcefully push the print!();
    let line: String = text_io::read!("{}\n"); // read input
    return line; // return input
}