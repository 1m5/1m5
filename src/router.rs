/// Router module
use ra_common::models::{Envelope, Route, Network, NetworkId, Packet, Service};
use onemfive_common::ManCon;
use std::collections::HashMap;

/// Primary method for ensuring uncensored communications.
pub struct NetworkRouter {
    _nets: HashMap<i16,Network>
}

impl NetworkRouter {
    pub fn new() -> Box<NetworkRouter> {
        Box::new(NetworkRouter {
           _nets: HashMap::new()
        })
    }
    /// Initialize Router by instantiating a Network for each network client to support then start
    /// each network client's discovery process.
    pub fn init(&mut self) {

    }

    /// Route incoming packet.
    ///
    /// When ManCon not provided or is set to Unknown,
    /// return an error to the from address stating requirement.
    ///
    /// When the current_route has been routed (env.slip.current_route._routed = true),
    /// env.slip.end_route(current_route) is called providing the next route if another route is available.
    ///
    /// When the route has not been routed (route._routed = false) and the route._to address
    /// is different than its route._destination, then a relay is requested.
    ///
    /// When relay requested and route._to address is the same as the current Node's address, then
    /// relay has been satisfied.
    ///
    /// ManCon in general:
    ///
    /// NEO: 1DN Only w/ Random Configurable Delays: 10-100 Relays (~2sec-90days) / 20-200 Round-trip (~4sec-90days)
    /// Extreme: 1DN + I2P w/ Random Configurable Delays: 5 Relays (~1sec-6minutes) / 10 Round-trip (~2sec-1day)
    /// VeryHigh:I2P w/ Random Configurable Delays: 4 Relays (~800ms-6minutes) / 8 Round-trip (~1.8sec-12minutes)
    /// High: I2P: 4 Relays (~800ms) / 8 Round-trip (~1.8sec)
    /// Medium: TOR: 3 Relays (~600ms) / 6 Round-trip (~1.4sec)
    /// Low: VPN: 1 Relay (~200ms) / 2 Round-trip (~600ms)
    /// None: HTTPS: 0 Relays
    /// UNKNOWN: Error
    ///
    pub fn route(&mut self, packet: Packet) {
        if packet.headers.get("mancon").is_none() {
            // Add error indicating ManCon is required, sending it back to from address

        }

    }
}
