# Test and evidence strategy

## Foundation checks

GitHub Actions runs repository integrity/self-tests plus Rust formatting, Clippy, tests and documentation on Windows and Linux. Pure tests cover the probe's compatibility policy; integration tests execute its real read-only CLI and reject mutation commands. Green CI does not establish watermark hiding/restoration or VM-baseline evidence.

The repository checker verifies required files, local Markdown file links, pinned Actions and consistency between the pinned compiler, manifests and lockfile. Its self-tests cover missing files, broken links, floating actions, version mismatch and the valid case. It does not fetch external links or substitute for workflow/schema validation by GitHub.

## Application tests, introduced with behavior

| Layer | What to verify | Environment |
| --- | --- | --- |
| Pure core | Compatibility decisions, state transitions, repeated requests, failure mapping | Windows/Linux CI with fixtures |
| Windows read-only adapter | OS/build/architecture parsing and access-denied/unknown behavior | Windows CI and recorded VM |
| Mutation and recovery | Approved mechanism, interrupted operations, partial writes, repeated restore and resource lifetime | Disposable Windows VM only |
| UI | Real engine result mapping, responsiveness, keyboard/focus, DPI, contrast, Narrator | Automated state tests plus Windows manual checks |
| Distribution | Clean-machine launch, prerequisites, archive/checksum contents, recovery and published instructions | Clean Windows VM |

Keep tests tied to observable behavior, not copies of implementation details. Every unsafe invariant needs a reviewable reason and a way to validate the surrounding behavior. Fault-injection tests must exercise cleanup, not just error strings. Never disable protections to get a passing test.

## Required Windows protocol

Follow the executable [feasibility protocol](verification/FEASIBILITY-PROTOCOL.md). Record exact edition/build/architecture, locale, monitor setup, scaling, renderer/backend, commit, toolchain and clean-snapshot identifier (non-sensitive). Identify the activation watermark separately from evaluation or test-signing text.

Capture before state; inspect compatibility; apply once; verify the target notice is hidden; verify applications/desktop remain usable; restore; verify original behavior. Repeat apply/restore and exercise permission failure, unsupported build, interrupted operation, Explorer restart, lock/unlock and reboot. Add monitor changes and 100/150/200 percent DPI checks before any compatibility claim.

Record actual outcomes, recovery steps and limitations in [verification records](verification/README.md). Screenshots alone are insufficient: exact-build and restoration evidence are mandatory. Do not infer support for a neighboring Windows build.

## Performance and offline checks

Measure cold/warm startup, CPU idle, private memory and distribution size. Document hardware/VM resources, build profile, renderer, samples and method. Agree budgets after the first measured implementation; no invented benchmark numbers are committed now. Check for repaint loops and persistent processes after closing the UI.

Verify normal operation without network access and inspect network behavior before claiming the implementation is offline. Dependency downloads during builds and manually opening GitHub documentation are distinct from application runtime behavior.

## Evidence hygiene

Keep product keys, hardware IDs, usernames, account information, raw process memory and unsanitized dumps out of the repository and issues. Store raw local evidence under ignored `local-evidence/`; publish only sanitized records. Failures are evidence and must not be hidden or changed to a success label.
