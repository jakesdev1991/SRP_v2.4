use crate::lib::{Result, SrpContext, State};

pub fn oob_deliver_liveos(_context: &mut SrpContext) -> Result<State> {
    println!("[S1.1] OOB_Deliver_LiveOS: Fetching via BMC...");
    Ok(State::S2_0_PCP_Verify)
}

pub fn pxe_deliver_liveos(context: &mut SrpContext) -> Result<State> {
    println!("[S1.2] PXE_Deliver_LiveOS: Fetching via Network...");
    if !context.peripherals_purged {
        return Err("PXE_Attempt_Before_Purge");
    }
    Ok(State::S2_0_PCP_Verify)
}
