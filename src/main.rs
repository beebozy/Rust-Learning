
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct User{
    age:i32,
    name:String,
    email:String,
}
fn main()->std::io::Result<()> {

let user = User {
    age:  15,
    name: "alice".to_string(),
    email: "alice@example.com".to_string(),
};

let json_data= serde_json::to_string_pretty(&user).expect("Failed to serialize data");

let mut file= File::create("user.json")?;
file.write_all(json_data.as_bytes())?;

println!("JSON data written to 'user.json' successfully!");
    Ok(())

}
