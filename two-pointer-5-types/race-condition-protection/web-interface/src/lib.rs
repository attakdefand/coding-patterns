//! Two-Pointer Web Interface
//!
//! A web-based interface to explore and test two-pointer coding patterns.

pub mod web_server;

// Re-export main functions
pub use web_server::start_server;