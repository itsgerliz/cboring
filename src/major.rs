//! Constants for major types, high-order 3 bits of the initial byte in each data item

/// Unsigned integer in the range 0..2^64-1 inclusive
pub const UNSIGNED_INTEGER: u8 = 0;

/// Negative integer in the range -2^64..-1 inclusive
pub const NEGATIVE_INTEGER: u8 = 1;

/// Byte string
pub const BYTE_STRING: u8 = 2;

/// Text string, encoded as UTF-8
/// # Considerations
/// - An invalid UTF-8 sequence makes this CBOR data item invalid
pub const TEXT_STRING: u8 = 3;

/// Array of data items
pub const ARRAY_OF_DATA_ITEMS: u8 = 4;

/// Map of pairs of data items
/// # Considerations
/// - A map must have an even number of pairs
/// - A map with an odd number of pairs is invalid
pub const MAP_OF_PAIRS_OF_DATA_ITEMS: u8 = 5;

/// Tagged data item, whose tag number is an integer in the range 0..2^64-1 inclusive
pub const TAGGED_DATA_ITEM: u8 = 6;

/// Floating-point numbers, simple values and the _break_ stop code
pub const FLOATING_SIMPLE_AND_BREAK: u8 = 7;
