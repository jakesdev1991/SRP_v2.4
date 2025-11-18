use crate::lib::{Result, State, SrpContext, Policy};

pub fn policy_fetch(context: &mut SrpContext) -> Result<State> {
    println!("[S5.0] Policy_Fetch: Fetching remediation policy...");
    // Simulation: Set policy to FullFlash for testing
    context.policy = Policy::FullFlash;
    Ok(State::S5_1_Flash_Verify)
}

pub fn flash_verify_integrity(_context: &mut SrpContext) -> Result<State> {
    println!("[S5.1] Flash_Verify: Checking SPI flash integrity...");
    Ok(State::S5_2_Firmware_Match)
}

pub fn firmware_match_pcp(context: &mut SrpContext) -> Result<State> {
    println!("[S5.2] Firmware_Match: Comparing Flash to PCP...");
    // Simulation: Mismatch found, requiring flash
    context.firmware_matches = false;
    if context.firmware_matches {
        Ok(State::S5_3_Cleanup_Evaluate)
    } else {
        Ok(State::S6_0_Firmware_Flash)
    }
}

pub fn cleanup_evaluate(context: &mut SrpContext) -> Result<State> {
    println!("[S5.3] Cleanup_Evaluate: Deciding cleanup path...");
    match context.policy {
        Policy::FullFlash => Ok(State::S6_0_Firmware_Flash),
        Policy::CleanupOnly => Ok(State::S5_4_Malware_Cleanup),
        Policy::DryRun => Ok(State::S7_0_Commit_Attest),
    }
}

pub fn malware_cleanup(_context: &mut SrpContext) -> Result<State> {
    println!("[S5.4] Malware_Cleanup: Running AV tools...");
    Ok(State::S7_0_Commit_Attest)
}

pub fn firmware_flash(_context: &mut SrpContext) -> Result<State> {
    println!("[S6.0] FIRMWARE_FLASH: Writing new firmware...");
    Ok(State::S7_0_Commit_Attest)
}
