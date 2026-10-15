# Activation-watermark feasibility protocol

Status: protocol prepared; no hide/restore result is implied by this document.

Use this protocol for issues #2–#5. A candidate mechanism is not part of the read-only probe. Record every actual run using [TEMPLATE.md](TEMPLATE.md), and preserve failures as failures.

## 1. Provision an isolated baseline

Use a disposable Windows 11 x64 VM, with at least 2 virtual CPUs, 4 GiB RAM and a dynamically allocated 64 GiB disk. Record the actual configuration. Use official Microsoft installation media and record its language, edition, SHA-256, download source and advertised build. Do not redistribute the ISO or publish expiring download/session URLs. Verify its hash against Microsoft's published hash when available; a self-computed hash establishes identity, not vendor authenticity.

Use UEFI, Secure Boot and a virtual TPM 2.0 when required by Windows. Do not turn off host security, virtualization-based security or guest endpoint protections to make a test run. If virtualization cannot start without a host reboot or configuration change, record the blocker before requesting the precise necessary action.

Use a VM-local test account with credentials stored only in the private lab directory. Do not reuse the host's Microsoft account or credentials. Disable shared clipboard, drag/drop and USB passthrough. Avoid broad host-folder shares; copy only the required test binary into the guest through an explicit lab transfer. Use NAT only during documented setup when necessary, then disconnect networking for the offline experiment. Do not expose remote-control services to the LAN.

Install the matching Guest Additions only if required for controlled guest execution or display tests. Record the version and that dependency in the evidence. Keep the VM under the project's ignored/private lab directory outside the repository. A hypervisor-present flag alone does not establish that an isolated VM exists.

## 2. Capture the read-only baseline

Inside the guest, use a standard-user session and run:

```powershell
.\ghostmark-probe.exe inspect > baseline.json
```

Or build from the pinned source/toolchain:

```powershell
cargo run -p ghostmark-probe -- inspect
```

Record the source commit and artifact SHA-256. The JSON reports exact native OS major/minor/build and registry update revision, native/process architecture, edition, locale, system-DPI context, monitor count and remote-session flag. Optional read failures are reported as warnings. The probe never reads product keys, account names, machine identifiers or process listings; it does not detect the presence of a watermark or activation status.

Record actual display scaling manually from guest Settings, because the `GetDpiForSystem` value is affected by the calling process's DPI awareness and is not a complete display-scaling inventory. Record effective resolution and monitor layout separately. Confirm the edition/build from guest Settings and investigate any discrepancy before proceeding.

Visually identify **Activate Windows** and its settings reminder. Record the visible wording and a sanitized screenshot of the clean guest desktop. Evaluation, Insider/build and test-signing notices are separate categories. If the activation notice is absent, record `not observable`; do not deactivate a licensed installation, change its clock or forge a success result. A fresh lab installation's normal unactivated state is distinct from modifying an existing license state.

Inspect activation state using the guest's Activation Settings UI and record only the broad state, never keys or account details. The probe deliberately does not access licensing data. If the notice appears only after a delay, record the elapsed observation time. Lack of a visible target blocks the experiment; it does not prove that the tool hid it.

Take a powered-off snapshot named descriptively, for example `baseline-before-ghostmark`, after setup and before any experimental modification. Prove that the snapshot can be restored once, then repeat the baseline inspection. Keep the snapshot identifier private and use a sanitized name in public evidence.

## 3. Review the candidate before mutation

Document the candidate's exact target process/module, affected resource or bytes, build/architecture assumptions, needed privileges, original-state capture, timeouts, cleanup and recovery path. Pin upstream references to commits when studying them; audit source rather than running an unexplained binary. The initial [source review](../research/activation-candidates.md) rejects adopting two references directly and identifies a smaller reversible hypothesis to test.

Reject a candidate that relies on an unbounded memory scan, an ambiguous target, unchecked writes, permanently writable executable memory, arbitrary thread termination, disabled protections or unverifiable restoration. Do not infer that a short byte pattern or a matching class name identifies the correct function across builds. A shell restart may be a valid restoration boundary only if its behavior and user impact are demonstrated and disclosed.

At this stage `ghostmark-probe` cannot apply or restore anything. Do not invent a `hide` command in a test record. Add any experiment binary in the issue #4 work only after this protocol and baseline are in place; keep its execution confined to the guest.

## 4. Execute and record

| Scenario | Procedure | Pass condition |
| --- | --- | --- |
| Baseline | Inspect twice and capture the visible activation notice | Environment identity agrees; target is observable; no changes |
| Unsupported build | Exercise a sanitized unsupported fixture before live mutation | Refused before modifying anything |
| Successful apply | Apply once to the explicitly matched guest target | The actual notice disappears; desktop and ordinary apps remain usable |
| Verified restore | Restore through the candidate's documented recovery path | Original behavior returns; no residual file/registry/process state introduced by the tool |
| Repeat | Repeat apply/restore at least three times | Defined, stable results; no accumulating resources or unexplained processes |
| Access denied | Run without a candidate's required permission | Clear failure and unchanged original state |
| Partial operation | Inject a failure at every mutation boundary | Original state restored, or explicit recoverable failure with proven independent recovery |
| Interrupted controller | Close/terminate the experimental controller at documented boundaries | Desktop remains usable and independent recovery works |
| Explorer lifecycle | Restart Explorer within the guest before and after apply | Defined result; any reapplication requirement is disclosed |
| Session lifecycle | Lock/unlock, sign out/in and reboot the guest | Results match the documented lifetime; no hidden persistence |
| Display changes | Test 100/150/200 percent scaling, resolution and monitor changes | Correct result or explicit limitation; unsupported combinations are not advertised |
| Offline | Disconnect guest networking and repeat the supported flow | No network requirement for normal operation |
| Licensing unchanged | Compare broad Activation Settings state before and after | Visual behavior changes only |

Use a separate restored snapshot for fault scenarios when needed. Never terminate or restart the host's Explorer as part of these procedures. A screenshot must be paired with the operation result, target identity and restoration observation.

## 5. Stop and recover

On any unexpected desktop instability, wrong target, ambiguous identity or incomplete rollback: stop the experiment, retain the failure record and restore the clean guest snapshot. If the guest is unusable, power off only the named disposable VM and restore its snapshot through the hypervisor. Verify baseline behavior again. Do not widen permissions or disable security to get a green result.

The snapshot is the final lab recovery boundary, not a substitute for a usable product Restore function. A candidate that can only recover through a VM snapshot is not ready for M2.

## 6. Exit gate

Issue #2 needs an actual guest baseline plus the protocol and verified snapshot preparation. Issue #3 needs the read-only probe tests and guest execution. Issue #4 needs reproducible hiding **and restoration** of the activation notice, including partial-failure/lifecycle evidence. Issue #5 records go/no-go and the chosen operation contract in ADR-0002.

Absent media, a missing VM, an invisible activation notice or an unproven mechanism is a documented blocker. It is never converted into a supported-build row or a completed experiment. Only then can the engine, desktop interface and later release work advance according to the [roadmap](../roadmap.md).
