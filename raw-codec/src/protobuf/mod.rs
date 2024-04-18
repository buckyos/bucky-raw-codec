pub mod protobuf_helper;
pub(crate) mod protos {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub use protobuf_helper::*;
pub use protos::EmptyContent;
