use crate::lib::{Result, State, SrpContext, Policy};

pub fn policy_fetch(context: &mut SrpContext) -> Result<State> {
    println!("[S5.0] Policy_Fetch...");
    context.policy = Policy::FullFlash; // Simulating full flash policy
    Ok(State::S5_1_Flash_Verify)
}

pub fn flash_verify_integrity(_context: &mut SrpContext) -> Result<State> {
    println!("[S5.1] Flash_Verify...");
    Ok(State::S5_2_Firmware_Match)
}

pub fn firmware_match_pcp(context: &mut SrpContext) -> Result<State> {
    println!("[S5.2] Firmware_Match...");
    context.firmware_matches = false; // Simulate mismatch to force flash
    if context.firmware_matches {
        Ok(State::S5_3_Cleanup_Evaluate)
    } else {
        Ok(State::S6_0_Firmware_Flash)
    }
}

pub fn cleanup_evaluate(context: &mut SrpContext) -> Result<State> {
    println!("[S5.3] Cleanup_Evaluate...");
    match context.policy {
        Policy::FullFlash => Ok(State::S6_0_Firmware_Flash),
        Policy::CleanupOnly => Ok(State::S5_4_Malware_Cleanup),
        Policy::DryRun => Ok(State::S7_0_Commit_Attest),
    }
}

pub fn malware_cleanup(_context: &mut SrpContext) -> Result<State> {
    println!("[S5.4] Malware_Cleanup...");
    Ok(State::S7_0_Commit_Attest)
}

pub fn firmware_flash(_context: &mut SrpContext) -> Result<State> {
    println!("[S6.0] FIRMWARE_FLASH: *** ERADICATING AND REFLASHING SPI ***");
    Ok(State::S7_0_Commit_Attest)
}
