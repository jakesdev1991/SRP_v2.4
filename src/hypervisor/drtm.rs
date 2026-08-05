use core::arch::naked_asm;

/// S0.1: DRTM Launch
/// Hardware entry point.
#[no_mangle]
#[unsafe(naked)]
pub extern "C" fn drtm_launch() -> ! {
    // Fix: Architecture-specific assembly.
    // The CI runner is x86_64 (requires 'jmp'), but the hardware might be RISC-V (requires 'j').

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    naked_asm!("jmp srp_main");

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    naked_asm!("j srp_main");

    // Fallback for ARM (e.g. aarch64) if needed in future
    #[cfg(target_arch = "aarch64")]
    naked_asm!("b srp_main");
}
