#![crate_name = "static"]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core, io, path)]

//! Static file-serving handler.

extern crate time;

extern crate hyper;
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;

pub use cache_handler::StaticWithCache;
pub use static_handler::Static;

mod cache_handler;
mod requested_path;
mod static_handler;
