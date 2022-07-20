use std::env;
use std::process::exit;

mod on_connect_start;
mod options;

/**
 * Default settings
 */
static HOST: &str = "http://ssh.mc.mcorangehq.xyz:3000";

fn handle_unknown_env_arg(env_arg: &str){
    println!("Unknown Env Argument => {}", env_arg);
    exit(1);
}

fn handle_help(exec: &str){
    println!("Usage: \n    {} [FLAGS]",exec);
    println!("FLAGS: \n{}\n{}",
                "          -h, --hello    Shows this help page,",
                "--host [ip or domain]    Changes the default server for the chat");
    exit(0);
}


fn main() {
    
    // let mut arg_itr = 0;
    let args = Vec::from_iter(env::args());
    let mut args: Vec<&str> = args.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let exec = args.remove(0);
    let mut host_tgl = false;
    let mut username_tgl = false;
    let mut password_tgl = false;
    let mut email_tgl = false;
    let mut options = options::Options{
        host: HOST,
        username: "",
        password: "",
        email: "",
        login: false
    };
    for argument in args {
        if host_tgl {
            host_tgl = false;
            options.host = argument;
            continue;
        }

        if username_tgl {
            username_tgl = false;
            options.username = argument;
            continue;
        }

        if password_tgl {
            password_tgl = false;
            options.password = argument;
            continue;
        }

        if email_tgl {
            email_tgl = false;
            options.email = argument;
            continue;
        }

        match argument {
            "-h" => {handle_help(exec)},
            "--help" => {handle_help(exec)},
            "--host" => {host_tgl = !host_tgl},
            "--login" => {options.login = true},
            "--username" => {username_tgl = true},
            "--password" => {password_tgl = true},
            "--email" => {email_tgl = true},
            _ => {handle_unknown_env_arg(argument);}
        }
        // arg_itr += 1;
    }
    // println!("{:#?}", options);
    on_connect_start::connect(options.host, options.username, options.password, options.email, options.login);
    // println!("Host: {}", options.host);
    // println!("Login:");
}
