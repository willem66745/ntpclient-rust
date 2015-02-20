#![feature(old_io,net,core)]
use std::net::UdpSocket;

const NTP_SERVER: &'static str = "sundial.columbia.edu:123";
const UDP_LOCAL: &'static str = "0.0.0.0:35000";

const NTP_CLIENT: u8 = 3;

const LEAP_SHIFT: i32 = 6;
const VERSION_SHIFT: i32 = 3;

struct NTPTimestamp {
    seconds: u32,
    fraction: u32
}

struct NTPHeader {
    leap: u8,
    version: u8,
    mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    reference_id: u32,
    reference_timestamp: NTPTimestamp,
    origin_timestamp: NTPTimestamp,
    receive_timestamp: NTPTimestamp,
    transmit_timestamp: NTPTimestamp,
}

impl NTPHeader {
    pub fn new() -> NTPHeader {
        NTPHeader {
            leap: 0,
            version: 3,
            mode: NTP_CLIENT,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay : 0,
            root_dispersion : 0,
            reference_id : 0,
            reference_timestamp : NTPTimestamp{seconds:0, fraction:0},
            origin_timestamp : NTPTimestamp{seconds:0, fraction:0},
            receive_timestamp : NTPTimestamp{seconds:0, fraction:0},
            transmit_timestamp : NTPTimestamp{seconds:0, fraction:0}
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();

        vec.write_u8(self.leap << LEAP_SHIFT | self.version << VERSION_SHIFT | self.mode).unwrap();
        vec.write_u8(self.stratum).unwrap();
        vec.write_u8(self.poll).unwrap();
        vec.write_u8(self.precision).unwrap();
        vec.write_be_u32(self.root_delay).unwrap();
        vec.write_be_u32(self.root_dispersion).unwrap();
        vec.write_be_u32(self.reference_id).unwrap();
        vec.write_be_u32(self.reference_timestamp.seconds).unwrap();
        vec.write_be_u32(self.reference_timestamp.fraction).unwrap();
        vec.write_be_u32(self.origin_timestamp.seconds).unwrap();
        vec.write_be_u32(self.origin_timestamp.fraction).unwrap();
        vec.write_be_u32(self.receive_timestamp.seconds).unwrap();
        vec.write_be_u32(self.receive_timestamp.fraction).unwrap();
        vec.write_be_u32(self.transmit_timestamp.seconds).unwrap();
        vec.write_be_u32(self.transmit_timestamp.fraction).unwrap();
        vec
    }
}

fn main() {
    let header = NTPHeader::new();
    let message = header.encode();

    let socket = match UdpSocket::bind(UDP_LOCAL) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    match socket.send_to(message.as_slice(), (NTP_SERVER)) {
        Ok(s) => s,
        Err(e) => panic!("Unable to send datagram: {}", e),
    };

    let mut buf = [0u8; 1000];

    match socket.recv_from(buf.as_mut_slice()) {
        Ok((amt, src)) => {
            println!("Got {} bytes from {}.", amt, src);
        },
        Err(e) => panic!("couldn't receive data: {}", e),
    };

    drop(socket);
}
