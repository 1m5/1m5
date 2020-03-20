extern crate log;
extern crate simple_logger;

use log::{trace,info};
use ra_common::Envelope;
use ra_common::LifeCycle;
use ra_common::Route;
use ra_common::Producer;
use seda_bus::bus::MessageBus;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting 1M5 Daemon...");

    trace!("Setting up Message Bus...");
    let mut bus = MessageBus::new(String::from("1M5"));
    let a_id = bus.create_endpoint();
    let mut end_a = bus.endpoint(a_id).unwrap();
    info!("A: {}", end_a.addr());
    let b_id = bus.create_endpoint();
    let mut end_b = bus.endpoint(b_id).unwrap();
    info!("B: {}", end_b.addr());
    let c_id = bus.create_endpoint();
    let mut end_c = bus.endpoint(c_id).unwrap();
    info!("C: {}", end_c.addr());
    let d_id = bus.create_endpoint();
    let mut end_d = bus.endpoint(d_id).unwrap();
    info!("D: {}", end_d.addr());
    bus.start();

    trace!("Sending test message...");
    let mut env_in = Envelope::new();
    env_in.payload = Option::Some(String::from("Hello World"));
    env_in.slip.add_route(Route::new_msg_route_no_relay(a_id, b_id));


    trace!("1M5 Daemon Stopped.");
}
