use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    UnsupportedType(String),

    DoesNotCorespondToVariant(String),

    EndOfMessage,

    UnExpectedValue(String),

    TrailingBytes,

    WontImplement,

    WrongLength { expected: usize, got: usize },

    Crc { recived: u8, calculated: u8 },
    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    // Eof,
    // Syntax,
    // ExpectedBoolean,
    // ExpectedInteger,
    // ExpectedString,
    // ExpectedNull,
    // ExpectedArray,
    // ExpectedArrayComma,
    // ExpectedArrayEnd,
    // ExpectedMap,
    // ExpectedMapColon,
    // ExpectedMapComma,
    // ExpectedMapEnd,
    // ExpectedEnum,
    // TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::UnsupportedType(typ) => formatter.write_str(&format!(
                "Type: {} not suported by this serde implementation",
                typ
            )),
            Error::DoesNotCorespondToVariant(msg) => formatter.write_str(msg),
            Error::EndOfMessage => formatter.write_str("Unexpected end of message"),
            Error::UnExpectedValue(msg) => formatter.write_str(msg),
            Error::TrailingBytes => formatter.write_str("Message has trailing bytes"),
            Error::WontImplement => {
                formatter.write_str("This does not make sense to implement for this format")
            }
            Error::WrongLength { expected, got } => formatter.write_str(&format!(
                "Lenght is expected to be {expected} but is {got}."
            )),
            Error::Crc {
                recived,
                calculated,
            } => formatter.write_str(&format!(
                "CRC does not match recived: {recived:x}, calculated: {calculated:x}"
            )),
        }
    }
}

impl std::error::Error for Error {}
