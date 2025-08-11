use crate::types::{DnsClass, DnsType, RDataDecode, RDataEncode};
use std::io;
use std::io::{Read, Write};
use std::time::Duration;

pub struct ResourceRecord<T> {
    name: String,
    rr_type: DnsType,
    class: DnsClass,
    ttl: Duration, // "Time To Live" - how long we can cache this at most
    data_length: u16,
    data: T,
}

impl<T: RDataEncode> RDataEncode for ResourceRecord<T> {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl<T: RDataDecode> RDataDecode for ResourceRecord<T> {
    fn decode(reader: &mut impl Read) -> io::Result<Self> {
        todo!()
    }
}
