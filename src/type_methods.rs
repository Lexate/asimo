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
        let bytes = self.msgtype.to_le_bytes();

        if bytes[1] == 0x00 {
            bytes[..1].to_vec()
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
