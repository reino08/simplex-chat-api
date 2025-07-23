#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
#![cfg_attr(target_arch = "wasm32", allow(clippy::future_not_send))]

pub mod command;
pub mod stream;
pub mod types;

mod client;
pub use client::{Client, ClientBuilder};
