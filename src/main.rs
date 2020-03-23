extern crate log;
extern crate simple_logger;

pub mod router;

use log::{trace,info};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting 1M5 Daemon...");

    // Start the Service/SEDA Bus

    // Register Desired Network Services (e.g. I2P Client)

    // Register Network Router Service

    // Register Desired App Services (e.g. InfoVault)

    trace!("1M5 Daemon Stopped.");
}
