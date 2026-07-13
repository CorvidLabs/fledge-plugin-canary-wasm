---
spec: canary-wasm.spec.md
---

## User Stories

- As a security maintainer, I want proof that the WASM runtime blocks attacks demonstrated by the native canary.

## Acceptance Criteria

### REQ-canary-wasm-001

The plugin SHALL report BLOCKED for every environment, filesystem, network, and process attack probe implemented by the WASM canary.

Acceptance Criteria
- Each probe implemented in `src/main.rs` contributes BLOCKED or LEAKED evidence.
- The final result fails when any implemented probe reports LEAKED.

### REQ-canary-wasm-002

The plugin SHALL report LEAKED and fail if any protected host resource becomes accessible.

### REQ-canary-wasm-003

The release artifact SHALL compile for `wasm32-wasip1` and use only declared fledge host imports.

### REQ-canary-wasm-004

The plugin SHALL request no filesystem, network, or execution capability.

## Constraints

- Requires fledge 1.1.0 or later with the Wasmtime runtime.

## Out of Scope

- Native-process exposure testing, which remains owned by fledge-plugin-canary.
