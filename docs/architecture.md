# Architecture and trust boundaries

Status: planned. Only a documentation-only `ghostmark-core` crate exists.

## Responsibilities

```text
Future desktop UI (egui/eframe, standard-user startup)
                    |
        explicit requests / operation results
                    |
       Core compatibility and operation state
                    |
    Narrow Windows adapter using windows-rs
                    |
   Approved operating-system change surface
```

The UI renders state and submits requests; it does not own unsafe pointer operations. Core logic handles compatibility decisions and valid operation transitions. The Windows adapter owns platform calls and their invariants. The adapter's physical crate/module layout is decided when the approved method needs it, not scaffolded speculatively.

A separate elevated helper, injected module or resident process is **not** assumed. Add one only if the M1 method requires it, and record its authentication, input validation, loading and recovery boundaries in ADR-0002. If the chosen implementation lives in another process, it must not depend on the lifetime of a UI allocation.

## Compatibility and results

Read exact OS build and architecture rather than trusting a marketing version or registry rumor. Unknown, unsupported, permission denied, failed and restored must remain distinguishable. The exact engine API/state machine is an M1 deliverable; the scaffold exports no pretend functions.

An operation must validate compatibility before mutation, preserve enough original state to recover, bound the work, and return a result that reflects observed behavior. Repeated operations and unexpected Explorer/session changes need explicit semantics. Restoration may require a shell/session transition; any requirement must be proven and disclosed rather than promised away.

## Trust boundaries

| Boundary | Required behavior |
| --- | --- |
| OS/foreign data into Rust | Validate lengths, types, lifetimes and build identity; document FFI invariants |
| Safe core into unsafe adapter | Keep unsafe small; own handles/resources and error cleanup explicitly |
| Standard user into elevated operation, if needed | Justify the boundary, restrict allowed requests and authenticate the caller |
| Disk into module/resource loading, if needed | Resolve trusted paths and prevent loading an attacker-controlled replacement |
| App into another process, if needed | Confirm target identity/architecture and compatibility; test partial failure/recovery |
| Diagnostics into public reports | Remove identifiers, keys, memory contents and personal paths |
| CI into release | Build reviewed source; publish checksums/provenance and actual signing status |

## Resource behavior

Use synchronous bounded core operations initially. If operations block the UI, run them on a small worker thread and return typed results. Do not add Tokio, a service manager, continuous polling or a plugin system without demonstrated need. Request repaint only when input or state changes warrant it. Select renderer features after testing actual graphics/VM prerequisites.

Normal operation is intended to remain offline with no telemetry. Prove that property in M4. Raw diagnostics and memory dumps are not public artifacts.
