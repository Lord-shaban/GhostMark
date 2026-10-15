# Compatibility record

**No Windows build is currently verified or supported.** The first target is Windows 11 x64. Architecture/marketing-version support in Rust or the UI framework does not establish support for the watermark mechanism.

| OS edition and exact build | Architecture | Commit | Hide result | Restore result | Evidence | Support |
| --- | --- | --- | --- | --- | --- | --- |
| None tested yet | — | — | Not tested | Not tested | None | Unsupported |

Add a row only after the [verification protocol](testing.md) has been executed. Mark a build supported only if both hide and restore passed, lifecycle behavior is understood and sanitized evidence identifies the tested commit. Explicitly record any permissions, restart or graphics/runtime requirements.

Unknown/unverified builds must be rejected before mutation. Revalidate after Windows updates and mechanism changes; do not generalize a passing row to all Windows 11 installations. Evaluation, Insider and test-mode watermarks are separate cases and outside initial claims.
