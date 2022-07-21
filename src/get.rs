

pub fn get_user_info(host: &str, token: &str) -> Result<String, ()>{
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("{}/user/me", host))
        .bearer_auth(&token)
        .send();

    match res {
        Ok(r) => {
            if r.status() != 200 {
                println!("Token file found, but authentication failed, redirecting to login.");
                return Err(())
            }

            match r.text() {
                Ok(txt) => {
                    Ok(txt)
                },
                Err(_e) => {
                    Err(())
                }
            }
        },
        Err(_e) => {
            Err(())
        }
    }
}