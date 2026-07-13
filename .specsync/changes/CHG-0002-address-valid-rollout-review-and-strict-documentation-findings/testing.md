---
change: CHG-0002-address-valid-rollout-review-and-strict-documentation-findings
artifact: testing
---

# Testing

- REQ-canary-wasm-001: inspect the probe calls and LEAKED failure counter in `src/main.rs`; run formatting, Clippy, the `wasm32-wasip1` release build, and artifact verification through the repository verify lane.
