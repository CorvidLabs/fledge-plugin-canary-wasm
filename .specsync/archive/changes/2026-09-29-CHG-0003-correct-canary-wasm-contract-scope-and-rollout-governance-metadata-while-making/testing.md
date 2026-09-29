---
change: CHG-0003-correct-canary-wasm-contract-scope-and-rollout-governance-metadata-while-making
artifact: testing
---

# Testing

- REQ-canary-wasm-001: run formatting and Clippy, build the release artifact for `wasm32-wasip1`, verify that the artifact exists, and inspect `src/main.rs` to confirm each implemented probe contributes BLOCKED or LEAKED evidence and any LEAKED result exits non-zero.
