use crate::lib::{State, SrpContext}; // Removed Policy, Result
use super::states::*;

/// Runs the main state machine loop.
/// This function only returns on S9.0 (Reboot) or S10.0 (Halt).
pub fn run() {
    let mut context = SrpContext::new();
    println!("[Core] State Machine Initialized. Starting loop...");

    loop {
        let next_state_result = match context.current_state {
            // S0: TCB Launch
            State::S0_2_TCB_Measure => Ok(State::S0_3_Silicon_Poison_Check),
            State::S0_3_Silicon_Poison_Check => s0_init::silicon_poison_check(&mut context),
            State::S0_4_Peripheral_Purge => s0_init::peripheral_purge(&mut context),

            // S1: Delivery
            State::S1_0_FIDO_Quorum => crate::crypto::fido::fido_quorum_verify(&mut context),
            State::S1_1_OOB_Deliver_LiveOS => s1_delivery::oob_deliver_liveos(&mut context),
            State::S1_2_PXE_Deliver_LiveOS => s1_delivery::pxe_deliver_liveos(&mut context),

            // S2: Payload Verification
            State::S2_0_PCP_Verify => crate::components::pcp::pcp_verify(&mut context),

            // S3: Analysis
            State::S3_0_Triage => crate::components::triage::triage_run(&mut context),
            State::S3_1_Microcode_Verify => s3_analysis::microcode_verify_and_patch(&mut context),

            // S4: Attestation
            State::S4_0_LiveOS_Attest => crate::attestation::eat::liveos_attest(&mut context),

            // S5: Remediation
            State::S5_0_Policy_Fetch => s5_remediation::policy_fetch(&mut context),
            State::S5_1_Flash_Verify => s5_remediation::flash_verify_integrity(&mut context),
            State::S5_2_Firmware_Match => s5_remediation::firmware_match_pcp(&mut context),
            State::S5_3_Cleanup_Evaluate => s5_remediation::cleanup_evaluate(&mut context),
            State::S5_4_Malware_Cleanup => s5_remediation::malware_cleanup(&mut context),

            // S6: Flash
            State::S6_0_Firmware_Flash => s5_remediation::firmware_flash(&mut context),

            // S7: Commit
            State::S7_0_Commit_Attest => crate::attestation::eat::commit_attest(&mut context),

            // S8: Publish
            State::S8_0_ZTA_Publish => crate::integration::zta::zta_publish_score(&mut context),

            // S9: Reboot (Terminal)
            State::S9_0_Reboot => {
                println!("[S9.0] Rebooting to clean state.");
                crate::hypervisor::sel4::reboot_clean();
                // Fix: Return a Result type to satisfy the match arm requirements.
                // Since reboot_clean returns (), we force an Err to break flow or satisfy types.
                Err("Reboot initiated") 
            },

            // S10: Halt (Terminal)
            State::S10_0_Halt_Error => {
                println!("[S10.0] FATAL ERROR. System Halted.");
                // halt_error_state returns ! (diverging), which effectively coerces to any type, including Result.
                halt_error_state("Entered Error State");
            },

            // Catch-all for safety
            _ => Err("Invalid or Unimplemented State Transition"),
        };

        match next_state_result {
            Ok(next_state) => {
                println!("TRANSITION: {:?} -> {:?}", context.current_state, next_state);
                context.current_state = next_state;
            },
            Err(e) => {
                println!("FATAL ERROR in state {:?}: {}", context.current_state, e);
                halt_error_state(e);
            }
        }
    }
}

/// Halts the system in a safe error state.
pub fn halt_error_state(error: &str) -> ! {
    println!("[S10.0] HALT: {}", error);
    loop {
        core::hint::spin_loop();
    }
}
