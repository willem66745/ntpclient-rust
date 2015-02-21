use std::net::UdpSocket;
use std::old_io::BufReader;
use time::Timespec;
use std::num::Float;
use std;

const NTP_SERVER: &'static str = "sundial.columbia.edu:123";
const UDP_LOCAL: &'static str = "0.0.0.0:35000";

const NTP_CLIENT: u8 = 3;
const NTP_HEADER_SIZE: usize = 48; // 12 words
const NTP_TO_UNIX_EPOCH: i64 = 2208988800;

const LEAP_SHIFT: i32 = 6;
const VERSION_SHIFT: i32 = 3;

#[derive(Debug)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32
}

impl NTPTimestamp {
    fn as_timespec(&self) -> Timespec {
        Timespec{sec: (self.seconds as i64) - NTP_TO_UNIX_EPOCH, nsec: (((self.fraction as f64) / 2f64.powi(32) ) / 1e-9) as i32}
    }
}

#[derive(Debug)]
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
    fn new() -> NTPHeader {
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

    fn encode(&self) -> Vec<u8> {
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

    fn decode(size: usize, buf: & [u8]) -> NTPHeader {
        let mut reader = BufReader::new(buf);
        let mut header = NTPHeader::new();

        if size < NTP_HEADER_SIZE {
            panic!("{} bytes expected in NTP header; {} bytes received", NTP_HEADER_SIZE, size);
        }

        let leap_version_mode = reader.read_u8().unwrap();
        header.leap = (leap_version_mode >> LEAP_SHIFT) & 0b11;
        header.version = (leap_version_mode >> VERSION_SHIFT) & 0b111;
        header.mode = leap_version_mode & 0b111;
        header.stratum = reader.read_u8().unwrap();
        header.poll = reader.read_u8().unwrap();
        header.precision = reader.read_u8().unwrap();
        header.root_delay = reader.read_be_u32().unwrap();
        header.root_dispersion = reader.read_be_u32().unwrap();
        header.reference_id = reader.read_be_u32().unwrap();
        header.reference_timestamp.seconds = reader.read_be_u32().unwrap();
        header.reference_timestamp.fraction = reader.read_be_u32().unwrap();
        header.origin_timestamp.seconds = reader.read_be_u32().unwrap();
        header.origin_timestamp.fraction = reader.read_be_u32().unwrap();
        header.receive_timestamp.seconds = reader.read_be_u32().unwrap();
        header.receive_timestamp.fraction = reader.read_be_u32().unwrap();
        header.transmit_timestamp.seconds = reader.read_be_u32().unwrap();
        header.transmit_timestamp.fraction = reader.read_be_u32().unwrap();

        header
    }
}

pub fn receive_network_timestamp() -> Result<Timespec, std::io::Error> {
    let header = NTPHeader::new();
    let message = header.encode();

    let socket = try!(UdpSocket::bind(UDP_LOCAL));

    try!(socket.send_to(message.as_slice(), (NTP_SERVER)));

    let mut buf = [0u8; 1000];

    let (amt, _) = try!(socket.recv_from(buf.as_mut_slice()));

    drop(socket);

    let header = NTPHeader::decode(amt, &buf);

    Ok(header.transmit_timestamp.as_timespec())
}

