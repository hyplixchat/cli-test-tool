use std::io; // package
use text_io; // package
use std::io::Write; // package
use reqwest;
use std::collections::HashMap;
use tinyjson::JsonValue;
// use std::process::exit;



#[path = "./options.rs"] mod options;
#[path = "./on_login.rs"] mod on_login;


fn read_line(question: &str) -> String{
    print!("{}", question); // print a line without a /n
    io::stdout().flush().unwrap(); // forcefully push the print!();
    let line: String = text_io::read!("{}\n"); // read input
    return line; // return input
}

#[allow(unused_must_use)]
pub fn connect(host: &str, _username: &str, _password: &str, _email: &str, login_tgl: bool){
    let mut save_token = false;


    #[allow(unused_assignments)]
    let mut status = String::new();
    if !login_tgl{
        status = read_line("Login or register? "); 
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
            username = read_line("Username: ");
        }

        if login_tgl && (_password != ""){
            password = _password.to_string();
        } else {
            password = read_line("Password: ");
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
                        let save_tokenq: String = read_line("Save token for other sessions? (y/n): "); 
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
        let username: String = read_line("Username: ");
        let password: String = read_line("Password: ");
        let password2: String = read_line("Password confirmation: ");
        let email: String = read_line("Email: ");


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
                        let save_tokenq: String = read_line("Save token for other sessions? (y/n): "); 
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