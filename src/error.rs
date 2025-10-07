//! Error types

use std::{
	io,
	fmt::Display
};
use serde::{
	ser,
	de
};
use thiserror::Error;

/// The main error type, represents either an encoding or decoding error
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
	Serialization(String),
	#[error("Input/Output error")]
	IO(#[from] io::Error)
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