| Layer                   | Purpose            | Must-Have Features                                                             |
| ----------------------- | ------------------ | ------------------------------------------------------------------------------ |
| Version Control         | Source of truth    | Branching model (trunk, GitFlow), PR/MR gates, required reviews, CODEOWNERS    |
| Triggers                | When pipelines run | Push/PR/MR, tag, schedule, manual, API, path filters                           |
| Runners/Agents          | Where jobs execute | Ephemeral containers, autoscaling, isolation, secrets mount, caching (sccache) |
| Build System            | Compile & package  | Matrix builds, incremental cache, reproducible builds, artifacts export        |
| Test System             | Prove correctness  | Unit/integration/e2e, flaky test quarantine, retry rules, coverage gates       |
| Quality Gates           | Block bad changes  | Lint/format, clippy, security scan, coverage threshold, conventional commits   |
| Dependency/Supply Chain | Trust inputs       | Lockfiles, SCA (vuln scan), SBOM, license scan, provenance attestations        |
| Secrets & Config        | Keep secrets safe  | OIDC → cloud KMS, sealed secrets, short-lived tokens, env segregation          |
| Artifacts/Registry      | Store outputs      | Artifact retention, container registry, immutability, garbage collection       |
| Release Mgmt            | Version + notes    | SemVer or CalVer, changelog, signed tags/releases, release branches            |
| Deploy Orchestrator     | Ship to envs       | Helm/Kustomize, GitOps (ArgoCD), strategy (blue/green/canary/rolling)          |
| Verification            | Prove it works     | Smoke/synthetic checks, health/readiness gates, canary analysis                |
| Observability           | See everything     | Build logs, test reports, metrics, traces, deploy timelines, SLO gates         |
| Policy & Compliance     | Enforce rules      | Policy-as-code (OPA/Conftest), approvals, SoD, audit logs, SLSA targets        |
| Rollback & DR           | Safety net         | Fast rollback, config/DB migration reversibility, backups, runbooks            |
