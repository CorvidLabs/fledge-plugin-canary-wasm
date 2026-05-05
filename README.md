# fledge-plugin-canary-wasm

[![CI](https://github.com/CorvidLabs/fledge-plugin-canary-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/CorvidLabs/fledge-plugin-canary-wasm/actions/workflows/ci.yml)

> **WASM (sandboxed) security canary** — proves the Wasmtime sandbox blocks every attack that the native canary exposes.
>
> Companion: [fledge-plugin-canary](https://github.com/CorvidLabs/fledge-plugin-canary) — the unsandboxed native counterpart that proves the attacks work in the first place.

WASM sandbox verification plugin for [fledge](https://github.com/CorvidLabs/fledge). Written in **Rust**, compiled to `wasm32-wasip1`, and runs inside fledge's Wasmtime sandbox — the new runtime introduced in fledge 1.1.0. Attempts every attack from the native canary and verifies each one is blocked.

## How It Works

A Rust program compiled to `wasm32-wasip1` that runs inside fledge's Wasmtime sandbox with `filesystem=none`, `network=false`, `exec=false`. It attempts every attack from the native canary's baseline:

- **Environment variables** — GITHUB_TOKEN, AWS keys, HOME, PATH, etc.
- **Filesystem reads** — credential files, /etc/hosts, path traversal, directory listing
- **Filesystem writes** — /tmp, working directory, .git/hooks injection
- **Network** — TCP connections to external hosts
- **Process spawning** — echo, curl, cat, crontab, pbpaste, ps, whoami

Every test should report **BLOCKED**. Any **LEAKED** result indicates a sandbox escape.

## Native vs WASM Comparison

| Attack | Native (bash) | WASM (sandbox) |
|--------|:---:|:---:|
| Read ~/.ssh/id_ed25519 | LEAKED | BLOCKED |
| Read ~/.aws/credentials | LEAKED | BLOCKED |
| Read ~/.config/fledge/config.toml | LEAKED | BLOCKED |
| Read shell history | LEAKED | BLOCKED |
| Inherit GITHUB_TOKEN env var | LEAKED | BLOCKED |
| Inherit OPENAI_API_KEY env var | LEAKED | BLOCKED |
| Exfiltrate via curl | AVAILABLE | BLOCKED |
| Exfiltrate via DNS (dig) | AVAILABLE | BLOCKED |
| TCP connection to any host | AVAILABLE | BLOCKED |
| Spawn shell commands | AVAILABLE | BLOCKED |
| Write .git/hooks (backdoor) | WRITABLE | BLOCKED |
| Write shell RC files | WRITABLE | BLOCKED |
| Install LaunchAgent daemon | WRITABLE | BLOCKED |
| Read clipboard (pbpaste) | AVAILABLE | BLOCKED |
| Schedule crontab | AVAILABLE | BLOCKED |
| List processes (ps aux) | AVAILABLE | BLOCKED |

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-canary-wasm
```

Requires fledge 1.1.0+ (WASM runtime support).

## Usage

```bash
fledge canary-wasm
```

## Build from Source

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
# Binary: target/wasm32-wasip1/release/canary-wasm.wasm
```

## How This Differs from the Native Canary

| | Native ([canary](https://github.com/CorvidLabs/fledge-plugin-canary)) | WASM (this repo) |
|---|---|---|
| **Language** | Bash | Rust |
| **Runtime** | Unsandboxed subprocess | Wasmtime WASM sandbox |
| **fledge version** | Any | 1.1.0+ |
| **Purpose** | Prove attacks **work** | Prove sandbox **blocks** them |
| **Expected result** | LEAKED / AVAILABLE / WRITABLE | BLOCKED on every test |
| **What it validates** | The attack surface exists | The sandbox closes it |

Together, the two canaries provide end-to-end validation: the native canary proves the threat is real, and the WASM canary proves the mitigation works.

## Why This Exists

The native canary proves the attacks work. The WASM canary proves the sandbox stops them. Together they validate fledge's security model end-to-end.

Run this after any change to:
- WASM runtime or WASI configuration
- Plugin capability enforcement
- Sandbox resource limits (fuel, epoch, memory)

## Supported Languages

Any language that compiles to `wasm32-wasip1` can be used to write fledge WASM plugins:

- **Rust** — first-class support, what this canary uses
- **C/C++** — via wasi-sdk or Emscripten
- **TinyGo** — Go's WASM-targeting compiler
- **AssemblyScript** — TypeScript-like, designed for WASM
- **Zig** — native WASM target support

The guest must import `fledge::send`, `fledge::recv`, and `fledge::exit` from the `"fledge"` WASM import module.

## License

MIT
