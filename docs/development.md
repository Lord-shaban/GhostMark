# Development environment

## Prerequisites

- Git and a GitHub account for contributions.
- Rustup; the repository pins Rust 1.99.0 with rustfmt and Clippy.
- Windows builds: Microsoft C++ Build Tools and Windows SDK, MSVC x64 toolchain.
- Python 3.11+ for repository checks; no third-party Python packages required.
- A disposable Windows 11 x64 VM for later platform experiments. Hosted CI is not the mutation test environment.

The Rust compiler version was selected from the official stable channel manifest during repository setup on 2026-10-15. Update it through a focused PR that also updates `rust-version`, verifies dependencies and reruns CI. Do not track floating `stable` in the repository file. `rust-version` is an MSRV declaration; this stage tests the pinned compiler, not a range of older compilers.

## Commands

From the repository root:

```powershell
rustup show active-toolchain
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
python tools/check_repository.py --self-test
python tools/check_repository.py
```

There is no application to run yet. Do not expect `cargo run` or a downloadable executable. The only crate is a documentation-only core foundation. Zero Rust behavior tests at this stage is expected and does not verify the planned watermark mechanism.

## Dependencies and layout

The workspace uses resolver 3 and edition 2024. Commit Cargo.lock. Introduce `windows` in the read-only probe task and `egui`/`eframe` in the desktop task, with narrow required features and compatible pinned lockfile entries. Cargo is the build system; do not introduce a second build system without a demonstrated requirement.

Use modules in the existing crate while responsibility remains clear. Split into crates only when an actual platform or dependency boundary justifies it. The core's portable tests run on Linux and Windows; Windows adapter code will be target-gated and must get Windows CI coverage when added.

Keep local builds in `target/`, unpublished artifacts in `artifacts/`, and unsanitized local test evidence in `local-evidence/`; all are ignored. Publish only sanitized evidence tied to the tested commit.

## Reproducibility and releases

Pin external GitHub Actions to full commit SHAs. Dependabot proposes Cargo and Actions updates weekly; dependency review checks PR dependency changes. Passing these checks does not prove a dependency or release safe. Toolchain and lockfile pins make inputs traceable, not a promise of bit-for-bit reproducible binaries.

No release workflow is included yet because there is no application artifact to release. Implement the real packaging workflow in issue #14, after clean-machine runtime requirements are known.
