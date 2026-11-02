# Security Policy

## Supported Versions

| Version | Supported          |
| :--- | :--- |
| `0.1.x` | :white_check_mark: |

---

## Reporting a Vulnerability

The Escrow-FairLance team takes the security of smart contracts, financial custody, and escrow funds with the utmost seriousness.

If you believe you have discovered a vulnerability in:
- Escrow accounting invariants
- Arbitrator consensus mathematics or fee splitting
- Token transfers or re-entrancy
- Access control, authorization, or administrator privilege abuse

**Please do not open a public GitHub issue.** Instead, submit your findings responsibly:

1. **Email**: Send detailed reproduction steps and impact analysis to `security@fairlance.io`.
2. **Encrypted Submission**: If sensitive, encrypt using PGP key `4A8F 9C21 8E34 B501`.
3. **Response Timeline**:
   - Initial confirmation: within **24 hours**.
   - Triage and severity assessment: within **72 hours**.
   - Coordinated disclosure & patch deployment: within **14 days**.

---

## Bug Bounty Guidelines

Eligible vulnerabilities reportable under responsible disclosure:
- **Critical**: Unauthorized drainage of escrowed funds, bypass of client/freelancer authorization.
- **High**: Permanent locking of escrow funds, manipulation of arbitrator majority vote consensus.
- **Medium**: Denial of service on milestone approval or dispute opening.

We thank security researchers for helping keep decentralized freelance escrow safe and reliable.
