# Contributing to GhostMark

GhostMark is in the foundation/feasibility stage. Start with the [roadmap](docs/roadmap.md) and an existing issue. Do not add a UI feature or a system-changing mechanism before its prerequisite gate passes.

## Before a change

- Check for an existing issue. For a new feature, describe the user outcome, supported environment and validation plan first.
- Move to Ready only when dependencies are closed with evidence. Keep the Project status current while working.
- Use a short branch such as `docs/compatibility-protocol`, `feat/environment-probe`, or `fix/recovery-state`.
- Keep changes small. Reuse the standard library, platform APIs and existing dependencies before adding a library.

## Development

Follow the [development guide](docs/development.md). Use the pinned stable toolchain, commit `Cargo.lock`, and run formatting, Clippy, tests, docs and repository checks. Add dependencies only in the task that uses them, enable the narrowest required features, and inspect their licenses.

Use descriptive Conventional Commit titles (`feat:`, `fix:`, `docs:`, `test:`, `ci:`, `chore:`). Sign off commits with `git commit -s` to attest that you have the right to contribute the changes under the MIT license; no separate CLA is required. A sign-off is not a cryptographic signature.

## Pull requests

Link the issue with `Closes #number` when the PR completes it. Describe the trigger, resulting behavior, validation and meaningful limitations. Include UI evidence when UI changes exist and exact Windows-build evidence when compatibility claims change. Never include product keys, device identifiers, credentials or unsanitized memory dumps.

Required checks must pass on the current PR commit and review conversations must be resolved. The maintainer uses the same PR route; independent review is encouraged when another contributor is available, but a solo maintainer is not required to approve their own PR. Squash merging keeps one coherent commit per change.

## Windows and unsafe changes

Use disposable VMs for experiments that can modify another process or desktop behavior. Normal hosted CI must never apply the mechanism to its own desktop. Document each `unsafe` block's preconditions, ownership, lifetimes, pointer validity and failure behavior. Exercise rollback and partial failure, not just successful hiding.

Unknown builds stay unsupported until revalidated. Do not disable security protections to make a test pass or bundle third-party binaries without verifiable source/provenance and compatible licensing.

## Reporting

Use the bug and feature templates for public reports. Send vulnerabilities through the [security policy](SECURITY.md). Be respectful and follow the [Code of Conduct](CODE_OF_CONDUCT.md).
