# Activation-notice candidate review

Status: source review only, 2026-10-15. No candidate has passed the [VM protocol](../verification/FEASIBILITY-PROTOCOL.md); no upstream executable was run and no implementation was copied.

## Memory patch / thread termination reference

Reviewed [no_watermark at 4c9b9a6](https://github.com/a7t0fwa7/no_watermark/tree/4c9b9a6e1104a5918b18466bf22c3f4fdde508e4). The source writes a fixed instruction sequence and terminates threads selected by a byte near their start address. Its scan helper returns during its first loop iteration, and it provides no restoration operation. These properties do not establish correct targeting, safe thread ownership or recoverability. Do not adopt it as a production mechanism or infer current-build support from its repository description.

## Worker-window reference

Reviewed [the pinned RemoveNag script](https://gist.github.com/cw2k/26e1818303e0061e38d6a612ac52b1d5/a394883335a79d74bc36d6d390b97b829972ef86). Besides hiding a window, it sends destruction/close messages and suspends a thread; its own notes describe a recreation/polling problem. Those steps are not adopted. No source code is copied into GhostMark.

## Smallest hypothesis to investigate

Investigate reversible visibility control of a positively identified activation-notice window before considering memory changes. Establish identity from observed guest evidence, reject ambiguous targets, preserve original visibility and verify restoration/lifecycle behavior. This is a hypothesis, not an implemented method or compatibility claim. If it does not work reliably, record failure and revisit the decision rather than broadening the target or killing threads.
