mod arpscan;
mod arpnode;

#[cfg(test)]
mod tests;

pub use arpscan::*;
pub use arpnode::ArpNode;

use std::collections::HashMap;

/// Result of a scan, including discovered nodes and metadata.
pub struct ScanResult {
    pub nodes: HashMap<String, ArpNode>,
    pub total_ips: usize,
    pub subnet: String,
}

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
