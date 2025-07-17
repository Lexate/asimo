use crate::error::Result;

pub struct Msgtype {
    msgtype: u16, // Sometimes u8 sometimes u16
    subcmd: u8,   // Always u8
}

impl Msgtype {
    pub fn new(msgtype: u16, subcmd: u8) -> Self {
        Self { msgtype, subcmd }
    }

    pub fn get_msgtype(&self) -> Vec<u8> {
        // If the messagetype is larger than 4000 it is used for the amg3
        // protocol and the messagetype should be or:ed with 0x8000 before
        // it is sent over the wire.
        // TODO: Rename this method to indicate that it is for use by the serializer
        let msgtype = match self.msgtype {
            4000.. => self.msgtype | 0x8000,
            _ => self.msgtype,
        };
        let bytes = msgtype.to_be_bytes();
        println!("{:?}", bytes);
        if bytes[0] == 0x00 {
            bytes[1..].to_vec()
        } else {
            bytes.to_vec()
        }
    }

    pub fn get_subcmd(&self) -> &u8 {
        &self.subcmd
    }
}

pub trait Hcp {
    fn get_msgtype_subcmd() -> Msgtype
    where
        Self: Sized;
}

pub trait HcpType {
    fn u8_to_variant(value: u8) -> Result<impl HcpType>;

    fn to_u8(&self) -> u8;
}
