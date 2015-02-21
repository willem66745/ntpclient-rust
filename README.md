# ntpclient-rust

Very simple Rust library to retrieve a time stamp from a
[NTP](https://tools.ietf.org/html/rfc5905) server.

**NOTE**: These are my first baby steps developing code using Rust.

```rust
use time::{Timespec,at};
use ntpclient::receive_network_timestamp;

let timestamp :Timespec = receive_network_timestamp("sundial.columbia.edu");
println!("Internet time: {}", at(timestamp).asctime());
```
