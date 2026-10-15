# GhostMark contributor-agent instructions

Read README.md, docs/roadmap.md, docs/workflow.md and the issue before editing. This repository is preparing feasibility evidence. A read-only environment probe exists; watermark operations and the desktop application do not. Do not convert planned features into claims of working behavior.

## Ponytail approach

Apply [Ponytail](https://github.com/DietrichGebert/ponytail/tree/main/skills/ponytail): make the smallest complete change that satisfies the issue. Read callers and the real flow first; reuse platform/standard-library behavior and existing dependencies. Do not add speculative abstractions, dependencies or options. Never cut validation, recovery, security or accessibility. Non-trivial behavior changes require a meaningful small test. Report what was not checked and material risks.

Ponytail is an authoring aid, not a runtime dependency or a requirement that contributors install an agent. These are project-specific rules; the upstream skill is not copied or vendored here.

## Boundaries

- Follow milestone gates; no hiding implementation before the feasibility protocol and no GUI before the engine gate.
- Do not test mutations on the user's live desktop or on a hosted CI runner. Use a disposable, recorded Windows VM.
- Keep unsafe/Windows adapters small, explain invariants, and reject unsupported or unknown builds.
- Do not alter activation/licensing state, disable protections, introduce hidden persistence or publish device identifiers.
- Add only necessary dependencies; pin the toolchain and commit Cargo.lock. Document important decisions in docs/adr/.
- Do not add service frameworks, async runtimes or separate helper processes until a demonstrated need justifies them.

## Delivery

Use a focused branch and linked PR, run the checks listed in CONTRIBUTING.md, and verify required CI on the latest commit. Never treat an empty crate's green tests as functional verification. Keep GitHub Issues/Project state current; do not duplicate live task state in versioned documents. Do not publish releases before docs/releasing.md is satisfied.
