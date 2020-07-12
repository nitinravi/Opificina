#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use postgres::{Client, NoTls};
use rocket::{
    http::{Cookie, Cookies},
    request::{self, Form, FromRequest, Request},
    response::{self, NamedFile, Redirect, Responder},
    Config, Outcome, Response, State,
};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{env, sync::Mutex};

mod db_operations;

use db_operations::*;

#[derive(Serialize)]
struct Context {
}

#[get("/")]
fn get_index() -> Template {
    Template::render("index", Context {})
}

#[get("/registration")]
fn get_registration() -> Template {
    Template::render("registration", Context {})
}

#[get("/browse")]
fn get_browse() -> Template {
    Template::render("browse", Context {})
}

fn configure() -> Config {
    // Configure Rocket to serve on the port requested by Heroku.
    let mut config = Config::active().expect("could not load configuration");
    config
        .set_secret_key(env::var("SECRET_KEY").unwrap())
        .unwrap();
    let port = if let Ok(port_str) = env::var("PORT") {
        port_str.parse().expect("could not parse PORT")
    } else {
        6633
    };
    config.set_port(port);
    config
}

fn rocket() -> rocket::Rocket {
    rocket::custom(configure())
        .mount(
            "/",
            routes![
                get_index,
                get_registration,
                get_browse,
            ],
        )
        .mount("/styles", StaticFiles::from("static/styles"))
        .mount("/scripts", StaticFiles::from("static/scripts"))
        .mount("/fonts", StaticFiles::from("static/fonts"))
        .mount("/images", StaticFiles::from("static/images"))
        .mount("/videos", StaticFiles::from("static/videos"))
        .mount("/", StaticFiles::from("static/icons").rank(20))
        .attach(Template::fairing())
}

fn main() {
    let client = Client::connect(&env::var("DATABASE_URL").unwrap(), NoTls).unwrap();
    rocket().manage(Mutex::new(client)).launch();
}
