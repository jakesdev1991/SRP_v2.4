use crate::lib::{Result, SrpContext, State};

pub fn liveos_attest(_context: &mut SrpContext) -> Result<State> {
    println!("[S4.0] LiveOS_Attest: Generating initial EAT token...");
    Ok(State::S5_0_Policy_Fetch)
}

pub fn commit_attest(_context: &mut SrpContext) -> Result<State> {
    println!("[S7.0] Commit_Attest: Generating final EAT token...");
    Ok(State::S8_0_ZTA_Publish)
}
