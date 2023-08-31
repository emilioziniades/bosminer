#![doc = include_str!("../README.md")]

/// TODO: documentation
tonic::include_proto!("braiins.bos");

/// TODO: documentation
pub mod v1 {
    tonic::include_proto!("braiins.bos.v1");
}
