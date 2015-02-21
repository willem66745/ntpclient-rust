#![feature(old_io,net,core,io)]
extern crate time;

mod ntp;

use std::net::{UdpSocket, lookup_host, SocketAddr};
use time::Timespec;

const NTP_PORT: u16 = 123;
const UDP_LOCAL: &'static str = "0.0.0.0:35000";

/// `receive_network_timestamp` retrieves the current from the Internet using
/// the NTP protocol.
///
/// # Arguments
///
/// * `host` - The NTP server (i.e. sundial.columbia.edu).
pub fn receive_network_timestamp(host: &str) -> Result<Timespec, std::io::Error> {
    let host = try!(lookup_host(host)).next().unwrap();
    let addr = SocketAddr::new(try!(host).ip(), NTP_PORT);
    let header = ntp::NTPHeader::new();
    let message = header.encode();

    let socket = try!(UdpSocket::bind(UDP_LOCAL));

    try!(socket.send_to(message.as_slice(), &addr));

    let mut buf = [0u8; 1000];

    // TODO: Rust doesn't support timeouts yet
    let (amt, _) = try!(socket.recv_from(buf.as_mut_slice()));

    drop(socket);

    let header = ntp::NTPHeader::decode(amt, &buf);

    Ok(header.transmit_timestamp.as_timespec())
}

#[test]
fn receive_timestamp() {
    const NTP_SERVER: &'static str = "sundial.columbia.edu";

    receive_network_timestamp(NTP_SERVER).unwrap();
}
