use crate::lib::{Result, State, SrpContext};

pub fn liveos_attest(_context: &mut SrpContext) -> Result<State> {
    println!("[S4.0] LiveOS_Attest...");
    Ok(State::S5_0_Policy_Fetch)
}

pub fn commit_attest(_context: &mut SrpContext) -> Result<State> {
    println!("[S7.0] Commit_Attest...");
    Ok(State::S8_0_ZTA_Publish)
}
