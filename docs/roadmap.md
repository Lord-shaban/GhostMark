# Engineering roadmap

This document defines scope, sequence and exit gates. [Issues](https://github.com/Lord-shaban/GhostMark/issues) own acceptance criteria; the [public Project](https://github.com/users/Lord-shaban/projects/11) owns live status. No dates are promised without known capacity and measured progress.

## Product boundary

The first product is a local Windows 11 x64 utility to inspect compatibility, hide the activation notice on explicitly verified builds, and restore its original behavior. It does not activate Windows or change licensing. No application has been implemented. A feasibility no-go is a valid engineering result.

Initial scope excludes automatic sign-in reapplication, services, auto-updaters, cross-platform support, plugin systems, accounts and telemetry. Add any of these only through a separate justified issue/decision.

## Phases and gates

| Milestone | Work | Exit gate | Issues |
| --- | --- | --- | --- |
| M0 — Repository foundation | Community files, plan, Rust scaffold, CI, ownership and protected main | Foundation PR merged with passing checks; Project and backlog usable | [#1](https://github.com/Lord-shaban/GhostMark/issues/1) |
| M1 — Feasibility and compatibility | Baseline/protocol, read-only probe, isolated experiment, decision ADR | Repeatable hide **and restore** on one exact build; change surface and permissions documented; go/no-go recorded | [#2](https://github.com/Lord-shaban/GhostMark/issues/2), [#3](https://github.com/Lord-shaban/GhostMark/issues/3), [#4](https://github.com/Lord-shaban/GhostMark/issues/4), [#5](https://github.com/Lord-shaban/GhostMark/issues/5) |
| M2 — Engine and recovery | Approved method, explicit state/errors, partial-failure handling, regression fixtures | Rejection of unsupported builds and recovery scenarios pass; unsafe invariants reviewed | [#6](https://github.com/Lord-shaban/GhostMark/issues/6), [#7](https://github.com/Lord-shaban/GhostMark/issues/7), [#8](https://github.com/Lord-shaban/GhostMark/issues/8) |
| M3 — Desktop experience | egui shell, real action/state wiring, accessibility/DPI | Success and failure flows verified; essential controls usable by keyboard; accessibility limitations recorded | [#9](https://github.com/Lord-shaban/GhostMark/issues/9), [#10](https://github.com/Lord-shaban/GhostMark/issues/10), [#11](https://github.com/Lord-shaban/GhostMark/issues/11) |
| M4 — Validation and performance | Exact-build matrix, recovery, benchmarks, offline verification | All advertised compatibility claims have evidence; performance measured and budgets agreed | [#12](https://github.com/Lord-shaban/GhostMark/issues/12), [#13](https://github.com/Lord-shaban/GhostMark/issues/13) |
| M5 — Preview and first release | Portable preview, provenance/checksums, public docs, feedback and final gate | Clean-VM end-to-end validation; release checklist complete; no unresolved recovery blocker | [#14](https://github.com/Lord-shaban/GhostMark/issues/14), [#15](https://github.com/Lord-shaban/GhostMark/issues/15), [#16](https://github.com/Lord-shaban/GhostMark/issues/16) |

## Critical path

`#1 → #2 → #3 → #4 → #5 → #6 → #7 → #8 → #9 → #10 → #11 → #12 → #13 → #14 → #15 → #16`

Some tasks have additional prerequisites listed in their issue. A later-phase task stays Blocked until all of them are done. Work need not invent parallel tracks for a small project. Documentation and fixture work can accompany the implementation that needs it.

## Definition of ready and done

Ready means a concrete outcome, closed dependencies, acceptance criteria, a validation method and a milestone. Done means the criteria are met, tests/evidence are attached, current required checks pass, the PR is merged, documentation reflects reality and the issue/Project state is updated. See [workflow](workflow.md).

## Risks and decisions

| Risk | Response |
| --- | --- |
| Windows rendering details change between builds | Refuse unverified builds; revalidate after updates |
| A candidate hides the wrong watermark | Distinguish activation from evaluation/test-mode in the baseline |
| Partial operation leaves desktop behavior altered | Design and test rollback before calling the method feasible |
| Privileged or unsafe code expands | Keep a narrow adapter; record invariants and justify helper boundaries |
| GUI work hides missing feasibility | No GUI implementation until M2 evidence exists |
| Signing, renderer or runtime assumptions fail on a clean machine | Verify prerequisites, disclose signing status and test distribution in M5 |

The first decision is [ADR-0001: Rust stack](adr/0001-rust-stack.md). The next decision, ADR-0002, must be written from M1 evidence rather than a guessed renderer or a fixed memory offset.
