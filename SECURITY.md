# Security policy

## Current support

No application release exists yet. The default branch receives foundation fixes; there is no supported released binary or promise of support for old tags. Release support windows will be documented when the first preview ships.

## Report a vulnerability privately

Use [GitHub private vulnerability reporting](https://github.com/Lord-shaban/GhostMark/security/advisories/new). Include the affected commit/version, Windows build, a minimal reproduction, impact and a proposed fix if available. Do not open a public exploit report or include keys, credentials or personal device data.

The maintainer will triage reports as availability permits. There is no guaranteed response SLA or bounty program. Coordinated disclosure should wait until an agreed fix/disclosure plan exists.

## Security expectations

- Standard-user startup; elevation only where a proven operation requires it.
- Reject unknown Windows builds before system-changing operations.
- Minimize and document unsafe code and process/FFI boundaries.
- Keep restoration and partial-failure recovery independent of success-path UI.
- No collection of product keys, telemetry or licensing changes.
- No requirement to disable security software or add broad exclusions.
- Releases must state provenance, checksums, signing status and compatibility truthfully.

Read the planned [trust boundaries](docs/architecture.md) and [validation strategy](docs/testing.md). These are design requirements, not claims of an implemented security audit.
