use std::io::{BufWriter, Write};
use serde::ser::{Serialize, Serializer, Impossible};
use crate::error::EncodeError;

/// The encoder type, encodes Rust types into CBOR data written to a writer
pub struct Encoder<W: Write> {
	writer: BufWriter<W>
}

impl<W: Write> Encoder<W> {
	/// Constructs a new encoder that will write its output to the inner writer _destination_
	/// # Example
	/// ```
	/// use cboring::serde::ser::Encoder;
	/// use std::io;
	/// 
	/// let my_encoder = Encoder::into_writer(io::stdout());
	/// ```
	pub fn into_writer(destination: W) -> Self {
		Self {
			writer: BufWriter::new(destination)
		}
	}

	/// Though dropping the encoder will automatically try to flush its inner buffer it is highly
	/// recommendable to use this method on the encoder before dropping it  
	/// If this method is ommited and the encoder is just dropped and any I/O error occurs when
	/// trying to flush the inner buffer data may be lost and not written to the inner writer
	/// # Example
	/// ```
	/// use cboring::serde::ser::Encoder;
	/// use std::io;
	/// 
	/// let mut my_encoder = Encoder::into_writer(io::stdout());
	/// // We encode our CBOR data...
	/// // Before reaching the end of the scope and dropping the encoder instance we ensure data is
	/// // written to the writer (the standard output) without errors
	/// my_encoder.flush().unwrap()
	/// // Here we assume it won't fail but we should check for a possible IO error in a real case
	/// ```
	pub fn flush(&mut self) -> Result<(), EncodeError> {
		Ok(self.writer.flush()?)
	}
}

impl<W: Write> Serializer for Encoder<W> {
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
		todo!()
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!()
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
		T: ?Sized + Serialize {
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
		T: ?Sized + Serialize {
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
		T: ?Sized + Serialize {
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