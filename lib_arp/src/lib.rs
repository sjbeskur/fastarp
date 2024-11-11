// extern crate pnet;
// extern crate ipnetwork;
// extern crate chrono;
// extern crate failure;
// extern crate serde_derive;
// extern crate log;

mod arpscan;
mod arpnode;

pub use arpscan::*;
pub use arpnode::ArpNode;

pub type ArpResult<T> = std::result::Result<T, ArpErrors>;

#[derive(Debug, /*Fail*/)]
pub enum ArpErrors{

    //#[fail(display="ArpError: {}",_0)]
    ArpError(String),

}

// use std::convert::From;
// use std::option::NoneError;

// impl From<NoneError> for ArpErrors{
//     fn from(e:NoneError) -> Self{

//     }
// }

//std::option::NoneError