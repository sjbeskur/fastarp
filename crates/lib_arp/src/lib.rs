mod arpscan;
mod arpnode;

#[cfg(test)]
mod tests;

pub use arpscan::*;
pub use arpnode::ArpNode;

pub type ArpResult<T> = std::result::Result<T, ArpErrors>;

#[derive(Debug)]
pub enum ArpErrors {
    ArpError(String),
}

impl std::fmt::Display for ArpErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArpErrors::ArpError(msg) => write!(f, "ArpError: {}", msg),
        }
    }
}

impl std::error::Error for ArpErrors {}
