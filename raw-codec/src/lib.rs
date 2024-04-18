#![allow(unused)]

mod protobuf;
mod raw;
mod error;

#[cfg(feature = "derive")]
pub use bucky_raw_codec_derive::*;

pub use protobuf::*;
pub use raw::*;
pub use error::*;

#[macro_use]
extern crate log;

// ObjectContent的编码类型，默认为Raw
pub const OBJECT_CONTENT_CODEC_FORMAT_RAW: u8 = 0;
pub const OBJECT_CONTENT_CODEC_FORMAT_PROTOBUF: u8 = 1;
pub const OBJECT_CONTENT_CODEC_FORMAT_JSON: u8 = 2;
