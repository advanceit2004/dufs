//! dufs as a library.
//!
//! The binary in `main.rs` is a thin CLI wrapper; embedding applications
//! (e.g. a desktop shell) can depend on this crate and drive
//! [`server::Server`] directly.

#[macro_use]
extern crate log;

pub mod args;

pub use args::Args;
pub mod auth;
pub mod http_logger;
pub mod http_utils;
pub mod logger;
pub mod noscript;
pub mod server;
pub mod utils;
