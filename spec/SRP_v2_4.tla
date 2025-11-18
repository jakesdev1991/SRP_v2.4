EXTENDS Naturals, FiniteSets, TLC
STATES == { "S0.1_DRTM_Launch", "S0.2_TCB_Measure", "S0.3_Silicon_Poison_Check", 
            "S0.4_Peripheral_Purge", "S1.0_FIDO_Quorum", "S1.1_OOB_Deliver_LiveOS", 
            "S1.2_PXE_Deliver_LiveOS", "S2.0_PCP_Verify", "S3.0_Triage", 
            "S3.1_Microcode_Verify", "S4.0_LiveOS_Attest", "S5.0_Policy_Fetch", 
            "S5.1_Flash_Verify", "S5.2_Firmware_Match", "S5.3_Cleanup_Evaluate", 
            "S5.4_Malware_Cleanup", "S6.0_Firmware_Flash", "S7.0_Commit_Attest", 
            "S8.0_ZTA_Publish", "S9.0_Reboot", "S10.0_Halt_Error" }

VARIABLES state, tcb_measured, silicon_clean, peripherals_purged, fido_ok, 
          pcp_verified, firmware_matches, policy

Init == 
    /\ state = "S0.1_DRTM_Launch"
    /\ tcb_measured = FALSE
    /\ silicon_clean = FALSE
    /\ peripherals_purged = FALSE
    /\ fido_ok = FALSE
    /\ pcp_verified = FALSE
    /\ firmware_matches = FALSE
    /\ policy = "dry_run"

Transition(s, next_s) == /\ state = s /\ state' = next_s

S0_1_to_S0_2 == Transition("S0.1_DRTM_Launch", "S0.2_TCB_Measure")
Halt_Error == /\ state \in STATES \ {"S10.0_Halt_Error"} /\ state' = "S10.0_Halt_Error"

Next == S0_1_to_S0_2 \/ Halt_Error (* Truncated for brevity *)
Spec == Init /\ [][Next]_vars
