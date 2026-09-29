---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-wasm-canary-fledge-plugi
artifact: testing
---

# Testing

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo build --target wasm32-wasip1 --release`
- Verify the release WASM artifact
- `specsync check --strict --require-coverage 100 --force`
- `fledge trust doctor` and `fledge trust verify`
