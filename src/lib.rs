// 1. Load the common definitions
pub mod common;

// 2. Create the 'lib' alias.
// This makes 'crate::lib::Result' resolve to 'crate::common::Result'.
pub use common as lib;

// 3. Register the rest of the modules
pub mod attestation;
pub mod components;
pub mod core;
pub mod crypto;
pub mod hypervisor;
pub mod integration;
