# Windows 11 Pro 26300.9457 baseline

Status: INCOMPLETE. Media and VM preparation are verified; guest baseline and snapshot recovery are not yet recorded. This is not hide/restore evidence or a supported-build claim.

## Source and isolation

- Date: 2026-10-15; observation timestamps use UTC.
- Source: Microsoft's [Windows 11 download page](https://www.microsoft.com/en-us/software-download/windows11), English x64 installation media, supplied by the operator.
- ISO size: 9,047,330,816 bytes.
- Microsoft-published and locally computed SHA-256: `BD4307DF32BC8AF33B39CCECB1174AEB345386630F89A2B86C7A4E36B55EA650`.
- VirtualBox media detection: Windows 11 Pro, image index 6, `10.0.26300.9457`, x64, en-US. This identifies the image; installed-guest identity still requires inspection.
- Hypervisor: Oracle VirtualBox 7.2.20 r175154, Windows host, NEM/Windows Hypervisor Platform execution backend.
- VM: 3 virtual CPUs, 4 GiB RAM, dynamically allocated 64 GiB SATA disk, EFI, Secure Boot enabled, virtual TPM 2.0, VBoxSVGA with 128 MiB video memory and 3D acceleration disabled.
- VM-scoped execution setting: `VBoxInternal/EM/ExitOptimizationEnabled=0`, following the [upstream maintainer's workaround](https://github.com/VirtualBox/virtualbox/issues/643). The local startup log confirms `enabled=false enabled-r0=false`; this changes a VirtualBox execution optimization, not a host or guest security setting.
- Isolation: guest network cable disconnected, shared clipboard and drag/drop disabled, no shared host folders or USB passthrough configured. Lab storage and credentials remain outside the repository.
- Installation uses a locally reviewed unattended template with VirtualBox's hardware-requirement bypass directives removed. Host/guest endpoint protections are not disabled. Separate VM-local setup and standard-user test accounts are prepared; effective post-install permissions remain to be checked.
- Matching Guest Additions 7.2.20 are included for controlled guest execution/file transfer and display verification. Installation success remains to be checked; they are lab tooling, not a GhostMark runtime dependency.

## Provisioning observations

| Observation | Result |
| --- | --- |
| ISO vendor hash and media metadata | Pass |
| Initial 2-vCPU EFI start | Failed to progress past `EFI: debug point DXE_AP`; screenshot dimensions remained 0 × 0 |
| Recovery from firmware stall | Powered off only the disposable VM and changed its CPU count to 3 |
| 3-vCPU boot | Reached the Windows 11 Setup interface; Secure Boot/TPM remained enabled |
| Slow installation | Setup remained at 11% while disk writes advanced slowly; a VM pause/resume did not establish a fix |
| Execution-setting change | Saved the VM state, applied the VM-scoped setting above and restored it; installation resumed and the virtual disk subsequently grew past 5 GB |
| Installed guest identity, activation notice and permissions | Not run |
| Snapshot creation and restore | Not run |
| Read-only probe in guest | Not run |
| Watermark hiding/restoration | Not run |

The firmware symptoms match the upstream [VirtualBox 2-vCPU/WHPX report](https://github.com/VirtualBox/virtualbox/issues/799). The CPU-count workaround was observed locally; this does not establish a universal cause or fix. A successful `startvm` command alone does not establish successful firmware or Windows boot.

## Pending baseline

Follow [FEASIBILITY-PROTOCOL.md](FEASIBILITY-PROTOCOL.md) to record the installed build/edition, locale, display/scaling, effective token, broad activation state and actual notice type. Record a powered-off snapshot, prove its restoration, and repeat read-only inspection. No activation state or system clock is changed to produce a target.

No watermark mutation has been attempted on either the guest or the host. No build is authorized by this preparation record.
