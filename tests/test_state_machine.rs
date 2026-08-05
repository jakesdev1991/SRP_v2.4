use srp_core_v2_4::common::{Policy, SrpContext, State};

#[test]
fn srp_context_default_matches_new() {
    let new = SrpContext::new();
    let default = SrpContext::default();
    assert_eq!(new.current_state, default.current_state);
    assert_eq!(new.tcb_measured, default.tcb_measured);
    assert_eq!(new.silicon_clean, default.silicon_clean);
    assert_eq!(new.peripherals_purged, default.peripherals_purged);
    assert_eq!(new.fido_quorum_ok, default.fido_quorum_ok);
    assert_eq!(new.pcp_verified, default.pcp_verified);
    assert_eq!(new.firmware_matches, default.firmware_matches);
    assert!(matches!(new.policy, Policy::DryRun));
}

#[test]
fn state_transitions_s0_to_s1() {
    let mut ctx = SrpContext::new();
    assert_eq!(ctx.current_state, State::S0_2_TCB_Measure);

    // S0.2 -> S0.3
    // S0.3: silicon_poison_check
    let next_state = srp_core_v2_4::core::states::s0_init::silicon_poison_check(&mut ctx);
    assert!(next_state.is_ok());
    assert_eq!(next_state.unwrap(), State::S0_4_Peripheral_Purge);
    assert!(ctx.silicon_clean);

    // S0.4: peripheral_purge
    let next_state = srp_core_v2_4::core::states::s0_init::peripheral_purge(&mut ctx);
    assert!(next_state.is_ok());
    assert_eq!(next_state.unwrap(), State::S1_0_FIDO_Quorum);
    assert!(ctx.peripherals_purged);
}

#[test]
fn pxe_delivery_requires_purged_peripherals() {
    let mut ctx = SrpContext::new();
    // Not purged yet
    ctx.peripherals_purged = false;
    let result = srp_core_v2_4::core::states::s1_delivery::pxe_deliver_liveos(&mut ctx);
    assert!(result.is_err());

    // Now purge
    ctx.peripherals_purged = true;
    let result = srp_core_v2_4::core::states::s1_delivery::pxe_deliver_liveos(&mut ctx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), State::S2_0_PCP_Verify);
}

#[test]
fn triage_requires_verified_pcp() {
    let mut ctx = SrpContext::new();
    ctx.pcp_verified = false;
    let result = srp_core_v2_4::components::triage::triage_run(&mut ctx);
    assert!(result.is_err());

    ctx.pcp_verified = true;
    let result = srp_core_v2_4::components::triage::triage_run(&mut ctx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), State::S3_1_Microcode_Verify);
}

#[test]
fn microcode_verify_rejects_poisoned_silicon() {
    let mut ctx = SrpContext::new();
    ctx.silicon_clean = false;
    let result = srp_core_v2_4::core::states::s3_analysis::microcode_verify_and_patch(&mut ctx);
    assert!(result.is_err());

    ctx.silicon_clean = true;
    let result = srp_core_v2_4::core::states::s3_analysis::microcode_verify_and_patch(&mut ctx);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), State::S4_0_LiveOS_Attest);
}

#[test]
fn cleanup_evaluate_respects_policy() {
    let mut ctx = SrpContext::new();

    // DryRun -> skip to commit
    ctx.policy = Policy::DryRun;
    let result = srp_core_v2_4::core::states::s5_remediation::cleanup_evaluate(&mut ctx);
    assert_eq!(result.unwrap(), State::S7_0_Commit_Attest);

    // CleanupOnly -> malware cleanup
    ctx.policy = Policy::CleanupOnly;
    let result = srp_core_v2_4::core::states::s5_remediation::cleanup_evaluate(&mut ctx);
    assert_eq!(result.unwrap(), State::S5_4_Malware_Cleanup);

    // FullFlash -> firmware flash
    ctx.policy = Policy::FullFlash;
    let result = srp_core_v2_4::core::states::s5_remediation::cleanup_evaluate(&mut ctx);
    assert_eq!(result.unwrap(), State::S6_0_Firmware_Flash);
}

#[test]
fn firmware_match_routes_correctly() {
    let mut ctx = SrpContext::new();

    // Mismatch -> flash
    ctx.firmware_matches = false;
    let result = srp_core_v2_4::core::states::s5_remediation::firmware_match_pcp(&mut ctx);
    assert_eq!(result.unwrap(), State::S6_0_Firmware_Flash);

    // We can't test the match=true path because the function hardcodes
    // firmware_matches=false, but we test the actual behavior which is flash.
}

#[test]
fn fido_quorum_sets_flag_and_routes_to_pxe() {
    let mut ctx = SrpContext::new();
    assert!(!ctx.fido_quorum_ok);
    let result = srp_core_v2_4::crypto::fido::fido_quorum_verify(&mut ctx);
    assert!(ctx.fido_quorum_ok);
    assert_eq!(result.unwrap(), State::S1_2_PXE_Deliver_LiveOS);
}

#[test]
fn pcp_verify_sets_flag_and_routes_to_triage() {
    let mut ctx = SrpContext::new();
    assert!(!ctx.pcp_verified);
    let result = srp_core_v2_4::components::pcp::pcp_verify(&mut ctx);
    assert!(ctx.pcp_verified);
    assert_eq!(result.unwrap(), State::S3_0_Triage);
}

#[test]
fn zta_publish_routes_to_reboot() {
    let mut ctx = SrpContext::new();
    let result = srp_core_v2_4::integration::zta::zta_publish_score(&mut ctx);
    assert_eq!(result.unwrap(), State::S9_0_Reboot);
}

#[test]
fn bmc_spdm_client_returns_blob() {
    let client = srp_core_v2_4::integration::bmc::SpdmClient::new();
    let blob = client.get_pcp_blob();
    assert!(blob.is_some());
    let blob = blob.unwrap();
    assert!(!blob.is_empty());
    assert_eq!(blob, vec![0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn spdm_client_default_impl_exists() {
    // Verify that Default is implemented (compiles) without triggering
    // the `default_constructed_unit_structs` clippy lint.
    fn assert_default<T: Default>() {}
    assert_default::<srp_core_v2_4::integration::bmc::SpdmClient>();
}
