#![feature(proc_macro_hygiene, decl_macro)]

extern crate log;
extern crate simple_logger;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod router;

use log::{trace,info};

use std::path::{Path,PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;

use ra_common::models::{Packet, PacketType, NetworkId};
use onemfive_common::ManCon;
use router::NetworkRouter;

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
    let exit_code = 0;
    simple_logger::init().unwrap();
    trace!("Starting 1M5...");
    // Start the Service/SEDA Bus

    // Register Network Services (e.g. I2P Client)

    // Register Network Router Service
    let mut n_router = NetworkRouter::new();
    n_router.init();

    let from = String::from("1234");
    let to = String::from("5678");
    let sig = String::from("sig");
    let mut packet = Packet::new(PacketType::Data, NetworkId::IMS, from, to, sig);
    // packet.headers.insert(String::from("mancon"), ManCon::VeryHigh.try_into());
    n_router.route(&mut packet);
    // Register App Services (e.g. InfoVault)

    // Start API service
    let err = rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/api", routes![index,peer])
        .launch();

    std::process::exit(exit_code);
}
