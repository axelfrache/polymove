pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub mod mi8_proto {
    tonic::include_proto!("mi8");
}
