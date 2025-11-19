use core::arch::naked_asm;

/// S0.1: DRTM Launch
/// Hardware entry point.
#[no_mangle]
#[unsafe(naked)]
pub extern "C" fn drtm_launch() -> ! {
    unsafe {
        naked_asm!("j srp_main");
    }
}
