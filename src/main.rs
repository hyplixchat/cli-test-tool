
mod config_parser; // local file
mod login; // local file
mod register; // local file
mod send_message; // local file

use reqwest; // package
use std::io; // package
use text_io; // package
use std::io::Write; // package
use tinyjson::JsonValue;

//custom function for cleaner code
fn read_line(question: &str) -> String{
    print!("{}", question); // print a line without a /n
    io::stdout().flush().unwrap(); // forcefully push the print!();
    let line: String = text_io::read!("{}\n"); // read input
    return line; // return input
}

fn credits(exec_name: &str){
    println!("Test cli tool for {}.\nAll rights reserved", exec_name);
}

fn main() {
    credits("still_no_name");
    let client = reqwest::Client::new(); // client for api
    let config = config_parser::parse();
    
    let mut line: String = String::new();
    let host: String = config["host"].try_into().unwrap();//.stringify().unwrap();
    println!("host: {:?}", host);
    loop { // loop forever
        line = read_line(">"); // use the custom function
        if line != "".to_string() {
            //send_message::send(client, config, line);
        }
        
    }
    
}
