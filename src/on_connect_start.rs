
use reqwest;
// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use std::collections::HashMap;
use tinyjson::JsonValue;
use std::fs;
use std::path::Path;
// use std::process::exit;

#[path = "./options.rs"] mod options;
#[path = "./on_login.rs"] mod on_login;
#[path = "./utils.rs"] mod utils;


pub fn try_saved_token_login(host: &str) -> [String; 2] {
    let mut user_data = String::new();
    println!("Trying to use the saved token.");
    if !Path::new("./.token").exists() {
        println!("Token file not found, redirecting to login.");
        return [String::new(), String::new()];
    }

    let client = reqwest::blocking::Client::new();
    let token = fs::read_to_string("./.token").expect("Unable to read file");
    // println!("token: {}", token);
    let res = client.get(format!("{}/user/me", host))
        .bearer_auth(&token)
        .send();

    println!("{:#?}", res);
    match res {
        Ok(r) => {
            if r.status() != 200 {
                println!("Token file found, but authentication failed, redirecting to login.");
                return [String::new(), String::new()];
            }

            match r.text() {
                Ok(txt) => {
                    user_data = txt;
                },
                Err(e) => {eprintln!("ERR: {:#?}", e)}
            }
        },
        Err(e) => {eprintln!("ERR: {:#?}", e)}
    }
    // println!("{:#?}", res.text());
    return [token, user_data];// not succesfull
}



#[allow(unused_must_use)]
pub fn connect(host: &str, _username: &str, _password: &str, _email: &str, login_tgl: bool){
    

    let mut save_token = false;


    #[allow(unused_assignments)]
    let mut status = String::new();
    if !login_tgl{
        status = utils::read_line("Login or register? "); 
    } else {
        status = "login".to_owned();
    }
    

    if status == "login".to_owned()  {

        let client = reqwest::blocking::Client::new();

        println!("Connecting to: '{}'", host);
        #[allow(unused_assignments)]
        let mut username = String::new();
        #[allow(unused_assignments)]
        let mut password: String = String::new();
        if login_tgl && (_username != ""){
            username = _username.to_string();
        } else {
            username = utils::read_line("Username: ");
        }

        if login_tgl && (_password != ""){
            password = _password.to_string();
        } else {
            password = utils::read_line("Password: ");
        }
        
        
        // println!("{{\"password\":\"{}\",\"username\":\"{}\"}}", password, username);
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        let res = client.post(format!("{}/auth/login", host))
        .json(&map)
        .send();
        match res{
            Ok(r) => {
                // println!("OK: {:#?}", r);
                match r.text(){
                    Ok(txt) => {
                        let parsed: JsonValue = txt.parse().unwrap();
                        let token = match &parsed["token"] {
                            JsonValue::String(tkn) => tkn,
                            _ => panic!("Token type is invalid!"),
                        };
                        // println!("{}", token);
                        println!("Succesfully logged in as MCorange!");
                        let save_tokenq: String = utils::read_line("Save token for other sessions? (y/n): "); 
                        if save_tokenq.to_lowercase().as_str() == "y" {
                            save_token = true;
                        }
                        on_login::continue_login(token, save_token);
                    },
                    Err(e) => {eprintln!("ERR: {:#?}", e)}
                }      
            },
            Err(e) => {eprintln!("ERR: {:#?}", e)}
        }
    
    } else if status == "register".to_owned() {
        let client = reqwest::blocking::Client::new();

        println!("Connecting to: '{}'", host);
        let username: String = utils::read_line("Username: ");
        let password: String = utils::read_line("Password: ");
        let password2: String = utils::read_line("Password confirmation: ");
        let email: String = utils::read_line("Email: ");


        if password != password2 {
            println!("Passwords do not match!");
            return;
        }

        // println!("{{\"password\":\"{}\",\"username\":\"{}\"}}", password, username);

        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("email", email);
        map.insert("password", password);

        let res = client.post(format!("{}/auth/register", host))
        .json(&map)
        .send();
        match res{
            Ok(r) => {
                // println!("OK: {:#?}", r);
                match r.text(){
                    Ok(txt) => {
                        let parsed: JsonValue = txt.parse().unwrap();
                        let token = match &parsed["token"] {
                            JsonValue::String(tkn) => tkn,
                            _ => panic!("Token type is invalid!"),
                        };
                        // println!("{}", token);
                        println!("Succesfully logged in as MCorange!");
                        let save_tokenq: String = utils::read_line("Save token for other sessions? (y/n): "); 
                        if save_tokenq.to_lowercase().as_str() == "y" {
                            save_token = true;
                        }
                        on_login::continue_login(token, save_token);
                    },
                    Err(e) => {eprintln!("ERR: {:#?}", e)}
                }      
            },
            Err(e) => {eprintln!("ERR: {:#?}", e)}
        }
    } else {
        println!("Unknow option {:?}", status);
    }

}