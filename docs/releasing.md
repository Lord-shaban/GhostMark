# Release process

No release, tag or downloadable application is published during foundation work. Version `0.0.0` is an internal scaffold marker, not an application release. Packaging automation belongs to issue #14 when a real artifact and its prerequisites exist.

## Preview gate

- [ ] M1 go/no-go ADR accepted with reproducible hide/restore evidence.
- [ ] Engine, recovery and desktop gates pass.
- [ ] Required CI passes on the exact candidate commit from protected main.
- [ ] Every advertised build has passing [compatibility evidence](compatibility.md).
- [ ] No unresolved critical recovery, desktop-stability or security defect.
- [ ] Startup, idle resource use, size and offline behavior are measured.
- [ ] Keyboard, DPI and accessibility checks completed; limitations disclosed.
- [ ] Install/use/restore and independent manual recovery instructions verified.
- [ ] Dependency licenses reviewed; MIT license and notices included in the package.
- [ ] Portable archive launches on a clean supported VM without developer tools.
- [ ] SHA-256 checksums and source commit are published; provenance is generated/verified by the packaging workflow.
- [ ] Actual signing status and runtime prerequisites disclosed. An unsigned build is described as unsigned; no guarantee about SmartScreen or antivirus outcomes.

## Publishing

1. Bump the workspace version in a focused PR and update Cargo.lock and release notes.
2. Review the diff, run all gates and merge through protected main.
3. Tag the verified commit `v0.1.0-alpha.1` for the first preview. Never create a product tag from a foundation-only commit.
4. Build the actual application in a least-privilege workflow, giving release-write permissions only to the publishing job.
5. Smoke-test the artifact, verify archive contents/checksums/provenance, and publish a GitHub **prerelease** with exact build support and known limitations.
6. Open issues for feedback and link them to the Project. Repeat validation before promoting to `v0.1.0`.

Use SemVer from the first preview. Do not overwrite published tags/artifacts silently. If a release is defective, publish an advisory/notice and a new version; document recovery and stop recommending affected builds. A nonfunctional or unsafe candidate remains unpublished.
