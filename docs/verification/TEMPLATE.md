# Verification: descriptive experiment name

Status: NOT RUN. This is a template, not evidence.

## Environment

- Date/time and timezone:
- Tested source commit:
- Windows edition / exact version and build / architecture:
- Activation notice type (separate from evaluation/test-mode):
- Locale, monitor configuration and DPI:
- Hardware/VM resources and graphics/renderer:
- Compiler/build profile:
- Permissions and runtime prerequisites:
- Clean-snapshot/recovery preparation (no sensitive identifiers):

## Protocol and observations

| Step | Expected behavior | Observed behavior | Pass / Fail / Not run |
| --- | --- | --- | --- |
| Baseline and read-only inspection | Exact environment; no changes | | Not run |
| Unsupported build | Refused before mutation | | Not run |
| Apply | Actual activation notice hidden; desktop usable | | Not run |
| Restore | Original behavior restored | | Not run |
| Repeated apply/restore | Defined, recoverable behavior | | Not run |
| Permission failure | Actionable error; no partial residue | | Not run |
| Interruption / partial failure | Recovery works | | Not run |
| Explorer restart | Documented lifecycle behavior | | Not run |
| Lock/unlock and reboot | Documented lifecycle behavior | | Not run |
| Monitor/DPI changes | Correct behavior or explicit limitation | | Not run |

## Evidence, recovery and conclusion

Link sanitized artifacts and raw measurement summaries. Record actual manual recovery steps, remaining limitations, whether activation state stayed unchanged, and any security-product observation without recommending disabled protections. State exactly what the experiment proves and what it does not. A supported-build claim requires passing hide and restore results.
