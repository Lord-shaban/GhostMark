# Engineering workflow

## Source of truth

- Versioned docs: product boundary, decisions, milestone exit gates and verification protocols.
- GitHub Issues: task outcome, dependencies, acceptance criteria and validation evidence.
- [Project](https://github.com/users/Lord-shaban/projects/11): live state and priority.
- Pull requests: implementation, review and exact-commit checks.

Do not maintain duplicate live task checklists in Markdown. Update this process only when the process changes.

## States

| State | Meaning |
| --- | --- |
| Backlog | Proposed/triaged work without an active commitment |
| Ready | All blockers closed with evidence; outcome and validation understood |
| In Progress | Work is active; default work-in-progress limit is one issue per contributor |
| In Review | Linked PR/evidence is ready for review and current CI is running or complete |
| Blocked | An explicit dependency or external condition prevents execution; link it |
| Done | Criteria verified, required checks passed, PR merged and issue closed |

Project states are maintained by the person doing the work. No automation silently marks a task complete. Closing an issue alone does not prove acceptance criteria were met. Priority P0 means it blocks its milestone exit gate; P1 is planned required work. Priority is represented by the issue label and shown in the Project Labels field.

## Change path

1. Identify an unblocked issue and move it to In Progress.
2. Branch from current `main`: `feat/…`, `fix/…`, `docs/…`, `test/…`, `ci/…` or `chore/…`.
3. Make the smallest complete change, including affected callers, tests and documentation.
4. Run local checks and open a PR with the template and linked issue.
5. Move to In Review; attach behavior evidence and state what was not verified.
6. Resolve review threads and pass all required checks on the latest commit.
7. Squash merge, close the completed issue, move it to Done, and move newly unblocked work to Ready.

Use a draft PR when feedback is needed before the work satisfies its criteria. Do not use `Closes` for a partially completed issue.

## Main branch rules

Require a PR, resolved review conversations, linear history and required CI. Disallow force-pushes and branch deletion. No administrator bypass is configured. Independent approval is encouraged but not required for this initially solo-maintained repository. All contributors still inspect the full change and evidence; see [governance](../GOVERNANCE.md).

Required contexts are `Repository checks`, `Rust (windows-latest)`, and `Rust (ubuntu-latest)`. Additional dependency-review results must be considered during review; it is not a required context on the initial push-only bootstrap because it runs only for PRs. A change to workflow/check names must update branch protection in the same maintenance operation to avoid deadlocking merges.

## Phase and release gates

Do not start M2 until ADR-0002 records a feasible method with restoration evidence. Do not implement the UI before the engine gate. Do not publish an application release before the compatibility, performance, documentation and clean-machine gates pass. A no-go result returns the plan to research; it is not worked around by claiming a cosmetic overlay removed the actual watermark.

Milestones have no fabricated due dates. Releases are gated by evidence, not a calendar deadline. See [roadmap](roadmap.md) and [releasing](releasing.md).
