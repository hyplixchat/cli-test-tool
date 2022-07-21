use std::fs;
#[path = "./utils.rs"] mod utils;
// use std::path::Path;

fn save_token(token: &str){
    fs::write("./.token", token).expect("Unable to write file");
}

fn command_handler(_input: String){

}


pub fn continue_login(token: &str){
    println!("{}", token);
    // if save_token_bool {
    //     save_token(token);
    // }
    let mut user = "";
    let mut channel = "";
    let mut server = "";

    loop {
        let input = utils::read_line(format!("{}:{}@{}>", user, channel, server).as_str());
        if input == "" {
            continue;
        }
        if input.starts_with("/"){
            println!("Running command {}", input);
            command_handler(input);
        } else {

        }
    }
    // println!("token: {}", token);

}