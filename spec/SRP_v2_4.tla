---- MODULE SRP_v2_4 ----
EXTENDS Naturals, FiniteSets, TLC

(* Based on Appendix C-1: Formal Verification Artifacts *)

(* Define the states of the machine *)
STATES == { "S0.1_DRTM_Launch", "S0.2_TCB_Measure", "S0.3_Silicon_Poison_Check", 
            "S0.4_Peripheral_Purge", "S1.0_FIDO_Quorum", "S1.1_OOB_Deliver_LiveOS", 
            "S1.2_PXE_Deliver_LiveOS", "S2.0_PCP_Verify", "S3.0_Triage", 
            "S3.1_Microcode_Verify", "S4.0_LiveOS_Attest", "S5.0_Policy_Fetch", 
            "S5.1_Flash_Verify", "S5.2_Firmware_Match", "S5.3_Cleanup_Evaluate", 
            "S5.4_Malware_Cleanup", "S6.0_Firmware_Flash", "S7.0_Commit_Attest", 
            "S8.0_ZTA_Publish", "S9.0_Reboot", "S10.0_Halt_Error" }

VARIABLES 
    state,              (* Current state *)
    tcb_measured,       (* bool: S0.2 successful *)
    silicon_clean,      (* bool: S0.3 successful *)
    peripherals_purged, (* bool: S0.4 successful *)
    fido_ok,            (* bool: S1.0 successful *)
    pcp_verified,       (* bool: S2.0 successful *)
    firmware_matches,   (* bool: S5.2 successful *)
    policy              (* "flash", "clean", "dry_run" *)

vars == << state, tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

(* Initial state *)
Init == 
    /\ state = "S0.1_DRTM_Launch"
    /\ tcb_measured = FALSE
    /\ silicon_clean = FALSE
    /\ peripherals_purged = FALSE
    /\ fido_ok = FALSE
    /\ pcp_verified = FALSE
    /\ firmware_matches = FALSE
    /\ policy = "dry_run"

(* Helper for simple transitions *)
Transition(s, next_s) == 
    /\ state = s
    /\ state' = next_s
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

(* S0: TCB Launch *)
S0_1_to_S0_2 == Transition("S0.1_DRTM_Launch", "S0.2_TCB_Measure")

S0_2_to_S0_3 == 
    /\ state = "S0.2_TCB_Measure"
    /\ state' = "S0.3_Silicon_Poison_Check"
    /\ tcb_measured' = TRUE
    /\ UNCHANGED << silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

S0_3_to_S0_4 == 
    /\ state = "S0.3_Silicon_Poison_Check"
    /\ state' = "S0.4_Peripheral_Purge"
    /\ silicon_clean' = TRUE
    /\ UNCHANGED << tcb_measured, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

S0_4_to_S1_0 == 
    /\ state = "S0.4_Peripheral_Purge"
    /\ state' = "S1.0_FIDO_Quorum"
    /\ peripherals_purged' = TRUE
    /\ UNCHANGED << tcb_measured, silicon_clean, fido_ok, pcp_verified, firmware_matches, policy >>

(* S1: Delivery *)
S1_0_to_S1_1_or_S1_2 == 
    /\ state = "S1.0_FIDO_Quorum"
    /\ fido_ok' = TRUE
    /\ \/ state' = "S1.1_OOB_Deliver_LiveOS"
       \/ state' = "S1.2_PXE_Deliver_LiveOS"
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, pcp_verified, firmware_matches, policy >>

S1_1_to_S2_0 == Transition("S1.1_OOB_Deliver_LiveOS", "S2.0_PCP_Verify")
S1_2_to_S2_0 == Transition("S1.2_PXE_Deliver_LiveOS", "S2.0_PCP_Verify")

(* S2: Payload Verification *)
S2_0_to_S3_0 == 
    /\ state = "S2.0_PCP_Verify"
    /\ state' = "S3.0_Triage"
    /\ pcp_verified' = TRUE
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, firmware_matches, policy >>

(* S3: Analysis *)
S3_0_to_S3_1 == Transition("S3.0_Triage", "S3.1_Microcode_Verify")
S3_1_to_S4_0 == Transition("S3.1_Microcode_Verify", "S4.0_LiveOS_Attest")
S4_0_to_S5_0 == Transition("S4.0_LiveOS_Attest", "S5.0_Policy_Fetch")

(* S5: Remediation Logic *)
S5_0_to_S5_1 == 
    /\ state = "S5.0_Policy_Fetch"
    /\ state' = "S5.1_Flash_Verify"
    /\ policy' \in {"flash", "clean", "dry_run"}
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches >>

S5_1_to_S5_2 == Transition("S5.1_Flash_Verify", "S5.2_Firmware_Match")

S5_2_Logic == 
    /\ state = "S5.2_Firmware_Match"
    /\ firmware_matches' \in {TRUE, FALSE}
    /\ IF firmware_matches' THEN state' = "S5.3_Cleanup_Evaluate" ELSE state' = "S6.0_Firmware_Flash"
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, policy >>

S5_3_Logic == 
    /\ state = "S5.3_Cleanup_Evaluate"
    /\ IF policy = "clean" THEN state' = "S5.4_Malware_Cleanup" 
       ELSE IF policy = "flash" THEN state' = "S6.0_Firmware_Flash" 
       ELSE state' = "S7.0_Commit_Attest"
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

S5_4_to_S7_0 == Transition("S5.4_Malware_Cleanup", "S7.0_Commit_Attest")

(* S6-S9 *)
S6_0_to_S7_0 == Transition("S6.0_Firmware_Flash", "S7.0_Commit_Attest")
S7_0_to_S8_0 == Transition("S7.0_Commit_Attest", "S8.0_ZTA_Publish")
S8_0_to_S9_0 == Transition("S8.0_ZTA_Publish", "S9.0_Reboot")

(* Error handling *)
Halt_Error == 
    /\ state /= "S10.0_Halt_Error"
    /\ state' = "S10.0_Halt_Error"
    /\ UNCHANGED << tcb_measured, silicon_clean, peripherals_purged, fido_ok, pcp_verified, firmware_matches, policy >>

(* Next state relation *)
Next == 
    \/ S0_1_to_S0_2
    \/ S0_2_to_S0_3
    \/ S0_3_to_S0_4
    \/ S0_4_to_S1_0
    \/ S1_0_to_S1_1_or_S1_2
    \/ S1_1_to_S2_0
    \/ S1_2_to_S2_0
    \/ S2_0_to_S3_0
    \/ S3_0_to_S3_1
    \/ S3_1_to_S4_0
    \/ S4_0_to_S5_0
    \/ S5_0_to_S5_1
    \/ S5_1_to_S5_2
    \/ S5_2_Logic
    \/ S5_3_Logic
    \/ S5_4_to_S7_0
    \/ S6_0_to_S7_0
    \/ S7_0_to_S8_0
    \/ S8_0_to_S9_0
    \/ Halt_Error

(* Specification *)
Spec == Init /\ [][Next]_vars

====
