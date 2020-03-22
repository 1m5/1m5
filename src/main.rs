extern crate log;
extern crate simple_logger;

pub mod router;

use log::{trace,info};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting 1M5 Daemon...");

    // Initialize the Service Bus with configuration information

    // Start Service Bus with SEDA Bus

    // Determine what applicable services are available registering each with the Service Bus

    // Start Router Service

    trace!("1M5 Daemon Stopped.");
}
