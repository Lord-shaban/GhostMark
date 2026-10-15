# Read-only feasibility probe

`ghostmark-probe` collects a limited environment report for the VM baseline. It does not hide or restore a watermark, detect activation, request elevation, change registry values, install services or persist after exit.

```powershell
cargo run -p ghostmark-probe -- inspect
```

The only operation is `inspect`, which writes JSON to standard output. `--help` describes its scope. Unknown arguments, including `hide`, `restore` and `--apply`, fail with exit code 2. A successfully written inspection exits 0 even if compatibility is unsupported/unknown; this exit code means inspection completed, not that modification is permitted. Output failure exits 1.

## Report and privacy

Schema version 1 contains `mutation_capable: false`, the environment and an assessment. The environment includes OS major/minor/build/update revision, desktop/server type, native/process architecture, edition, locale, system-DPI context, monitor count, remote-session flag and sanitized read warnings. It does not read account names, product keys, machine names, hardware identifiers, file paths or application window titles. It performs no network operation.

`RtlGetVersion` obtains the native OS version without relying on version-helper manifest compatibility. `RegGetValueW` reads only `UBR` and `EditionID` from the Windows CurrentVersion key. Other narrowly enabled Win32 APIs read architecture, locale and display/session metrics. Native buffers have fixed capacities and explicit byte sizes, and invalid/missing fields are not silently treated as trusted identity.

The system-DPI value follows `GetDpiForSystem` and depends on process DPI awareness; it is **not** a per-monitor scaling measurement. The baseline protocol also records guest display settings manually. The probe does not determine whether it runs in a disposable VM: that must be established by the test operator/hypervisor record. A hypervisor flag would be insufficient.

## Compatibility policy

- Non-Windows, older desktop Windows, servers and known non-x64 native/process architectures are unsupported.
- Missing/malformed version, edition, revision or architecture yields unknown where eligibility cannot be determined.
- A complete Windows 11 x64 identity still requires an exact build/revision and edition match against accepted verification evidence.
- **The production verified-build list is currently empty. No real installation can receive `supported`.** Synthetic support fixtures only exercise the pure matching rule; they are not compatibility evidence.

This assessment is a conservative planning guard, not a tested permission model for a future mechanism. ADR-0002 must refine any operation-specific invariants before mutation is introduced.

## Validation

Pure tests cover exact-revision/edition matching, an unverified fixture, missing/malformed identities, servers/older clients and architecture/emulation rejection. CLI integration tests execute the real read-only probe on Windows/Linux CI, validate JSON and reject system-changing commands. These tests do not prove watermark feasibility. Follow the [VM protocol](verification/FEASIBILITY-PROTOCOL.md) for that evidence.
