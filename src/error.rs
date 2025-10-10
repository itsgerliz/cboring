use std::{
	io,
	fmt::Display
};
use serde::{
	ser,
	de
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Error when encoding a CBOR sequence")]
	Encode(#[from] EncodeError),
	#[error("Error when decoding a CBOR sequence")]
	Decode(#[from] DecodeError)
}

#[derive(Error, Debug)]
pub enum EncodeError {
	#[error("Error when serializing")]
	Serialization(String),
	#[error("Input/Output error")]
	IO(#[from] io::Error)
}

#[derive(Error, Debug)]
pub enum DecodeError {
	#[error("Error when deserializing")]
	Deserialization(String),
	#[error("Input/Output error")]
	IO(#[from] io::Error),
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
