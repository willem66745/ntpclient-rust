extern crate time;
extern crate ntpclient;

use time::at;
use ntpclient::ntp::receive_network_timestamp;

fn main() {
    let timestamp = match receive_network_timestamp() {
        Ok(s) => s,
        Err(e) => panic!("Error retrieving network timestamp: {}", e),
    };

    println!("{}", at(timestamp).asctime());
}
