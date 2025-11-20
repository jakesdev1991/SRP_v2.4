use core::arch::naked_asm;

/// S0.1: DRTM Launch
/// Hardware entry point.
#[no_mangle]
#[unsafe(naked)]
pub extern "C" fn drtm_launch() -> ! {
    // Fix: Removed redundant 'unsafe' block. 
    // The function is marked #[unsafe(naked)], so naked_asm! is allowed directly.
    naked_asm!("j srp_main");
}
