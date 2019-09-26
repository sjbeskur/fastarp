extern crate env_logger;
extern crate pnet;
extern crate ipnetwork;
extern crate chrono;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod arpscan;
mod arpnode;

pub use arpscan::*;
pub use arpnode::ArpNode;

pub type ArpResult<T> = std::result::Result<T, ArpErrors>;

#[derive(Debug, Fail)]
pub enum ArpErrors{

    #[fail(display="ArpError: {}",_0)]
    ArpError(String),

}