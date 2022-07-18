

pub fn parse() -> JsonValue{

    if !Path::new("./config.json").exists() {
        panic!("{}","Could not read config file './config.json'!".red());
    }

    let file = fs::read_to_string("./config.json").expect("Unable to read config file");
    let host: String = String::new();
    let port: String = String::new();
    let route_auth_login: String = String::new();
    let route_auth_register: String = String::new();
    let debug: bool = false;

    for i in file.lines() {
        let y = i.split("=");
        match y[0] {
            "host" => host = y[1],
            "port" => port = y[1],
        }
    }
    //println!("{:?}", file);
    let config = {   
        host: "127.0.0.1",
        port: "3000",
        routes: {
            auth_login: "/login",
            auth_register: "/register"
        },
        debug: false
    }
    return config
}