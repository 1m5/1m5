#![feature(proc_macro_hygiene, decl_macro)]

extern crate log;
extern crate simple_logger;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod router;

use log::{trace,info};

use std::path::{Path,PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;
use crate::router::NetworkRouter;

/// **** Admin UI **** ///


/// **** API **** ///
#[get("/networks")]
fn networks() -> JsonValue {
    unimplemented!()
}

#[get("/peer/<id>")]              // <- route attribute
fn peer(id: String) -> JsonValue {  // <- request handler
    let id = format!("{}", id.as_str());
    json!({
        "id": id,
        "name": "Matt"
    })
}

#[get("/")]
fn index() -> &'static str {
    "API is up!"
}

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting 1M5 Daemon...");

    // Start the Service/SEDA Bus

    // Register Network Services (e.g. I2P Client)

    // Register Network Router Service
    let mut n_router = NetworkRouter::new();
    n_router.init();


    // Register App Services (e.g. InfoVault)

    // Start API service
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/api", routes![index,peer])
        .launch();
    trace!("1M5 Daemon Stopped.");
}
