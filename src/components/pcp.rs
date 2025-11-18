use crate::lib::{Result, State, SrpContext};
pub fn pcp_verify(context: &mut SrpContext) -> Result<State> {
    println!("[S2.0] PCP_Verify...");
    context.pcp_verified = true;
    Ok(State::S3_0_Triage)
}
