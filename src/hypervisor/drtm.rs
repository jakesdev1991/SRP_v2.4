// DRTM Stub
#[no_mangle]
#[naked]
pub extern "C" fn drtm_launch() -> ! {
    unsafe {
        core::arch::asm!("j main", options(noreturn));
    }
}
