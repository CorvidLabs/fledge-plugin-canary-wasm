---
module: canary-wasm
version: 1
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Canary-wasm

## Purpose

Verify that fledge's Wasmtime sandbox blocks the environment, filesystem, network, process, and persistence attacks demonstrated by the native Canary plugin.

## Public API

| Surface | Expected result |
|---------|-----------------|
| Environment variables | Sensitive and host variables are absent. |
| Filesystem reads | Credentials, host files, traversal, and directory listing are blocked. |
| Filesystem writes | Temporary, working-tree, hook, and persistence writes are blocked. |
| Network | External TCP and exfiltration tools are unavailable. |
| Process execution | Shell commands and host utilities cannot be spawned. |
| Fledge host calls | Communication occurs only through declared fledge WASM imports. |

## Invariants

1. Every native-canary attack must report BLOCKED inside the configured WASM sandbox.
2. Any readable secret, writable host path, network connection, or spawned process is a LEAKED failure.
3. The guest relies only on the declared fledge send, receive, and exit imports.
4. The release artifact targets `wasm32-wasip1`.
5. The plugin requests no filesystem, network, or exec capability.

## Behavioral Examples

```
Given the WASM guest runs with no filesystem preopens, network, or exec capability
When it attempts to read a host credential file
Then the attempt reports BLOCKED and no content is returned
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Sandbox escape | A host secret, path, network socket, or process becomes accessible | Report LEAKED and fail. |
| Missing fledge import | The host does not provide the required protocol call | Exit with a clear runtime failure. |
| Build target unavailable | `wasm32-wasip1` is not installed | Fail the native build lane before packaging. |

## Dependencies

- Rust stable with `wasm32-wasip1`
- fledge 1.1.0 or later Wasmtime runtime
- fledge WASM send, receive, and exit imports
- native fledge-plugin-canary threat comparison

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing WASM sandbox canary behavior for SpecSync 5 adoption. |
