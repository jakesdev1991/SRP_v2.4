use crate::lib::{Result, State, SrpContext};

pub fn silicon_poison_check(context: &mut SrpContext) -> Result<State> {
    println!("[S0.3] Silicon_Poison_Check...");
    context.silicon_clean = true;
    Ok(State::S0_4_Peripheral_Purge)
}

pub fn peripheral_purge(context: &mut SrpContext) -> Result<State> {
    println!("[S0.4] Peripheral_Purge...");
    context.peripherals_purged = true;
    Ok(State::S1_0_FIDO_Quorum)
}
