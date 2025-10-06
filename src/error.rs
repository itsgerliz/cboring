use std::fmt::Display;
use serde::{de, ser};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Error when encoding a CBOR sequence")]
	Encode(#[from] EncodeError),
	#[error("Error when decoding a CBOR sequence")]
	Decode(#[from] DecodeError)
}

/// Represents possible errors when encoding a CBOR data item
#[derive(Error, Debug)]
pub enum EncodeError {
	#[error("Error when serializing")]
	Serialization(String)
}

/// Represents possible errors when decoding a CBOR data item
#[derive(Error, Debug)]
pub enum DecodeError {
	#[error("Error when deserializing")]
	Deserialization(String)
}

impl ser::Error for EncodeError {
	fn custom<T: Display>(msg: T) -> Self {
		Self::Serialization(msg.to_string())
	}
}

impl de::Error for DecodeError {
	fn custom<T: Display>(msg: T) -> Self {
		Self::Deserialization(msg.to_string())
	}
}