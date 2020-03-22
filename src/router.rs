/// Router module
use ra_common::models::{Envelope, Route, Network, NetworkId};
use onemfive_common::ManCon;
use std::collections::HashMap;

/// Primary method for ensuring uncensored communications.
pub struct Router {
    _nets: HashMap<NetworkId,Network>
}

impl Router {
    pub fn new() -> Box<Router> {
        Box::new(Router {
           _nets: HashMap::new()
        })
    }
    /// Initialize Router by instantiating a Network for each network client to support then start
    /// each network client's discovery process.
    pub fn init() {

    }
    /// Update Route in Envelope by determining current state followed by expected route decision.
    ///
    /// When ManCon not provided or is set to Unknown,
    /// return an error to the last address stating requirement.
    ///
    /// When the current_route has been routed (env.slip.current_route._routed = true),
    /// env.slip.end_route(current_route) is called providing the next route if another route is available.
    ///
    /// When the route has not been routed (route._routed = false) and the route._to address
    /// is different than its route._destination, then a relay is requested.
    ///
    ///
    ///
    pub fn route(&mut self, env: Envelope) {
        let mancon_header = env.headers.get("mancon");
        let current_route = env.slip.current_route();

    }
}