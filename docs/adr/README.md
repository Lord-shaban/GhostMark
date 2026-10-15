# Architecture decision records

Use a short numbered Markdown file when a consequential decision needs durable rationale: stack, mechanism, privilege boundaries, persistence or compatibility policy. Routine refactoring does not require an ADR.

Each record contains Status, Context, Decision, Alternatives, Consequences and Evidence. Accepted records can be superseded by a later numbered record; preserve the history.

| Record | Decision | Status |
| --- | --- | --- |
| [0001](0001-rust-stack.md) | Rust, Windows bindings and egui/eframe | Accepted for planning |

ADR-0002 will select or reject the watermark mechanism after the M1 experiment. Do not write it as accepted before the evidence exists.
