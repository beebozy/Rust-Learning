mod serialize;
mod deserialize;

use serialize::{serialize_user, User as SerializeUser};
use deserialize::{deserialize_user, User as DeserializeUser};

#[warn(unused_imports)]
fn main() -> std::io::Result<()> {
    let user = SerializeUser {
        age: 15,
        name: "alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Serialize the user to a file
    serialize_user(&user, "user.json")?;

    // Deserialize the user from the file
    let deserialized_user = deserialize_user("user.json")?;
    println!(
        "Deserialized Struct:\nName: {}\nAge: {}\nEmail: {}",
        deserialized_user.name, deserialized_user.age, deserialized_user.email
    );

    Ok(())
}
