<div align="center">
  <img src="https://1m5.io/assets/img/bg/official2.jpg"  />

  <h1>1M5</h1>

  <p>
    <strong>Invisible Matrix Services - Uncensored Communications</strong>
  </p>
  <p>
    <a href="https://travis-ci.com/1m5/1m5-daemon"><img alt="build" src="https://img.shields.io/travis/1m5/1m5-daemon"/></a>
    <a href="https://crates.io/crates/onemfive-daemon"><img alt="Crate Info" src="https://img.shields.io/crates/v/onemfive-daemon.svg"/></a>
    <a href="https://docs.rs/crate/onemfive_daemon/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-onemfive_daemon-green"/></a>
  </p>
  <p>
    <a href="https://github.com/1m5/1m5-daemon/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/1m5/1m5-daemon"/></a>
    <a href="https://1m5.io/ks/publickey.objectorange@1m5.io.asc"><img alt="PGP" src="https://img.shields.io/keybase/pgp/objectorange"/></a>
  </p>
  <p>
    <img alt="commits" src="https://img.shields.io/crates/d/onemfive-daemon"/>
    <img alt="repo size" src="https://img.shields.io/github/repo-size/1m5/1m5-daemon"/>
  </p>
  <p>
    <img alt="num lang" src="https://img.shields.io/github/languages/count/1m5/1m5-daemon"/>
    <img alt="top lang" src="https://img.shields.io/github/languages/top/1m5/1m5-daemon"/>
    <a href="https://blog.rust-lang.org/2020/03/12/Rust-1.42.html"><img alt="Rustc Version 1.42+" src="https://img.shields.io/badge/rustc-1.42+-green.svg"/></a>
  </p>

  <h4>
    <a href="https://1m5.io">Info</a>
    <span> | </span>
    <a href="https://docs.rs/crate/onemfive_daemon/">Docs</a>
    <span> | </span>
    <a href="https://github.com/1m5/1m5-daemon/blob/master/CHANGELOG.md">Changelog</a>
    <span> | </span>
    <a href="https://1m5.io/technical-roadmap.html">Roadmap</a>
  </h4>
</div>

## Donate

[![Become a patron](https://resolvingarchitecture.io/images/become_a_patron_button.png)](https://www.patreon.com/1m5) or [request](mailto:info@1m5.io) a BTC or XMR address to donate crypto directly!

## About

A secure open-source decentralized censorship-resistant peer-to-peer application platform with end-to-end encryption
and anonymity as a base layer for creating easy to build and use secure decentralized peer-to-peer
applications requiring no server connections that can be used around the world by any person looking
to protect their communications and personal data from unethical monitoring, interception, intrusion,
and censorship.

Currently moving Java-based version 0.6.5-SNAPSHOT to Rust embedded within a Linux-based operating system.

## Why rewrite in Rust?

* 1M5 is heavily networked and part of its targeted platforms are tiny electronics with limited resources.
* Rust has great support for threading and keeping threads safe. 
* Mozilla has a great reputation, Oracle nor Google does not. 
* Rust was open sourced from the beginning therefore not likely to cause future issues with copyright.
* Many of the platforms 1M5 is integrating with like Bitcoin, Monero, GNU Radio were written in C++ and a bit of a pain 
to integrate with Java, so ease of interoperating with them is important. 
* C, C++, and Go were considered but the first two don't have the memory protections I wish for (not entirely true with C++) 
and are often written in very terse methods and the latter uses garbage collection which I wanted to avoid and was written by Google who is slowly restricting Android development. 
* Learning Rust is exciting. Neither Clojure, Scala, or other JVM languages have produced much of that affect.
* 1M5 codebase is going to be much smaller with far less objects yet still get the benefit of object-orientation.

Notes: a re-design in routing was also necessary as the original routing across networks exposed the origination 
to the destination and every 1M5 relay in between. It's desired to change this to onion routing so that 1M5 relays
are not aware of whom is an originator nor destination. This requires building up lease sets similar to how TOR/I2P works.

## Current TODO:

* Use this [example](https://github.com/saschagrunert/webapp.rs) turn build the front-end using Wasm and Rust.

## Licensing

In the interests of ensuring the 1M5 mission, all copyright automatically imposed on the 1M5 project by any and all people
and organizations are removed and thus nullified. We believe this to be encompassed in the [Unlicense](https://unlicense.org/) statement.
All 1M5 services and sensors are created with the Unlicense statement by default unless otherwise specified,

Bouncycastle is embedded in 1M5 Core and its MIT-like license is [here](http://www.bouncycastle.org/licence.html).

## Authors / Developers

* objectorange (Brian Taylor) - [GitHub](https://github.com/objectorange) | [LinkedIn](https://www.linkedin.com/in/decentralizationarchitect/) | objectorange@1m5.io PGP: DD08 8658 5380 C7DF 1B4E 04C2 1849 B798 CF36 E2AF | brian@resolvingarchitecture.io PGP: 2FA3 9B12 DA50 BD7C E43C 3031 A15D FABB 2579 77DC
* evok3d (Amin Rafiee) - [Site](https://arafiee.com/) | PGP: D921 C2EE 60BA C264 EA40 4DC5 B6F8 2589 96AA E505
* azad (Erbil Kaplan) - [LinkedIn](https://www.linkedin.com/in/erbil-kaplan-b8971b18/) | PGP: 2EBC 2239 E9B8 2BCA 7176 77FE FD80 A0C2 95FD EBAC
* z0??0z - Hard to reach; found in I2P and 1M5 IRC using I2P

## Opportunities

[**Freedom of Speech**](https://en.wikipedia.org/wiki/Freedom_of_speech) - a principle that supports the freedom of
an individual or a community to articulate their opinions and ideas without fear of retaliation, censorship,
or sanction. The term "freedom of expression" is sometimes used synonymously but includes any act of seeking,
receiving, and imparting information or ideas, regardless of the medium used.

[**Censorship**](https://en.wikipedia.org/wiki/Censorship) - the suppression of speech, public communication,
or other information, on the basis that such material is considered objectionable, harmful, sensitive,
politically incorrect or "inconvenient" as determined by government authorities or by community consensus.

Constraining the free flow of information between people is a direct threat to our freedom and censorship of
communications on-line is growing world-wide.

- https://internetfreedomwatch.org/timeline/
- https://www.wired.com/2017/04/internet-censorship-is-advancing-under-trump/
- https://rsf.org/en/news/more-100-websites-blocked-growing-wave-online-censorship

On-line communications are censored at the point of entrance by Internet Service Providers (ISP).
They act as gateways to the internet providing governments control over speech by having the
ability to restrict usage and track people's usage via their leased IP addresses. In order to make tracking usage much more
difficult, tools have come out that provide techniques called onion-/garlic-routing where the source and destinations of
internet routes can not be determined without breaking encryption, a very expensive feat, sometimes impossible today when
considering the encryption algorithms used.

Two primary tools today that support this are Tor and I2P. Tor provides a browser
that makes it easier to use while I2P is much less known. Both are complementary in that Tor was designed for browsing
today's current web sites anonymously. I2P was designed for peer-to-peer communications within I2P. Neither have good
APIs for developers to embed in their products making uptake slow for many applications.

A third tool on the horizon is one that completely circumvents ISPs by not using them. They're called direct wireless
mesh networks and they can communicate directly device-to-device using technologies such as WiFi Direct. Firechat is an
example used during the 2014 Hong Kong protests after the Chinese government threatened to shutdown the internet in that
area. New mesh solutions are popping up including RightMesh that seek to improve on earlier designs. But the technology
is still in its infancy and needs to be pulled into ever day applications more easily once they've matured.

Even getting these technologies in wide use doesn't solve the problem of online censorship. People in governments, corporations, and
other thieves are constantly finding ways to circumvent these technologies to censor and steal information.

In addition:

- Most organizations today (e.g. Tech, Banks, Governments, Hospitals) track, persist, and use our behavior for their profit not ours.
- Centralized organizations are major targets for theft.
- Closed source software can easily contain hidden back doors for thieves to access our information without our knowledge and many open source applications have closed source libraries embedded in them.
- Whistleblowers, the abused, visible minorities, and a myriad of other people could be emboldened by anonymity to speak out in a manner that would otherwise be unavailable if they were forced to identify themselves.
- Decentralized applications and cryptocurrencies like Bitcoin are helping to wrestle some control from centralized organizations although they are largely used on servers and distributed ledgers are still logically centralized and difficult to maintain anonymity at the network layer.
- Smartphone ownership around the world is greater than PC ownership.
- Smartphones, our primary means of global communication and collaboration, are weak in maintaining our anonymity and privacy - critical to ensuring individual freedom.

## Solution

1M5 works to solve these issues by providing three primary benefits.

1. Intelligent Censorship-Resistant Anonymous Router embedding Tor, I2P, Direct Wireless Ad-Hoc Networks, and other
networks, using them intelligently as one dynamic censorship-resistant, end-to-end encrypted, anonymous mesh network.
2. Offers access to commonly desired decentralized services in an anonymous fashion including
self-sovereign decentralized identities (DID), Bitcoin, and other privacy preserving services.
3. Provides easy to use APIs for developers to embed in their applications to facilitate up-take.

### Routing

We provide a Maneuvering Condition (ManCon) setting to signal what level of maneuvering is likely required to prevent censorship.
This should be set by the end user based on their circumstances. They should also be made aware of recommended ManCon
levels for the jurisdiction they are currently in. These ManCon levels are largely based on [Press Freedom Index](https://en.wikipedia.org/wiki/Press_Freedom_Index)
and updated on the [ManCon page](https://1m5.io/mancon.html).

Setting the ManCon level manually by the end user informs the codebase what based level of ManCon should be used although
final ManCon is determined by blocks encountered during routing and thus how to ratchet up resistance as these blocks occur.

For developers using the API, all requests for services, e.g. Bitcoin, require an Envelope with a ManCon level set. This ManCon level decides
what base level of privacy is desired. Options are Low, Medium, High, Very High, Extreme, and Neo.
All P2P communications use High ManCon as the default resulting in the use of I2P with latency expectations between 200 milliseconds and 2 seconds.
This is the default setting in the Envelope.

When making web requests, remember to set the appropriate ManCon level otherwise all web requests will use the HIGH ManCon
level thereby routing all web requests through the I2P layer to 1M5 nodes with Tor enabled resulting in higher than might be
expected latencies yet very censorship-resistant and private page views. This is the ideal setup for people in China as an example wishing
to view web pages globally without censorship and without getting a knock on their door where Tor is getting heavily blocked.

#### LOW - MANCON 5

Open/normal SSL based communications with no expected censorship or privacy intrusion attempts is the norm.

Examples: Norway, Iceland, Costa Rica, Jamaica, Ireland

* Web: I2P used for .i2p addresses and Tor for other web requests including .onion addresses. If that fails, it will be assumed that the site is down.
* P2P (Messenger): I2P used unless found to be blocked. Then Tor will be used as a tunnel to a peer that has I2P enabled.
If Tor blocked, will ratchet up to 1DN for assistance.

Expect latencies of 500 milliseconds to 2 seconds unless 1DN is needed.

#### MEDIUM - MANCON 4

Normal censorship attempts by states on reading news (public web sites getting blocked, government shutdown of cloud cdn content).
Many moving towards using Tor and/or VPNs although no fear of circumventing censorship attempts.

Examples: Australia, United States, France, United Kingdom

* Web: Tor will be used. If that fails, the request will be forwarded to other peers until a peer can make the request
returning the result directly back to the requesting peer. If those fail, it will be assumed that the site is down.
* P2P unchanged

Expect latencies of 500 milliseconds to 4 seconds unless 1DN is needed.

#### HIGH - MANCON 3

Tor and VPNs are beginning to get blocked. Many beginning to move to I2P. Some self-censorship likely. This is the default setting for 1M5.

Examples: Brazil, Greece, Poland, Panama, Nicaragua

* Web: will use an I2P peer that has access to Tor to make the request.
* P2P unchanged

Expect latencies of 4-10 seconds.

#### VERYHIGH - MANCON 2

I2P is getting attacked slowing the network and people are beginning to get threatened for circumventing censorship attempts resulting in self-censorship.

Examples: Mexico, Venezuela, Russia, India, Turkey

* Web: will use I2P with random delays to forward all requests to a 1M5 peer with Tor access at a lower ManCon. If both I2P and
Tor blocked at end user, 1DN will be used to find a 1M5 peer at a lower ManCon to fulfill the request.
* P2P: will use I2P with random delays of 4-10 seconds. If I2P gets blocked, will attempt to use Tor as a tunnel. If that is blocked,
1DN will be used.

Expect latencies of 6-16 seconds unless 1DN used which could result in very large latencies where only asynchronous messaging
(e.g. Email) and asynchronous web requests are plausible.

#### EXTREME - MANCON 1

Internet has been blocked for end user, e.g. local cellular service towers shutdown or provider turns off access and/or
threats of imprisonment and/or death are made to induce self-censorship with actual evidence of enforcement.

Examples: China, North Korea, East Turkestan, Iran, Saudi Arabia, Iraq, Egypt

* Web: 1DN will be used to forward requests to Tor users with a lower ManCon to fulfill the request.
* P2P: 1DN peers will be used until a peer with I2P access can route the request.

Expect wide-ranging latencies.

#### NEO - MANCON 0

Whistleblower with deep state top secrets or investigative journalist with life-threatening information.

* Web: 1DN is used to forward requests to a peer that will then request another peer using I2P with high delays to make the Tor request.
* P2P: 1DN is used to forward a message through a random number and combination of 1DN/I2P peers at random delays of up to 90 seconds
at the I2P layer and up to 3 months at the 1M5 layer. A random number of copies (3 min 12 max) of the message are sent out
with the end user having a 12 word mnemonic passphrase as the only key to the data.

Wide-ranging latencies but highest privacy and censorship-resistance.

## Design

1M5 is composed of a Service-Oriented Architecture (SOA) design using a minimalistic service bus for micro-services,
a Staged Event-Driven Architecture (SEDA) design for asynchronous multi-threaded inter-service communications,
a service registry, internal core services, and a number of networked services for advanced intelligent interaction with peers.

This software runs as an operating system daemon providing:

* a network router for routing Packets through networks to ensure censorship resistance
* network clients for each network supported (e.g. I2P, TOR, VPN) handling network interactions
* a service router for routing Envelopes through intra-/inter-services
* keychain service for interacting with keychain usb key like Nitrokey
* infovault service for persistence to/from personal usb key like Apricorn
* content distribution service for distributing information across network for backup and performance
* decentralized identification service for building reputation

Packets are used to route Envelopes across networks. 

Envelopes are routed within the destination node's 1M5 Daemon process to fulfill a request.

``` 

----------- OS Process --------------    ----------- OS Process --------------
-  -------------------------------  -    -  -------------------------------  -
-  -         SEDA Bus            -  -    -  - [API]    Browser            -  -
-  -------------------------------  -    -  -------------------------------  -
-                                   -    -------------------------------------
-  -------  ----------------------  -    
-  -     -  - Network Router     -  -    ------------ OS Process -------------
-  -     -  ----------------------  -    -  -------------------------------  -
-  -     -                          -    -  - [API]    Messenger          -  -
-  -     -  ----------------------  -    -  -------------------------------  -
-  -     -  - LiFi Client        -  -    -------------------------------------
-  -     -  ----------------------  -    
-  -     -                          -    ----------- OS Process --------------
-  -  S  -  ----------------------  -    -  -------------------------------  -
-  -  e  -  - Bluetooth Client   -  -    -  - [API]       CLI             -  -
-  -  r  -  ----------------------  -    -  -------------------------------  -
-  -  v  -                          -    -------------------------------------
-  -  i  -  ----------------------  -
-  -  c  -  - WiFi Direct Client -  -    ----------- OS Process --------------
-  -     -  ----------------------  -    -  -------------------------------  -
-  -  B  -                          -    -  - [API] External Service      -  -
-  -  u  -  ----------------------  -    -  -------------------------------  -
-  -  s  -  - HTTPS Client       -  -    -------------------------------------
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - VPN Client         -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - TOR Client         -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - I2P Client         -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - Locha Mesh Client  -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - Satellite Client   -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - FS Radio Client    -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - API (Localhost)    -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - Service Router     -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - Keychain           -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - InfoVault          -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - CDN                -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - DID                -  -
-  -     -  ----------------------  -
-  -     -                          -
-  -     -  ----------------------  -
-  -     -  - MongoDB            -  -
-  -     -  ----------------------  -
-  -------                          -
-------------------------------------

```

### Components

External processes like the Browser, Messenger, and CLI can use the API to send/receive messages via services.
The API packs up the service request into an Envelope and transmits it to the SEDA Bus via its inbound external channel.
The SEDA Bus retrieves this message asynchronously sending it to the Service Bus.
If the message is a Packet, it sends it to the Network Router.
If the message is an Envelope, it sends it to the Service Router.

#### [SEDA Bus](https://github.com/resolvingarchitecture/seda-bus)

#### [Service Bus](https://github.com/resolvingarchitecture/service-bus)

#### [LiFi Client](https://github.com/resolvingarchitecture/lifi-client)

#### [Bluetooth Client](https://github.com/resolvingarchitecture/bluetooth-client)

#### [WiFi Direct Client](https://github.com/resolvingarchitecture/wifidirect-client)

#### HTTPS Client

#### VPN Client

#### [TOR Client](https://github.com/resolvingarchitecture/tor-client)

#### [I2P Client](https://github.com/resolvingarchitecture/i2p-client)

#### [Locha Mesh Client](https://github.com/resolvingarchitecture/locha-mesh-client)

#### Satellite Client

#### [FS Radio Client](https://github.com/resolvingarchitecture/gnuradio-client)

#### [Service Router](https://github.com/resolvingarchitecture/service-router)

#### [Keychain](https://github.com/resolvingarchitecture/keychain)

#### InfoVault

#### [CDN](https://github.com/resolvingarchitecture/content-distribution-network)

#### [DID](https://github.com/resolvingarchitecture/decentralized-identification)

#### MongoDB
