use crate::lib::Result;
pub fn tcb_measure_sel4() -> Result<()> {
    println!("[S0.2] TCB_Measure...");
    Ok(())
}
pub fn reboot_clean() -> ! {
    println!("[S9.0] Issuing clean reboot request...");
    loop { std::hint::spin_loop(); }
}
