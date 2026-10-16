# Windows 11 Pro 26300.9457 baseline

Status: INCOMPLETE. Media, guest tooling, standard-user read-only inspection and powered-off snapshot recovery are verified. Interactive display/activation observations remain incomplete. This is not hide/restore evidence or a supported-build claim.

## Source and isolation

- Initial baseline: 2026-10-15; continued on 2026-10-16 Africa/Cairo (2026-10-15 UTC). Observation timestamps use UTC.
- Source: Microsoft's [Windows 11 download page](https://www.microsoft.com/en-us/software-download/windows11), English x64 installation media, supplied by the operator.
- ISO size: 9,047,330,816 bytes.
- Microsoft-published and locally computed SHA-256: `BD4307DF32BC8AF33B39CCECB1174AEB345386630F89A2B86C7A4E36B55EA650`.
- VirtualBox media detection: Windows 11 Pro, image index 6, `10.0.26300.9457`, x64, en-US. This identifies the image; installed-guest identity still requires inspection.
- Hypervisor: Oracle VirtualBox 7.2.20 r175154, Windows host, NEM/Windows Hypervisor Platform execution backend.
- VM: initially 2, then 3, and currently 4 virtual CPUs; 4 GiB RAM, dynamically allocated 64 GiB SATA disk, EFI, Secure Boot enabled, virtual TPM 2.0, VBoxSVGA with 128 MiB video memory and 3D acceleration disabled.
- VM-scoped execution setting: `VBoxInternal/EM/ExitOptimizationEnabled=0`, following the [upstream maintainer's workaround](https://github.com/VirtualBox/virtualbox/issues/643). The local startup log confirms `enabled=false enabled-r0=false`; this changes a VirtualBox execution optimization, not a host or guest security setting.
- Isolation: guest network cable disconnected, shared clipboard and drag/drop disabled, no shared host folders or USB passthrough configured. Lab storage and credentials remain outside the repository.
- Installation uses a locally reviewed unattended template with VirtualBox's hardware-requirement bypass directives removed. Host/guest endpoint protections are not disabled. Separate VM-local setup and standard-user test accounts are present; the test token has medium integrity and no Administrators membership.
- Matching Guest Additions 7.2.20 r175154 are running for controlled guest execution/file transfer and display verification. They are lab tooling, not a GhostMark runtime dependency.

## Provisioning observations

| Observation | Result |
| --- | --- |
| ISO vendor hash and media metadata | Pass |
| Initial 2-vCPU EFI start | Failed to progress past `EFI: debug point DXE_AP`; screenshot dimensions remained 0 × 0 |
| Recovery from firmware stall | Powered off only the disposable VM and changed its CPU count to 3 |
| 3-vCPU boot | Reached the Windows 11 Setup interface; Secure Boot/TPM remained enabled |
| Slow installation | Setup remained at 11% while disk writes advanced slowly; a VM pause/resume did not establish a fix |
| Execution-setting change | Saved the VM state, applied the VM-scoped setting above and restored it; installation resumed and the virtual disk subsequently grew past 5 GB |
| Later installation stalls | Image and disk activity repeatedly stalled; temporary VM pause/resume allowed progress. The installation-only monitor was stopped before baseline validation. Four vCPUs were tried from a setup checkpoint; this is not evidence of stable guest operation |
| OOBE failure | The locally edited answer file supplied the invalid `NetworkLocation` value `Public`. Windows reported that installation could not complete. Removed the deprecated setting from the private template and generated answer; preserved the failure screenshot privately |
| Offline answer repair | Restored the setup checkpoint, booted Microsoft's WinPE and backed up the guest's cached `Windows/Panther/unattend.xml`. Copied the corrected answer from the repair DVD; `fc.exe /b` reported no differences against the source |
| Boot-order recovery | The installed Windows boot entry initially took precedence over the repair DVD. Saved the original UEFI order, selected the DVD temporarily, then restored and read back the original order byte-for-byte. Detached the repair medium; Windows subsequently reached its first desktop |
| Installed guest identity and permissions | Pass for guest-control inspection: Windows Professional 10.0.26300.9457, x64, en-US; standard-user medium-integrity token without Administrators membership. Interactive Settings confirmation remains pending |
| Activation notice and display settings | Pending: the earlier continuation reached the lock screen; the later observation below confirms an unlocked session, but does not establish the desktop notice or Settings values |
| Snapshot creation and restore | Pass: clean guest shutdown confirmed as `poweroff`; detached setup media and created `baseline-before-ghostmark` at 14:11:24 UTC. Restored while powered off at 14:14:29 UTC; post-restore marker, artifact, token and probe checks passed at 14:16:45 UTC |
| Read-only probe in guest | Pass: artifact hash matched; `inspect` exited 0, reported no warnings and correctly returned `unsupported / build_not_verified` |
| Watermark hiding/restoration | Not run |

The firmware symptoms match the upstream [VirtualBox 2-vCPU/WHPX report](https://github.com/VirtualBox/virtualbox/issues/799). The CPU-count workaround was observed locally; this does not establish a universal cause or fix. A successful `startvm` command alone does not establish successful firmware or Windows boot.

## Read-only guest evidence

The private lab artifact was built from source commit `975cf515b3678af84df039bad49c88ef37971c2e`, using Rust 1.99.0 with `-C target-feature=+crt-static`. Its guest-verified SHA-256 is `E238BDCFAC3CF443D2E870B8CD9746AFB77F982579B2C0E4FB29561D7D2543AD`. This is an unpublished diagnostic artifact, not a release.

At 14:07:48 UTC, the guest-control account's effective Administrator check and Administrators-membership check were both false. A separate `whoami /groups` observation confirmed medium integrity. The controller runs these processes in Session 0; this evidence does not substitute for inspection in the interactive desktop session.

Repeated guest inspection returned this sanitized report:

```json
{
  "schema_version": 1,
  "mutation_capable": false,
  "environment": {
    "platform": "windows",
    "version": {
      "major": 10,
      "minor": 0,
      "build": 26300,
      "revision": 9457,
      "product_type": 1
    },
    "native_architecture": "x86_64",
    "process_architecture": "x86_64",
    "edition": "Professional",
    "locale": "en-US",
    "system_dpi": 96,
    "monitor_count": 1,
    "remote_session": false,
    "warnings": []
  },
  "compatibility": {
    "status": "unsupported",
    "reason": "build_not_verified"
  }
}
```

The DPI value is the calling process's system-DPI context, not a verified display-scaling setting. Guest-control and PowerShell startup previously timed out; a later run completed, including first-use module preparation. This is not proof that the earlier VM stalls are resolved.

Raw evidence remains in the private lab directory outside the repository, alongside media/setup/VM evidence. It includes `verified-baseline-before.txt`, `baseline-probe-before.json`, `baseline-probe-after.json`, `token-before.txt`, `token-after.txt`, `artifact-hash-after.txt` and the private screenshots. The sanitized report above is the public evidence; no account names, device identifiers or credentials are included.

## Snapshot recovery evidence

After creating the powered-off snapshot, booted the guest and created an inert marker file in its lab directory; reading it back confirmed the marker existed at 14:13:42 UTC. Shut down Windows normally, confirmed `poweroff`, restored `baseline-before-ghostmark`, confirmed that it restored to `poweroff`, and booted again. At 14:16:45 UTC, checks confirmed that the marker was absent, the probe's artifact hash still matched, the before/after JSON reports were equal, and the test token still had medium integrity without Administrators membership. The marker's disappearance demonstrates rollback of guest disk state, rather than merely successful snapshot-command execution.

Both cold boots required time for Guest Additions to become ready; early execution requests were rejected, including a bounded 45-second readiness wait after restoration. Later verification succeeded without pause/resume or security changes. The private `rollback-marker-before.txt`, `rollback-marker-after.txt`, `snapshot-restore.json` and `snapshot-verification.json` retain the recovery observations. Snapshot identifiers remain private. This proves lab rollback, not restoration of a watermark mechanism.

## Remaining observations

At 21:39:19 UTC on 2026-10-15, a hypervisor framebuffer capture showed an unlocked guest session with Edge covering the desktop. The capture is retained privately as `baseline-continuation-20261016.png`. Its framebuffer size of 1024 × 768 is not a Settings-confirmed resolution or scaling measurement. A subsequent standard-user guest-control `inspect` completed successfully; its report matched `baseline-probe-after.json` exactly, including no warnings and `unsupported / build_not_verified`. The new raw report is retained as `baseline-probe-continuation-20261016.json`.

A separate PowerShell token/artifact recheck exceeded its 45-second guest-process limit and ended with `VERR_TIMEOUT`; it produced no new verification result. The earlier passing token/hash evidence above remains the last completed check. No pause/resume, security change or watermark operation was used to work around this failure.

The host UI-control helper returned an unrelated host-browser image when asked to capture the selected VirtualBox window, including after activation and a refreshed window inventory. No input was sent through that mismatched capture. This is a lab-control limitation, not evidence about the guest desktop or watermark behavior.

Follow [FEASIBILITY-PROTOCOL.md](FEASIBILITY-PROTOCOL.md) to confirm edition/build in guest Settings, record display resolution/scaling and monitor layout, inspect broad Activation Settings state, and observe the actual desktop-notice type with a sanitized screenshot. These interactive observations remain pending; an application covering the desktop does not establish whether the activation notice is present or absent. No activation state or system clock is changed to produce a target.

No watermark mutation has been attempted on either the guest or the host. No build is authorized by this preparation record.
