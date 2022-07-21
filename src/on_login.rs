use rust_socketio::{ClientBuilder, Payload, Client};
use std::fs;
use tinyjson::JsonValue;
// use substring::Substring;

#[path = "./commands.rs"] mod commands;
#[path = "./utils.rs"] mod utils;
#[path = "./get.rs"] mod get;
use std::path::Path;
pub struct Data{
    server: String,
    channel: String,
    channel_old: String,
    server_old: String
}



fn save_token(token: &str){

    if !Path::new("./.token").exists() {
        if fs::read_to_string("./.token").expect("Unable to read file") != token {
            if utils::read_line("Save token for other sessions? (y/n): ").to_lowercase() == "y".to_string(){
                fs::write("./.token", token).expect("Unable to write file");
                println!("Saved!")
            } 
        }
    }
    
}


pub fn continue_login(host: &str, token: &str){
    let mut data = Data{server: String::new(), channel: String::new(), server_old: String::new(), channel_old: String::new()};
    // println!("{}", token);
    // let mut input = String::new();//Vec::new();
    let token_str = token.to_string();
    
    let on_hello = move |_payload: Payload, socket: Client| {
        let token_payload: String = "{\"token\":\"".to_string() + &token_str + "\"}";
        socket.emit("verify", token_payload).expect("Server unreachable");
    };

    let socket = ClientBuilder::new("http://ssh.mc.mcorangehq.xyz:3000/ws")
     .namespace("/")
     .on("hello", on_hello)
     .on("error", |err, _| eprintln!("Error: {:#?}", err))
     .connect()
     .expect("Connection failed");

    save_token(token);
    

    let userdata_tmp = match get::get_user_info(host, token) {
        Ok(data) => data,
        Err(_e) => "".to_string()
    };
    let user_data: JsonValue = userdata_tmp.parse().unwrap();

    // println!("{:#?}", user_data);
    let user = match &user_data["username"] {
        JsonValue::String(s) => s,
        _ => panic!("Unexpected!"),
    };

    loop {

        // if data.channel != data.channel_old {

        // }

        // if data.server != data.server_old {

        // }
        let input = utils::read_line(format!("{}:{}@{}>", user, data.channel, data.server).as_str());

        // let input_arr: Vec<&str> = input.split(" ").collect();
        if input == "" {
            continue;
        }
        if input.starts_with("/"){
            println!("Running command {}", input);
            
            
        } else {
            socket.emit("global-message-create", input.to_string());
        }
    }
    // println!("token: {}", token);

}