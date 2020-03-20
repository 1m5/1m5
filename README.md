# Daemon
A full rewrite of 1M5 Java in Rust. Why?

"1M5 is heavily networked and part of its targeted platforms are tiny electronics with limited resoureces. 
Has great support for threading and keeping threads safe. 
I like Mozilla as an organization. 
Many of the platforms 1M5 is integrating with like Bitcoin, Monero, GNU Radio...were written in C++ and a bit of a pain 
to integrate with Java, so ease of interoperating with them is important. 
C, C++, and Go were considered but the first two don't have the memory protections I wish for (not entirely true with C++) 
and are often written in very terse methods and the latter uses garbage collection which I wanted to avoid - and I don't like Google. 
Plus learning Rust excites me like when I was learning Java in 1998. Neither Clojure, Scala, or other JVM languages really excited me. 
I feel like the 1M5 codebase is going to be much smaller with far less objects yet still get the benefit of object-orientation." - Brian Taylor (ObjectOrange)

Contains a main.rs file to launch it independently as its own daemon.