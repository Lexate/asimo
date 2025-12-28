#![allow(unused)]
use std::any::Any;
use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, VariantAccess, Visitor,
};
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::proto::calc_crc;
use crate::type_methods::Hcp;

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
    place: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input, place: 0 }
    }
}

pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a> + Hcp,
{
    let msgtype = T::get_msgtype_subcmd();
    let mut deserializer = Deserializer::from_bytes(b);

    // Check STX
    let stx = deserializer.take_n_bytes(1)?;
    if stx != [0x02] {
        return Err(Error::UnExpectedValue(format!(
            "Expected STX: 2, got: {:?}",
            stx
        )));
    }

    let id = deserializer.take_n_bytes(1)?[0];

    // Amproto
    if id != 0x81 {
        // Check message type
        if id != msgtype.get_msgtype()[0] + 1 {
            return Err(Error::UnExpectedValue(format!(
                "Expected messagetype: {:?}, got: {:?}",
                msgtype.get_msgtype()[0] + 1,
                id,
            )));
        }

        // Check that the reported payload length (hence the - 2) matches.
        let length = deserializer.take_n_bytes(1)?[0];
        if length as usize != deserializer.check_len() - 2 {
            return Err(Error::WrongLength {
                expected: length as usize,
                got: deserializer.check_len() - 2,
            });
        };

        // TODO: Check what this value means, it always seems to be zero.
        assert_eq!(0, deserializer.take_n_bytes(1)?[0]);
    } else {
        let length = u16::from_le_bytes(deserializer.take_n_bytes(2)?.try_into().unwrap());
        if length as usize != deserializer.check_len() {
            return Err(Error::WrongLength {
                expected: length as usize,
                got: deserializer.check_len(),
            });
        };

        // I don't really know what this does
        let trans_id = deserializer.take_n_bytes(1)?[0];

        // Check messagetype
        let first_byte = deserializer.take_n_bytes(1)?[0];
        let recived_msg_type = if first_byte < 0x7F {
            first_byte as u16
        } else {
            u16::from_be_bytes([first_byte, deserializer.take_n_bytes(1)?[0]])
        };

        let expected_msg_type = u16::from_be_bytes(msgtype.get_msgtype().try_into().unwrap());

        if recived_msg_type != expected_msg_type + 1 {
            return Err(Error::UnExpectedValue(format!(
                "Expected messagetype: {:?}, got: {:?}",
                msgtype.get_msgtype(),
                expected_msg_type + 1,
            )));
        };

        // Check payload length
        let length = deserializer.take_n_bytes(1)?[0];
        if length as usize != deserializer.check_len() - 2 {
            return Err(Error::WrongLength {
                expected: length as usize,
                got: deserializer.check_len() - 2,
            });
        };

        // Command result, which I belive is usualy zero
        let command_result = deserializer.take_n_bytes(1)?[0];
        assert_eq!(command_result, 0);
    }

    let t = T::deserialize(&mut deserializer)?;

    // Check to see if the crc:s match
    let crc = deserializer.take_n_bytes(1)?[0];
    let calculated_crc = calc_crc(&b[..b.len() - 2], 1);

    if crc != calculated_crc {
        return Err(Error::Crc {
            recived: crc,
            calculated: calculated_crc,
        });
    }

    // Check ETX
    let etx = deserializer.take_n_bytes(1)?;
    if etx != [0x03] {
        return Err(Error::UnExpectedValue(format!(
            "Expected ETX: 3, got: {:?}",
            etx
        )));
    }

    if deserializer.input.len() == deserializer.place {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

// Helper methods for parsing
impl<'de> Deserializer<'de> {
    fn take_n_bytes(&mut self, n: usize) -> Result<&'de [u8]> {
        if self.place + n > self.input.len() {
            return Err(Error::EndOfMessage);
        }
        let out = &self.input[self.place..(self.place + n)];
        self.place += n;
        Ok(out)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        match self.take_n_bytes(1)?[0] {
            0 => Ok(false),
            1 => Ok(true),
            e => Err(Error::UnExpectedValue(format!(
                "Expected 0 or 1 got {:x?}",
                e
            ))),
        }
    }

    fn check_len(&self) -> usize {
        self.input.len() - self.place
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        false
    }

    // Format not selfdescribing
    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.take_n_bytes(1)?[0].cast_signed())
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(i16::from_le_bytes(
            self.take_n_bytes(2)?.try_into().unwrap(),
        ))
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(i32::from_le_bytes(
            self.take_n_bytes(4)?.try_into().unwrap(),
        ))
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(i64::from_le_bytes(
            self.take_n_bytes(8)?.try_into().unwrap(),
        ))
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.take_n_bytes(1)?[0])
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(u16::from_le_bytes(
            self.take_n_bytes(2)?.try_into().unwrap(),
        ))
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(u32::from_le_bytes(
            self.take_n_bytes(4)?.try_into().unwrap(),
        ))
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(u64::from_le_bytes(
            self.take_n_bytes(8)?.try_into().unwrap(),
        ))
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("f32".to_string()))
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("f64".to_string()))
    }

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("char".to_string()))
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("str".to_string()))
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("string".to_string()))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("bytes".to_string()))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("byte_buf".to_string()))
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("option".to_string()))
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("unit".to_string()))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("unit struct".to_string()))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("newtype struct".to_string()))
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("seq".to_string()))
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("tuple struct".to_string()))
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("map".to_string()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }
}

struct SeqAccess<'a, 'b> {
    deserializer: &'a mut Deserializer<'b>,
    len: usize,
}

impl<'a, 'b> de::SeqAccess<'b> for SeqAccess<'a, 'b> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'b>,
    {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(DeserializeSeed::deserialize(
                seed,
                &mut *self.deserializer,
            )?))
        } else {
            Ok(None)
        }
    }
}

impl<'de, 'a> EnumAccess<'de> for &'a mut Deserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: DeserializeSeed<'de>,
    {
        // This is a bit hacky, but we assume that the out value is in the first
        // variant. As this deserializer is for internal use only this should
        // not be a problem as all the autogenerated types should follow this.
        let v = DeserializeSeed::deserialize(seed, 0u32.into_deserializer())?;
        Ok((v, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        Err(Error::UnsupportedType("newtype variant".to_string()))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType("newtype variant".to_string()))
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
