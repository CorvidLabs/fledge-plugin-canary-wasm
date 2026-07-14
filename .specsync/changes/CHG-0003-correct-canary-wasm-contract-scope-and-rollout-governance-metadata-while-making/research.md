---
change: CHG-0003-correct-canary-wasm-contract-scope-and-rollout-governance-metadata-while-making
artifact: research
---

# Research

Inspection of `src/main.rs` found environment-variable probes, filesystem read and write probes, TCP/process probes, and clipboard/system-info probes implemented through process execution. There is no independent persistence category or persistence-specific probe. The `.specsync/sdd.json` inventory omitted all four installed agent directories, and Gemini's generated command wrapped an invocation placeholder in another literal quote layer.
