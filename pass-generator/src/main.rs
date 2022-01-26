use log::{info};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha256::digest;
use redis::Commands;
use amiquip::{Connection, Exchange, Publish};

#[macro_use] extern crate rocket;
extern crate redis;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    name: String,
    hash: String,
    served: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    WAITING,
    SERVED,
}

impl Default for Status {
    fn default() -> Self {
        Status::WAITING
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
    let _ = send_to_redis(&user, &sha);
    // send to rabiitmq...
    let _ = send_to_rabbit(&user, &sha);
}

fn send_to_redis(user: &Json<WaitingUser>, hash: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let user_as_json = json!({
        "id": &user.id,
        "sha": &hash,
        "status": "WAITING",
    });
    let _ : () = con.set(&user.id, user_as_json.to_string())?;
    let received: String = con.get(&user.id)?;
    println!("received: {}", received);

    Ok(())
}

fn send_to_rabbit(user: &Json<WaitingUser>, hash: &str) -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let user_as_json = json!({
        "id": &user.id,
        "sha": &hash,
        "status": "WAITING",
    });
    exchange.publish(Publish::new(
        user_as_json.to_string().as_bytes(), 
        "generated-pass"))?;

    connection.close()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            generate_hash
        ])
}
