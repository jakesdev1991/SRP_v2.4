use crate::lib::{Result, State, SrpContext};

pub fn triage_run(context: &mut SrpContext) -> Result<State> {
    println!("[S3.0] Triage: Running forensic tools...");
    // Verify PCP was checked before running tools
    if !context.pcp_verified {
        return Err("Triage_Attempt_With_Unverified_PCP");
    }
    Ok(State::S3_1_Microcode_Verify)
}
