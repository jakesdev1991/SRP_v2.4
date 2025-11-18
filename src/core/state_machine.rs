use crate::lib::{Result, State, SrpContext, Policy};
use super::states::*;

pub fn run() {
    let mut context = SrpContext::new();
    loop {
        let next_state_result = match context.current_state {
            State::S0_2_TCB_Measure => Ok(State::S0_3_Silicon_Poison_Check),
            State::S0_3_Silicon_Poison_Check => s0_init::silicon_poison_check(&mut context),
            State::S0_4_Peripheral_Purge => s0_init::peripheral_purge(&mut context),
            State::S1_0_FIDO_Quorum => crate::crypto::fido::fido_quorum_verify(&mut context),
            State::S1_1_OOB_Deliver_LiveOS => s1_delivery::oob_deliver_liveos(&mut context),
            State::S1_2_PXE_Deliver_LiveOS => s1_delivery::pxe_deliver_liveos(&mut context),
            State::S2_0_PCP_Verify => crate::components::pcp::pcp_verify(&mut context),
            State::S3_0_Triage => crate::components::triage::triage_run(&mut context),
            State::S3_1_Microcode_Verify => s3_analysis::microcode_verify_and_patch(&mut context),
            State::S4_0_LiveOS_Attest => crate::attestation::eat::liveos_attest(&mut context),
            State::S5_0_Policy_Fetch => s5_remediation::policy_fetch(&mut context),
            State::S5_1_Flash_Verify => s5_remediation::flash_verify_integrity(&mut context),
            State::S5_2_Firmware_Match => s5_remediation::firmware_match_pcp(&mut context),
            State::S5_3_Cleanup_Evaluate => s5_remediation::cleanup_evaluate(&mut context),
            State::S5_4_Malware_Cleanup => s5_remediation::malware_cleanup(&mut context),
            State::S6_0_Firmware_Flash => s5_remediation::firmware_flash(&mut context),
            State::S7_0_Commit_Attest => crate::attestation::eat::commit_attest(&mut context),
            State::S8_0_ZTA_Publish => crate::integration::zta::zta_publish_score(&mut context),
            State::S9_0_Reboot => {
                println!("[S9.0] Rebooting to clean state.");
                crate::hypervisor::sel4::reboot_clean();
                break;
            },
            State::S10_0_Halt_Error => {
                println!("[S10.0] FATAL ERROR. System Halted.");
                loop { std::hint::spin_loop(); }
            },
            _ => Err("Invalid or unimplemented state"),
        };

        match next_state_result {
            Ok(next_state) => {
                println!("State transition: {:?} -> {:?}", context.current_state, next_state);
                context.current_state = next_state;
            },
            Err(e) => {
                println!("State failed: {:?} with error: {}", context.current_state, e);
                context.current_state = State::S10_0_Halt_Error;
            }
        }
    }
}

pub fn halt_error_state(error: &'static str) -> ! {
    println!("[S10.0] HALT: {}", error);
    loop { std::hint::spin_loop(); }
}
