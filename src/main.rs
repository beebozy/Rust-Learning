
use serde::{Serialize, Deserialize};
use std::fs::{File,OpenOptions};
use std::io::{Write,Read};

#[derive(Serialize,Deserialize)]
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


let mut file= OpenOptions::new().read(true).open("user.json")?;

let mut contents= String::from("");

file.read_to_string(&mut contents)?;
let deserialized_user : User= serde_json::from_str(&contents).expect("Deserialization failed");
println!("Deserialized Struct:\nName: {}\nAge: {}\nEmail: {}", deserialized_user.name, deserialized_user.age, deserialized_user.email);

    Ok(())
}

// fn deserialize(){

//     let json_data = r#"
//     {
    
//     age :15,
//     name: "alice",

//     email: "alice@xample.co"


//     }#";


//     let user : User = serde_json::jsond_data.to_string().expect("Deserializaiton failed");

//     println!("")
// }