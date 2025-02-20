use serde::{ser, Serialize};

use crate::error::{Error, Result};

pub struct Serializer {
    // This string starts empty and bytes appended as values are serialized.
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push(u8::from(v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.push(v.to_le_bytes()[0]);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.extend(v.to_le_bytes());
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::UnsupportedType("char".to_string()))
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(Error::UnsupportedType("str".to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::UnsupportedType("none".to_string()))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType("some".to_string()))
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.push(0x00); // Return a zero byte on unit, this might need to be changed further on
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedType("seq".to_string()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self) // I am a bit unsure of how this works but i hope it does
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType("map".to_string()))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!();
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        unreachable!()
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}
