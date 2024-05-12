mod application;
pub mod cnf;
pub mod container;
mod domain;
mod error;
pub mod infra;
pub mod presentation;

pub mod ordergrpc {
    tonic::include_proto!("app");
}
