use std::fs;
mod utils;
// use std::path::Path;

fn save_token(token: &str){
    fs::write("./.token", token).expect("Unable to write file");
}

fn command_handler(){

}


pub fn continue_login(token: &str, save_token_bool: bool){
    if save_token_bool {
        save_token(token);
    }
    let mut user = "";
    let mut channel = "";
    let mut server = "";

    loop {
        let input = utils::read_line(format!("{}:{}@{}>", user, channel, server).as_str());

        if input.starts_with("/"){
            println!("Running command {}", input);
        }
    }
    // println!("token: {}", token);

}