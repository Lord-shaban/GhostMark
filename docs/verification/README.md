# Verification records

No Windows mechanism evidence exists yet. Follow [FEASIBILITY-PROTOCOL.md](FEASIBILITY-PROTOCOL.md), copy [TEMPLATE.md](TEMPLATE.md) into a descriptively named record when executing a real experiment, and link it from its issue/PR and the compatibility table.

The [Windows 11 Pro 26300.9457 baseline](windows-26300-baseline.md) currently records verified media and VM preparation only. Guest inspection and snapshot recovery remain pending; the record is explicitly incomplete.

Include the tested commit, exact build, protocol, observed failures, restoration and evidence. Do not commit raw dumps, keys or device/user identifiers. Publish sanitized media only when necessary to substantiate a result; keep raw data under ignored local-evidence/.

An incomplete or failed record must remain labeled incomplete or failed. A record's existence is not itself proof of support.
