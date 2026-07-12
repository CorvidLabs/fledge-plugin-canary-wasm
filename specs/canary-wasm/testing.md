---
spec: canary-wasm.spec.md
---

## Test Plan

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo build --target wasm32-wasip1 --release`
- Verify the release `.wasm` artifact exists.
