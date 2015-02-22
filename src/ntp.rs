//! Tools to retrieve Internet-time using NTP protocol.

use std::num::Float;
use time::Timespec;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

const NTP_CLIENT: u8 = 3;
const NTP_HEADER_SIZE: usize = 48; // 12 words
const NTP_TO_UNIX_EPOCH: i64 = 2208988800;

const LEAP_SHIFT: i32 = 6;
const VERSION_SHIFT: i32 = 3;

#[derive(Debug)]
pub struct NTPTimestamp {
    seconds: u32,
    fraction: u32
}

impl NTPTimestamp {
    pub fn as_timespec(&self) -> Timespec {
        Timespec{sec: (self.seconds as i64) - NTP_TO_UNIX_EPOCH,
                 nsec: (((self.fraction as f64) / 2f64.powi(32) ) / 1e-9) as i32}
    }
}

#[derive(Debug)]
pub struct NTPHeader {
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
    pub transmit_timestamp: NTPTimestamp,
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

        // TODO: since Vec still implements old_io::Write trait the next 4 lines does not compile
        //vec.write_u8(self.leap << LEAP_SHIFT | self.version << VERSION_SHIFT | self.mode).unwrap();
        //vec.write_u8(self.stratum).unwrap();
        //vec.write_u8(self.poll).unwrap();
        //vec.write_u8(self.precision).unwrap();

        // TODO: remove workaround when possible
        let first_word = ((self.leap << LEAP_SHIFT | self.version << VERSION_SHIFT | self.mode) as u32) << 24 |
                         ((self.stratum) as u32) << 16 |
                         ((self.poll) as u32) << 8 |
                         (self.precision) as u32;
        vec.write_u32::<BigEndian>(first_word).unwrap();

        vec.write_u32::<BigEndian>(self.root_delay).unwrap();
        vec.write_u32::<BigEndian>(self.root_dispersion).unwrap();
        vec.write_u32::<BigEndian>(self.reference_id).unwrap();
        vec.write_u32::<BigEndian>(self.reference_timestamp.seconds).unwrap();
        vec.write_u32::<BigEndian>(self.reference_timestamp.fraction).unwrap();
        vec.write_u32::<BigEndian>(self.origin_timestamp.seconds).unwrap();
        vec.write_u32::<BigEndian>(self.origin_timestamp.fraction).unwrap();
        vec.write_u32::<BigEndian>(self.receive_timestamp.seconds).unwrap();
        vec.write_u32::<BigEndian>(self.receive_timestamp.fraction).unwrap();
        vec.write_u32::<BigEndian>(self.transmit_timestamp.seconds).unwrap();
        vec.write_u32::<BigEndian>(self.transmit_timestamp.fraction).unwrap();
        vec
    }

    pub fn decode(size: usize, buf: & [u8]) -> NTPHeader {
        let mut reader = Cursor::new(buf);
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
        header.root_delay = reader.read_u32::<BigEndian>().unwrap();
        header.root_dispersion = reader.read_u32::<BigEndian>().unwrap();
        header.reference_id = reader.read_u32::<BigEndian>().unwrap();
        header.reference_timestamp.seconds = reader.read_u32::<BigEndian>().unwrap();
        header.reference_timestamp.fraction = reader.read_u32::<BigEndian>().unwrap();
        header.origin_timestamp.seconds = reader.read_u32::<BigEndian>().unwrap();
        header.origin_timestamp.fraction = reader.read_u32::<BigEndian>().unwrap();
        header.receive_timestamp.seconds = reader.read_u32::<BigEndian>().unwrap();
        header.receive_timestamp.fraction = reader.read_u32::<BigEndian>().unwrap();
        header.transmit_timestamp.seconds = reader.read_u32::<BigEndian>().unwrap();
        header.transmit_timestamp.fraction = reader.read_u32::<BigEndian>().unwrap();

        header
    }
}
