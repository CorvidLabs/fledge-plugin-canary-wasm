## MODIFIED

### SPEC SECTION Purpose

Verify that fledge's Wasmtime sandbox blocks the environment, filesystem, network, and process attacks implemented by the WASM Canary plugin.

### SPEC SECTION Public API

| Surface | Expected result |
|---------|-----------------|
| Environment variables | Sensitive and host variables are absent. |
| Filesystem reads | Credentials, host files, traversal, and directory listing are blocked. |
| Filesystem writes | Temporary, working-tree, and hook writes are blocked. |
| Network | External TCP and exfiltration tools are unavailable. |
| Process execution | Shell commands and host utilities cannot be spawned. |
| Fledge host calls | Communication occurs only through declared fledge WASM imports. |

### SPEC SECTION Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing WASM sandbox canary behavior for SpecSync 5 adoption. |
| 2 | 2026-07-13 | Address valid rollout review and strict documentation findings. |

### REQUIREMENT REQ-canary-wasm-001

The plugin SHALL report BLOCKED for every environment, filesystem, network, and process attack probe implemented by the WASM canary.

Acceptance Criteria
- Each probe implemented in `src/main.rs` contributes BLOCKED or LEAKED evidence.
- The final result fails when any implemented probe reports LEAKED.
