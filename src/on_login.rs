use std::fs;
use std::path::Path;

fn save_token(token: &str){

    if Path::new("./.token").exists() {

    }

    fs::write("./.token", token).expect("Unable to write file");
}


pub fn continue_login(token: &str, save_token_bool: bool){
    if save_token_bool {
        save_token(token);
    }

    println!("token: {}", token);

}