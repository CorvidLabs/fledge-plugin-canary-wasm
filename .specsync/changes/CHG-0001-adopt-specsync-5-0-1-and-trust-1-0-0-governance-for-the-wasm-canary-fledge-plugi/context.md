---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-wasm-canary-fledge-plugi
artifact: context
---

# Context

This Rust WASM guest is the sandboxed counterpart to the native Canary. It must continue treating any protected host access as a sandbox escape while preserving the dedicated wasm32-wasip1 build.
