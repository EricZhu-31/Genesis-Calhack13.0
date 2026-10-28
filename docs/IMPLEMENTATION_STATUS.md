# Implementation status

Snapshot for the `slim/core` branch (October 2026). The previous version of
this file was a PR-by-PR review ledger from the hackathon; it is in Git history.

## Implemented

| Area | Where | Evidence boundary |
| --- | --- | --- |
| Organizational authority and persistence | `genesis-core/src/engine.rs`, `genesis-server/src/{store,saga,outbox}.rs` | Roles, responsibilities, assignment epochs, leases, capability attachment, retirement, event history and durable effect journals. Invariant, integration and fault-injection tests. Dynamic structure alone does not show better problem solving. |
| Generated task capabilities | `genesis-forge/src/{synthesis,acquisition}.rs`, `genesis-server/src/capability_acquisition_runtime.rs` | Requirement/example synthesis, exact source/build binding, trusted-suite verification and one bounded repair. Specialized to TypeScript web extraction on Apify with the venue-shaped output contract. Loopback-tested. |
| Publication and independent inheritance | `genesis-server/src/{registry,capability_adoption,capability_gap_adoption}.rs` | Registry publication/discovery, version pins, receiver-local acceptance, use after the original specialist retires. `tests/two_org_inheritance.rs` and `examples/rehearse_autonomy.rs`. |
| Invocation evidence | `genesis-server/src/capability_invocation.rs` | Reconstructs invocation evidence against exact artifact, input, suite and historical authority. |
| Planner | `genesis-planner` | Anthropic, OpenAI, Gemini, OpenAI-compatible or Elastic Agent Builder models; replay evaluation in `genesis-planner-eval`. |
| Development-method foundations | `genesis-core/src/development_*.rs`, `genesis-forge/src/method_*.rs`, `genesis-server/src/method_*.rs`, `experiments/invention-lab` | Method-source generation and revision, pre-result rosters, admission and cost accounting, receiver-bound adoption, invocation authorization and run records. Retained live attempts are in `docs/invention-lab/evidence`; all retained comparisons returned Rejected. |
| Optional integrations | `genesis-creao`, `genesis-elastic` | CREAO API-Trigger specialists with gateway readiness; Elastic projection, evidence search and Agent Builder specialists. Off unless configured; tested with scripted providers. |
| Operator console | `apps/web` | Organization, audit, capability-gap and invention-lab views. |

## Removed on `slim/core`

Event/venue mission features (assessments, monitors, Fetch recovery and
intake), the two-organization scenario driver and CLI, the Fetch.ai crate and
Python bridge, the stage presenter, sponsor live-gate kits, and hackathon
coordination documents.

## Not implemented or demonstrated

- A generic executable development-method package and executor with exact
  dependencies, bounded effects and independent acceptance.
- A capability output contract beyond the venue-shaped web extractor.
- An accepted reciprocal A → B → A method-improvement loop.
- Measured benefit from method inheritance over an equal-budget baseline,
  including failures, inappropriate transfer and creation/validation cost.
- Authenticated network repair advisories, method deprecation and rollback.
- Production hardening: bearer tokens are static and compared as plain strings,
  `?token=` is accepted for event streams, and each organization is a single
  process over SQLite.
