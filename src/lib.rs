// Main Library Entry Point
// We re-export 'common' as 'lib' so your code's "use crate::lib::*" imports work
pub mod common;
pub use common as lib;

// Register all the other modules so Rust compiles them
pub mod attestation;
pub mod components;
pub mod core;
pub mod crypto;
pub mod hypervisor;
pub mod integration;
