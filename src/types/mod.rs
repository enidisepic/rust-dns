mod dns_class;
mod dns_type;
mod header;
mod message;
mod resource_record;

use std::io;
use std::io::{Read, Write};

pub trait RDataEncode {
    fn encode(&self, writer: &mut impl Write) -> io::Result<()>;
}
pub trait RDataDecode: Sized {
    fn decode(reader: &mut impl Read) -> io::Result<Self>;
}

pub(crate) use crate::types::dns_class::*;
pub(crate) use crate::types::dns_type::*;
pub(crate) use crate::types::header::*;
pub(crate) use crate::types::message::*;
pub(crate) use crate::types::resource_record::*;
