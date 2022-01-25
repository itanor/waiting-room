use log::{info};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    name: String,
    hash: String,
    served: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct WaitingUser {
    id: String,
    name: String,
}

#[post("/generate-hash", format = "json", data = "<user>")]
fn generate_hash(user: Json<WaitingUser>) {
    println!("{:?}", user); 

    let sha = digest(&user.id);
    info!("sha = {}", sha);

    // send to redis...
    // send to rabiitmq...
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            generate_hash
        ])
}
