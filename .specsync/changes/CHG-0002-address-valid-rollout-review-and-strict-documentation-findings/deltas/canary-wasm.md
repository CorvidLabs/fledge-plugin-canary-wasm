## MODIFIED

### SPEC SECTION Invariants

1. Every environment, filesystem, network, and process attack probe implemented by the WASM canary must report BLOCKED inside the configured sandbox.
2. Any readable secret, writable host path, network connection, or spawned process observed by those probes is a LEAKED failure.
3. The guest relies only on the declared fledge send, receive, and exit imports.
4. The release artifact targets `wasm32-wasip1`.
5. The plugin requests no filesystem, network, or exec capability.

### REQUIREMENT REQ-canary-wasm-001

The plugin SHALL report BLOCKED for every environment, filesystem, network, and process attack probe implemented by the WASM canary.

Acceptance Criteria
- Each probe implemented in `src/main.rs` contributes BLOCKED or LEAKED evidence.
- The final result fails when any implemented probe reports LEAKED.
