use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Deserialize)]
pub struct User {
    pub age: i32,
    pub name: String,
    pub email: String,
}

pub fn deserialize_user(file_name: &str) -> std::io::Result<User> {
    let mut file = OpenOptions::new().read(true).open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let user: User = serde_json::from_str(&contents).expect("Deserialization failed");
    Ok(user)
}
