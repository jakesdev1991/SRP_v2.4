use crate::lib::{Result, Policy};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    S0_1_DRTM_Launch,
    S0_2_TCB_Measure,
    S0_3_Silicon_Poison_Check,
    S0_4_Peripheral_Purge,
    S1_0_FIDO_Quorum,
    S1_1_OOB_Deliver_LiveOS,
    S1_2_PXE_Deliver_LiveOS,
    S2_0_PCP_Verify,
    S3_0_Triage,
    S3_1_Microcode_Verify,
    S4_0_LiveOS_Attest,
    S5_0_Policy_Fetch,
    S5_1_Flash_Verify,
    S5_2_Firmware_Match,
    S5_3_Cleanup_Evaluate,
    S5_4_Malware_Cleanup,
    S6_0_Firmware_Flash,
    S7_0_Commit_Attest,
    S8_0_ZTA_Publish,
    S9_0_Reboot,
    S10_0_Halt_Error,
}

#[derive(Debug, Clone)]
pub struct SrpContext {
    pub current_state: State,
    pub tcb_measured: bool,
    pub silicon_clean: bool,
    pub peripherals_purged: bool,
    pub fido_quorum_ok: bool,
    pub pcp_verified: bool,
    pub firmware_matches: bool,
    pub policy: Policy,
}

impl SrpContext {
    pub fn new() -> Self {
        SrpContext {
            current_state: State::S0_2_TCB_Measure,
            tcb_measured: true,
            silicon_clean: false,
            peripherals_purged: false,
            fido_quorum_ok: false,
            pcp_verified: false,
            firmware_matches: false,
            policy: Policy::DryRun,
        }
    }
}
