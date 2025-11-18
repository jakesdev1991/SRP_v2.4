use crate::lib::{Result, State, SrpContext};
pub fn fido_quorum_verify(context: &mut SrpContext) -> Result<State> {
    println!("[S1.0] FIDO_Quorum...");
    context.fido_quorum_ok = true;
    // Simulate decision to use PXE
    Ok(State::S1_2_PXE_Deliver_LiveOS)
}
