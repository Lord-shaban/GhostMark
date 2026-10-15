# GhostMark

**A small Rust desktop utility for controlling the visibility of the Windows activation watermark.**

[![CI](https://github.com/Lord-shaban/GhostMark/actions/workflows/ci.yml/badge.svg)](https://github.com/Lord-shaban/GhostMark/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[العربية](docs/README.ar.md) · [Roadmap](docs/roadmap.md) · [Engineering Project](https://github.com/users/Lord-shaban/projects/11) · [Contributing](CONTRIBUTING.md)

## Project status

**Feasibility preparation. A read-only diagnostic probe is implemented; no desktop application or remover has been released.**

The next gate is an isolated Windows feasibility experiment using the [baseline protocol](docs/verification/FEASIBILITY-PROTOCOL.md). Hiding and restoring the activation notice must both be demonstrated on an exact Windows build before engine/UI development proceeds. An evaluation/build watermark is a separate case and is not evidence that the activation notice can be hidden.

There is currently no download, supported-build claim, screenshot of a working product, or performance claim. See the [compatibility record](docs/compatibility.md).

## Intended scope

- Windows 11 x64 first, with explicit support for individually tested builds.
- Inspect compatibility, hide the activation notice, and restore the original behavior.
- A small desktop interface with readable status, recovery instructions and actionable errors.
- Local operation with no telemetry or account requirement.

GhostMark is intended to change visual behavior only. It does not activate Windows, provide a license, unlock licensed features, or change activation state. The feasibility gate may reject a method or the project direction if reliable recovery cannot be demonstrated.

## Stack

| Component | Decision |
| --- | --- |
| Implementation | Rust stable, edition 2024 |
| Windows integration | Microsoft `windows` bindings, read-only environment adapter |
| Desktop interface | `egui` / `eframe`, introduced after the engine gate |
| Build | Cargo, Windows MSVC target |
| Automation | GitHub Actions, SHA-pinned external actions |
| License | MIT |

The workspace contains a core inspection/policy library and a small [read-only JSON probe](docs/probe.md). UI libraries are deferred until the engine gate. No production build is marked supported. See [ADR-0001](docs/adr/0001-rust-stack.md).

## Get started

Install [Rust through rustup](https://rustup.rs/). On Windows, select the MSVC toolchain and install Microsoft's C++ Build Tools with the Windows SDK. The compiler/linker prerequisites are for development; end-user runtime requirements will be established by clean-machine packaging tests.

```powershell
git clone https://github.com/Lord-shaban/GhostMark.git
cd GhostMark
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
python tools/check_repository.py
cargo run -p ghostmark-probe -- inspect
```

Repository checks require Python 3.11 or newer and use only the standard library. Rustup reads the pinned compiler in `rust-toolchain.toml`. Tests exercise read-only inspection and compatibility decisions; they do not establish that watermark hiding/restoration works. The probe has no system-changing commands and does not inspect activation state or product keys.

## How work moves

1. Pick an unblocked issue from the [Project](https://github.com/users/Lord-shaban/projects/11).
2. Create a short-lived branch and open a focused pull request.
3. Add evidence and tests proportionate to the changed behavior.
4. Merge only after required checks pass and review threads are resolved.

`main` is protected. The maintainer uses the same PR path. The [workflow](docs/workflow.md) defines readiness, completion and release gates; GitHub Issues and Project fields own live status.

## Repository map

```text
.github/                 CI, dependency updates, ownership and issue/PR templates
crates/ghostmark-core/   Environment report, pure compatibility policy, read-only Win32 adapter
crates/ghostmark-probe/  Diagnostic CLI; inspect only
docs/                   Roadmap, architecture, ADRs, test and release protocols
docs/verification/      Templates for reproducible Windows evidence
tools/                  Repository documentation/configuration checks
```

## Community and security

Read [CONTRIBUTING](CONTRIBUTING.md), [GOVERNANCE](GOVERNANCE.md), the [Code of Conduct](CODE_OF_CONDUCT.md), and the [security policy](SECURITY.md). Report security issues privately using [GitHub's vulnerability form](https://github.com/Lord-shaban/GhostMark/security/advisories/new).

Copyright © 2026 Ahmed Sha’ban. Licensed under [MIT](LICENSE).
