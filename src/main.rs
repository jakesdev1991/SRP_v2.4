mod lib;
mod core {
    pub mod state_machine;
    pub mod states {
        pub mod s0_init;
        pub mod s1_delivery;
        pub mod s3_analysis;
        pub mod s5_remediation;
    }
}
mod attestation { pub mod eat; }
mod components {
    pub mod live_os;
    pub mod pcp;
    pub mod triage;
}
mod crypto { pub mod fido; }
mod hypervisor {
    pub mod drtm;
    pub mod sel4;
}
mod integration {
    pub mod bmc;
    pub mod zta;
}

use core::state_machine;

fn main() {
    println!("*** Sovereign Recovery Protocol v2.4 Entry ***");
    // S0.2_TCB_Measure
    if let Err(e) = hypervisor::sel4::tcb_measure_sel4() {
        println!("FATAL: TCB Measurement Failed: {:?}", e);
        state_machine::halt_error_state(e);
    }
    println!("TCB Measured. Handing off to state machine.");
    state_machine::run();
    println!("FATAL: State machine exited unexpectedly.");
    state_machine::halt_error_state("StateMachineExited");
}
