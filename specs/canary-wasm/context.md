---
spec: canary-wasm.spec.md
---

## Context

This is the sandboxed counterpart to the native Canary and validates the mitigation rather than the threat surface.

## Related Modules

- fledge WASM runtime
- fledge-plugin-canary

## Design Decisions

- Treat every accessible protected host resource as a sandbox escape.
- Compile to WASI Preview 1 for the supported fledge runtime.
