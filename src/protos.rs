mod inner {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use inner::*;
