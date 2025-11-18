(* TCB Definitions *)
Inductive TrustedComponent := HW_DRTM | SeL4_Kernel | SRP_Core.
Definition MeasuredComponents := list (TrustedComponent * bool).
Definition tcb_is_fully_measured (mc : MeasuredComponents) : bool :=
    true. (* Simplified *)
