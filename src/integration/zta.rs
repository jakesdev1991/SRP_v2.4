use crate::lib::{Result, State, SrpContext};
pub fn zta_publish_score(_context: &mut SrpContext) -> Result<State> {
    println!("[S8.0] ZTA_Publish...");
    Ok(State::S9_0_Reboot)
}
