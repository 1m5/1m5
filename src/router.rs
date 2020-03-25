/// Router module
use ra_common::models::{Envelope, Route, Network, NetworkId, Packet, Service};
use onemfive_common::ManCon;
use std::collections::HashMap;

use log::{trace,info,warn};

use i2p_client::I2PClient;
use tor_client::TORClient;
use bluetooth_client::BluetoothClient;
use std::convert::TryFrom;
use https_client::HTTPSClient;
use vpn_client::VPNClient;

/// Primary method for ensuring uncensored communications.
pub struct NetworkRouter {
    _https: Box<HTTPSClient>,
    _vpn: Box<VPNClient>,
    _tor: Box<TORClient>,
    _i2p: Box<I2PClient>,
    _bt: Box<BluetoothClient>,
    _initialized: bool
}

impl NetworkRouter {
    pub fn new() -> Box<NetworkRouter> {
        Box::new(NetworkRouter {
            _https: HTTPSClient::new(),
            _vpn: VPNClient::new(),
            _tor: TORClient::new(),
            _i2p: I2PClient::new(),
            _bt: BluetoothClient::new(),
            _initialized: false
        })
    }
    /// Initialize Router by instantiating a Network for each network client to support then start
    /// each network client's discovery process.
    pub fn init(&mut self) {
        info!("{}","Initializing Network Router...");
        self._https.init();
        self._vpn.init();
        self._tor.init();
        self._i2p.init();
        self._bt.init();
        self._initialized = true;
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
    pub fn route(&mut self, packet: &mut Packet) {
        if !self._initialized { self.init(); }
        if packet.headers.get("mancon").is_none() {
            // Add error indicating ManCon is required, sending it back to from address
            packet.headers.insert(String::from("err"),String::from("ManCon required in packet header with name = mancon"));
            return;
        }
        let mancon_str = packet.headers.get("mancon").unwrap();
        let mancon_num = mancon_str.parse::<u8>().unwrap();
        let res = ManCon::try_from(mancon_num);
        if res.is_ok() {
            let mancon = res.unwrap();
            match mancon {
                ManCon::Low => {self.low(packet);},
                ManCon::Medium => {self.med(packet);},
                ManCon::High => {self.high(packet);},
                ManCon::VeryHigh => {self.vhigh(packet);},
                ManCon::Extreme => {self.ex(packet);},
                ManCon::Neo => {self.neo(packet);},
                _ => {self.none(packet);}
            }
        }
    }

    /// HTTPS: 0 Relays
    fn none(&mut self, packet: &mut Packet) {
        self._https.handle(packet);
    }

    /// VPN: 1 Relay (~200ms) / 2 Round-trip (~600ms)
    fn low(&mut self, packet: &mut Packet) {
        self._vpn.handle(packet);
    }

    /// TOR: 3 Relays (~600ms) / 6 Round-trip (~1.4sec)
    fn med(&mut self, packet: &mut Packet) {
        self._tor.handle(packet);
    }

    /// I2P: 4 Relays (~800ms) / 8 Round-trip (~1.8sec)
    fn high(&mut self, packet: &mut Packet) {
        self._i2p.handle(packet);
    }

    /// I2P w/ Random Configurable Delays: 4 Relays (~800ms-6minutes) / 8 Round-trip (~1.8sec-12minutes)
    fn vhigh(&mut self, packet: &mut Packet) {
        packet.max_delay = 90 * 1000; // 90 seconds
        self._i2p.handle(packet);
    }

    /// 1DN + I2P w/ Random Configurable Delays: 5 Relays (~1sec-6minutes) / 10 Round-trip (~2sec-1day)
    fn ex(&mut self, packet: &mut Packet) {
        packet.max_delay = 90 * 1000; // 90 seconds
        packet.headers.insert(String::from("relay_net"),String::from("5"));
        self._bt.handle(packet);
    }

    /// 1DN Only w/ Random Configurable Delays: 5-10 Relays (~50sec-50days) / 10-20 Round-trip (~100sec-100days)
    fn neo(&mut self, packet: &mut Packet) {
        packet.min_delay = 10 * 1000; // 10 seconds
        packet.max_delay = 50 * 24 * 60 * 60 * 1000; // 50 days
        packet.headers.insert(String::from("1dn_only"),String::from("true"));
        self._bt.handle(packet);
    }
}
