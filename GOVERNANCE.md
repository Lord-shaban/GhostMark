# Governance

## Ownership and decisions

The initial maintainer is [Ahmed Sha’ban (@Lord-shaban)](https://github.com/Lord-shaban). Contributors may propose changes through Issues and pull requests. The maintainer is responsible for scope, review, compatibility claims and releases.

Routine implementation choices stay in the issue/PR. A change to the stack, privilege model, watermark mechanism, persistence behavior or compatibility policy needs a short [architecture decision record](docs/adr/README.md) with evidence and tradeoffs. A failed feasibility experiment must be recorded honestly and may stop or redirect the project.

## Work and review

GitHub Issues hold acceptance criteria and blockers. The public [Project](https://github.com/users/Lord-shaban/projects/11) holds live workflow state. Milestones represent exit gates rather than calendar promises. The [roadmap](docs/roadmap.md) describes their sequence without duplicating live task status.

All normal changes use pull requests and required CI. There is no standing administrative bypass. Zero mandatory independent approvals avoids deadlocking a solo-maintained project; it does not waive review of the diff, evidence or resolved discussions. Increase that requirement when the maintainer group grows.

Only maintainers publish releases. A release must satisfy [the release checklist](docs/releasing.md). Nobody may mark an untested Windows build as supported or publish a working-product claim unsupported by evidence.

## Community decisions

Discuss disagreements on the issue or PR and explain concrete tradeoffs. The maintainer makes the final call after considering input. Conduct concerns are handled privately through the contact route in the [Code of Conduct](CODE_OF_CONDUCT.md).
