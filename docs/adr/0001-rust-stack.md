# ADR-0001: Rust stack for GhostMark

## Status

Accepted for planning, 2026-10-15. The user selected Rust. Feasibility of the actual mechanism remains unproven.

## Context

GhostMark is a small Windows-specific utility. It needs direct platform access, explicit resource lifetime management and a compact maintainable interface. Supporting another operating system is not an initial requirement. A stack decision cannot guarantee compatibility with undocumented Windows internals.

## Decision

Use stable Rust, edition 2024, Cargo and the MSVC Windows target. Pin compiler inputs in rust-toolchain.toml and dependency resolution in Cargo.lock. Start with a documentation-only core crate; add the Microsoft `windows` crate during the read-only probe task and `egui`/`eframe` after the engine gate. Enable only required features.

Use one language for the core and desktop interface. Keep unsafe Windows/FFI code in a narrow adapter with documented invariants. Pure core logic stays portable/testable. Introduce additional crates or an elevated helper only when the proven method requires a concrete boundary.

Use native desktop rendering rather than a browser UI. Select/test renderer features during UI implementation, including VM/driver prerequisites. Design the interface for this utility rather than promising automatic pixel-identical Windows controls.

## Alternatives

- C#/.NET WPF plus C++: good Windows integration and UI tooling, but adds another language/runtime boundary contrary to the user's Rust preference.
- C++/Win32 throughout: direct control and fewer runtime requirements, but manual resource/unsafe-memory management and more custom UI work.
- Rust plus Tauri: useful for a web-driven or cross-platform product; here the WebView/frontend toolchain adds a boundary without a current requirement.
- Rust with a more elaborate declarative UI framework: revisit if egui fails concrete accessibility or layout requirements; evaluate licensing before adoption.

## Consequences

Safe Rust improves memory/lifetime checks for safe code; raw Windows operations still require careful unsafe/FFI review. egui is custom rendered, so Windows-native visual fidelity and accessibility need actual validation. Startup, memory, binary size, graphics dependencies and CRT prerequisites are measured properties, not guaranteed benefits.

No async runtime, database, account backend, updater, service or plugin framework is included. Repository checks use Python's standard library as development tooling; Python is not an application runtime dependency. MIT is the project license; new dependency licenses require review.

## Evidence and references

- [Microsoft windows-rs](https://github.com/microsoft/windows-rs)
- [egui and eframe](https://github.com/emilk/egui)
- [Rust editions](https://doc.rust-lang.org/edition-guide/editions/index.html)
- [Rustup toolchain files](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
- [Cargo lockfile guidance](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)

The stack is selected from requirements and documented capabilities; no GhostMark performance benchmark or mechanism test has been run at this stage.
