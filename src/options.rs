#[allow(dead_code)]
#[derive(Debug)]
pub struct Options<'a>{
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub login: bool
}