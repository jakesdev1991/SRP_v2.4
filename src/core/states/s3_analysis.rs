use crate::lib::{Result, State, SrpContext};

pub fn microcode_verify_and_patch(context: &mut SrpContext) -> Result<State> {
    println!("[S3.1] Microcode_Verify: Checking CPU microcode...");
    if !context.silicon_clean {
        return Err("Microcode_Patch_Attempt_On_Poisoned_Silicon");
    }
    Ok(State::S4_0_LiveOS_Attest)
}
