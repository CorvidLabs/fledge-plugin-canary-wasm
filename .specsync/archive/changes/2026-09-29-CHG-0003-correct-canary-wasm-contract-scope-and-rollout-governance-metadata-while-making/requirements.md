---
change: CHG-0003-correct-canary-wasm-contract-scope-and-rollout-governance-metadata-while-making
artifact: requirements
---

# Requirements

The canonical public contract SHALL name only attack categories that the WASM guest implements.

Acceptance Criteria
- Purpose lists environment, filesystem, network, and process attacks.
- Public API describes temporary, working-tree, and hook write probes without claiming a separate persistence probe.
- The existing REQ-canary-wasm-001 BLOCKED/LEAKED behavior remains unchanged.
