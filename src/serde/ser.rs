use crate::error::EncodeError;
use serde::ser::{Impossible, Serialize, Serializer};
use std::io::{BufWriter, Write};

pub struct Encoder<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Encoder<W> {
    pub fn into_writer(destination: W) -> Self {
        Self {
            writer: BufWriter::new(destination),
        }
    }

    pub fn flush(&mut self) -> Result<(), EncodeError> {
        Ok(self.writer.flush()?)
    }
}

impl<W: Write> Serializer for &mut Encoder<W> {
    type Ok = ();
    type Error = EncodeError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let byte: u8 = if v { 0xF5 } else { 0xF4 };
        Ok(self.writer.write_all(&[byte])?)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        if v < 0 { // Signed (-128..=-1)
            let encoded_value: u8 = (-1 - v) as u8;
            if encoded_value < 24 {
                Ok(self.writer.write_all(&[(0b001_00000 | encoded_value)])?)
            } else {
                Ok(self.writer.write_all(&[0x38, encoded_value])?)
            }
        } else { // Unsigned (0..=127)
            self.serialize_u8(v as u8)
        }
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        if v < 0 { // Signed (-32_768..=-1)
            let encoded_value: u16 = (-1 - v) as u16;
            if encoded_value < 24 {
                Ok(self.writer.write_all(&[(0b001_00000 | (encoded_value as u8))])?)
            } else {
                let encoded_value_bigend: [u8; 2] = encoded_value.to_be_bytes();
                Ok(self.writer.write_all(&[0x39, encoded_value_bigend[0], encoded_value_bigend[1]])?)
            }
        } else { // Unsigned (0..=32_767)
            self.serialize_u16(v as u16)
        }
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        if v < 0 { // Signed (-2_147_483_648..=-1)
            let encoded_value: u32 = (-1 - v) as u32;
            if encoded_value < 24 {
                Ok(self.writer.write_all(&[(0b001_00000 | (encoded_value as u8))])?)
            } else {
                let encoded_value_bigend: [u8; 4] = encoded_value.to_be_bytes();
                Ok(self.writer.write_all(&[0x3A, encoded_value_bigend[0], encoded_value_bigend[1],
                encoded_value_bigend[2], encoded_value_bigend[3]])?)
            }
        } else { // Unsigned (0..=2_147_483_647)
            self.serialize_u32(v as u32)
        }
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if v < 0 { // Signed (-9_223_372_036_854_775_808..=-1)
            let encoded_value: u64 = (-1 - v) as u64;
            if encoded_value < 24 {
                Ok(self.writer.write_all(&[(0b001_00000 | (encoded_value as u8))])?)
            } else {
                let encoded_value_bigend: [u8; 8] = encoded_value.to_be_bytes();
                Ok(self.writer.write_all(&[0x3B, encoded_value_bigend[0], encoded_value_bigend[1],
                encoded_value_bigend[2], encoded_value_bigend[3], encoded_value_bigend[4],
                encoded_value_bigend[5], encoded_value_bigend[6], encoded_value_bigend[7]])?)
            }
        } else { // Unsigned (0..=9_223_372_036_854_775_807)
            self.serialize_u64(v as u64)
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if v < 24 {
            Ok(self.writer.write_all(&[(0b000_00000 | v)])?)
        } else {
            Ok(self.writer.write_all(&[0x18, v])?)
        }
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        if v < 24 {
            Ok(self.writer.write_all(&[(0b000_00000 | (v as u8))])?)
        } else {
            let encoded_value_bigend: [u8; 2] = v.to_be_bytes();
            Ok(self.writer.write_all(&[0x19, encoded_value_bigend[0], encoded_value_bigend[1]])?)
        }
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if v < 24 {
            Ok(self.writer.write_all(&[(0b000_00000 | (v as u8))])?)
        } else {
            let encoded_value_bigend: [u8; 4] = v.to_be_bytes();
            Ok(self.writer.write_all(&[0x1A, encoded_value_bigend[0], encoded_value_bigend[1],
            encoded_value_bigend[2], encoded_value_bigend[3]])?)
        }
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v < 24 {
            Ok(self.writer.write_all(&[(0b000_00000 | (v as u8))])?)
        } else {
            let encoded_value_bigend: [u8; 8] = v.to_be_bytes();
            Ok(self.writer.write_all(&[0x1B, encoded_value_bigend[0], encoded_value_bigend[1],
            encoded_value_bigend[2], encoded_value_bigend[3], encoded_value_bigend[4],
            encoded_value_bigend[5], encoded_value_bigend[6], encoded_value_bigend[7]])?)
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let byte: u8 = 0b011_00000 | (v.len_utf8() as u8);
        let mut buf: [u8; 4] = [0; 4];
        let utf8_bytes = v.encode_utf8(&mut buf).as_bytes();
        self.writer.write_all(&[byte])?;
        self.writer.write_all(utf8_bytes)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
