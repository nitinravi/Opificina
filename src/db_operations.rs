use super::RegistrationInfo;
use argon2::{self, Config};
use postgres::Client;
use chrono::NaiveDateTime;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
};

/*
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR (100) UNIQUE NOT NULL,
    password_hash VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
)
*/

pub fn create_table(client: &mut Client) {
    client.execute("CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR (100) UNIQUE NOT NULL,
        password_hash VARCHAR NOT NULL,
        category VARCHAR NOT NULL,
    )", &[]).unwrap();
}

// Function to create a user with the given details if they're valid
pub fn register(
    client: &mut Client,
    registration_info: Form<RegistrationInfo>,
    mut cookies: Cookies,
) -> String {
    let valid_categories = ["Individual Business", "Private Organisation", "Government Organisation", "Other"];
    if registration_info.email.len() == 0 {
        return String::from("Error: email not provided");
    } else if registration_info.password.len() < 8 {
        return String::from("Error: password has to be at least 8 characters");
    } else if !valid_categories.contains(&registration_info.category.as_str()) {
        return String::from("Invalid category");
    }

    // Generate salt using a CSPRNG
    let rng = thread_rng();
    let salt = Hc128Rng::from_rng(rng).unwrap().next_u64();
    let config = Config::default();
    let password_hash = argon2::hash_encoded(
        &registration_info.password.as_bytes(),
        &salt.to_ne_bytes(),
        &config,
    )
    .unwrap();
    if let Err(e) = client.query(
        "INSERT INTO users VALUES (
        DEFAULT, $1, $2
    )",
        &[&registration_info.email, &password_hash],
    ) {
        println!("Error: {}", e.to_string());
        return e.to_string();
    }
    cookies.add_private(Cookie::new("email", registration_info.email.clone()));
    cookies.add_private(Cookie::new("hash", password_hash));
    return String::from("Success");
}
