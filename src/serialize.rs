use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct User {
    pub age: i32,
    pub name: String,
    pub email: String,
}

pub fn serialize_user(user: &User, file_name: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(user).expect("Failed to serialize data");

    let mut file = File::create(file_name)?;
    file.write_all(json_data.as_bytes())?;

    println!("JSON data written to '{}' successfully!", file_name);
    Ok(())
}
