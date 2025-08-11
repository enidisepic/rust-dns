use crate::types::{DnsClass, DnsHeader, DnsType, RDataDecode, RDataEncode, ResourceRecord};
use std::io;
use std::io::{Read, Write};

struct DnsQuestion {
    labels: Vec<String>,
    question_type: DnsType,
    class: DnsClass,
}

pub struct DnsMessage<T> {
    header: DnsHeader,
    question: DnsQuestion,
    data: Vec<ResourceRecord<T>>,
}

impl<T: RDataEncode> RDataEncode for DnsMessage<T> {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}

impl<T: RDataDecode> RDataDecode for DnsMessage<T> {
    fn decode(reader: &mut impl Read) -> io::Result<Self> {
        todo!()
    }
}
