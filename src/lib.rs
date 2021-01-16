#[macro_use] extern crate log;
#[macro_use] extern crate hyper;
extern crate url;
extern crate rustc_serialize;

pub mod simple_log;
pub mod server;
pub mod escape_json;

pub use server::start;
pub use server::handle;
