# Sovereign Recovery Protocol (SRP) v2.4
## A Formally-Verified, Zero-Trust Framework for Endpoint Restoration

**Abstract**
Persistent, below-OS malware—firmware rootkits, supply-chain implants, even GPU-VRAM or CPU-micro-code payloads—renders “detect-and-remove” tools impotent. Sovereign Recovery Protocol (SRP) replaces cleaning with verifiable rebuilds: a RAM-only Live OS executes inside a formally verified seL4 micro-hypervisor launched by hardware Dynamic Root of Trust for Measurement (DRTM).

**New in v2.4:**
* Adaptive Cleanup Path: Conditional malware removal (e.g., AV disinfect) when firmware is clean.
* Silicon-Poison detection & micro-code re-patch.
* Peripheral-Purge: PCIe Function-Level Reset + dynamic VRAM scrub.
* Out-of-Band LiveOS delivery path.
* Complete TLA+ spec and Rust implementation.
