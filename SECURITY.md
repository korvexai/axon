# SECURITY POLICY

## Reporting

Do NOT open public issues for vulnerabilities.

Report security issues via email to the maintainer with:

- Description
- Steps to reproduce
- Impact assessment

Initial response target: within 72 hours.

---

## Security Model

AXON:

- Runs locally
- Has no telemetry
- Has no auto-update mechanism
- Does not escalate privileges implicitly
- Does not sandbox OS-level execution

AXON may execute arbitrary shell commands if configured to do so.

Users must review execution behavior carefully before enabling automation.

---

Software is provided AS IS, without warranty.