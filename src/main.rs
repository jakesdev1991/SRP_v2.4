// src/main.rs
// Reference Implementation Entry Point for SRP v2.4

// Note: We use the library crate name defined in Cargo.toml
// If your Cargo.toml name is different, change 'srp_core_v2_4' to match it.
use srp_core_v2_4::core::state_machine;
use srp_core_v2_4::hypervisor::sel4;

/// Standard entry point for CI/Linux testing environments.
fn main() {
    println!("*** SRP v2.4 Reference Implementation Starting (Simulation Mode) ***");

    // 1. Simulate S0.2 TCB Measurement
    println!("[Init] Simulating TCB Measurement...");
    if let Err(e) = sel4::tcb_measure_sel4() {
        eprintln!("FATAL: TCB Measurement failed: {}", e);
        std::process::exit(1);
    }

    // 2. Hand off to the Core State Machine
    println!("[Init] Handing off to State Machine...");
    state_machine::run();
}

/// Bare-metal Entry Point (Required for the spec/DRTM)
#[no_mangle]
pub extern "C" fn srp_main() -> ! {
    state_machine::run();
    loop { core::hint::spin_loop(); }
}
