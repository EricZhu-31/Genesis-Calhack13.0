# GENESIS

**Networked capability improvement with fixed model weights.**

GENESIS is infrastructure through which independent AI organizations create,
verify, share and improve executable capabilities. What evolves is the system
around the model — tools, workflows, specialist roles and development methods —
while the foundation model's weights stay fixed.

Each organization is durable, governed state: roles, responsibilities,
authority, budgets and contracts. A planner proposes structural changes; the
Rust kernel validates them. Capabilities keep exact versions, provenance and
tested scope, and a receiving organization decides locally whether to adopt,
adapt or decline them.

> This repository started as a Cal Hacks 13.0 project. The hackathon-only
> parts (the event/venue mission demo, sponsor proof kits, the Fetch.ai
> bridge and the stage presenter) were removed on the `slim/core` branch;
> they remain in Git history before that branch.

## The loop

1. **Discover** — an agent hits a requirement and searches for a capability that fits.
2. **Organize** — if none fits, the organization assigns responsibility and recruits a specialist.
3. **Build and verify** — the specialist produces an executable candidate; independent checks establish its behavior, identity, permissions and limits.
4. **Publish and inherit** — the verified version enters a shared registry; another organization runs its *own* acceptance checks before adopting it.
5. **Repair and build further** — failures lead to bounded, evidence-backed repairs.

## What works today

| Mechanism | Status |
| --- | --- |
| Organizational evolution | Kernel supports role creation/retirement, recruitment, responsibility assignment/delegation and capability attachment, with authority and lifecycle checks before commit. |
| Executable capability creation | The forge synthesizes TypeScript extractors with a model, packages them as Apify Actors, pins builds/runs and verifies outputs against an independently configured suite, with one bounded semantic repair. |
| Cross-organization inheritance | Immutable versioned manifests, registry discovery, adopt-before-acquire and organization-local acceptance. Org B can install Org A's capability without synthesizing its own copy. |
| Development methods (invention lab) | Method-source generation, exact-parent revision, pre-result rosters, cost accounting, receiver-bound adoption and invocation records. Fixture-tested; no measured recursive benefit yet. |

**Not yet demonstrated:** a general executable development-method package, an
accepted A → B → A method-improvement loop, or measured recursive benefit over
an equal-budget baseline. See [implementation status](docs/IMPLEMENTATION_STATUS.md).

**Known narrowness:** the only executable capability type is the web extractor
in `actors/venue-extractor-template`, whose output contract is still the
venue-shaped schema from the hackathon (`venue_id`, `capacity`, `price`, …).
The forge, verification crate and acceptance suites are built around it.
Generalizing that contract is the main prerequisite for other domains.

## Quick start

Requirements: Rust 1.94+, Node 22+ (Node 24 for the Actor template's own tests).

```sh
# Two organizations + registry, fixture providers, full A → B inheritance
cargo run -p genesis-server --example rehearse_autonomy

# Or an interactive dev stack: registry :7100, Org A :7101, Org B :7102
cargo run -p genesis-server -- dev
cd apps/web && npm ci && npm run dev          # console on :5173
node scripts/dev-scenario.mjs --delay 1200    # optional scripted mission
```

The dev stack prints its development bearer tokens (`org_a-operator`, …). The
HTTP API lives under `/api/v1` on each organization and `/registry/v1` on the
registry; every route requires a bearer token.

### Live configuration

| What | Environment |
| --- | --- |
| Capability synthesis (forge / server acquisition) | `ANTHROPIC_API_KEY` |
| Building and running Actors | `APIFY_TOKEN` (billable) |
| Planner model | `GENESIS_LLM_PROVIDER` = `anthropic` \| `openai` \| `gemini` \| `openai-compatible` \| `elastic`, plus `GENESIS_LLM_MODEL`, `GENESIS_LLM_API_KEY`, optional `GENESIS_LLM_BASE_URL` |
| Optional: CREAO specialists | `CREAO_TRIGGER_API_KEY` and friends — see [CREAO runtime](docs/CREAO_RUNTIME.md) |
| Optional: Elastic projection / Agent Builder | `ELASTICSEARCH_URL`, `ELASTIC_DATA_API_KEY`, `ELASTIC_KIBANA_URL`, `ELASTIC_MANAGEMENT_API_KEY` — see [Elastic](docs/ELASTIC.md) |

Optional integrations stay off unless their variables are set.

## Repository layout

| Path | Contents |
| --- | --- |
| `crates/genesis-core` | Pure organization model, engine, lifecycle and development-method types |
| `crates/genesis-server` | Organization kernel and registry: Axum API, SQLite store, sagas, outbox, acquisition/adoption runtimes |
| `crates/genesis-forge` | Capability synthesis/acquisition and method generation/accounting CLIs |
| `crates/genesis-verification` | Independent output verification against trusted suites |
| `crates/genesis-apify` | Apify client (builds, runs, datasets) |
| `crates/genesis-providers` | Provider adapters (Apify executor, CREAO specialists, dev stub) |
| `crates/genesis-planner`, `genesis-planner-eval` | LLM mission planner and its replay evaluation |
| `crates/genesis-creao`, `genesis-elastic` | Optional integrations |
| `actors/venue-extractor-template` | Scaffold the forge fills to build Actors |
| `apps/web` | React operator console, including the invention-lab views |
| `experiments/`, `scripts/iteration` | Invention-lab executor/evaluator, scheduling and job-shop studies, iteration engine |
| `docs/` | Design and runtime guides; `docs/invention-lab/evidence` holds retained experiment records used as test fixtures |

## Checks

```sh
cargo test --workspace                 # all Rust crates
(cd apps/web && npm ci && npx tsc -b --noEmit && npx vitest run)
bash scripts/check-capability-core.sh  # fmt/clippy/tests for the capability core
```

Tests use loopback provider fixtures; none make paid calls. A few `#[ignore]`d
tests need retained rehearsal directories or real credentials.

## Further reading

- [Technical specification](docs/TECH_SPEC.md) and [research report](GENESIS-organization-evolution-research.md)
- [Recursive capability improvement goal](docs/RECURSIVE_CAPABILITY_IMPROVEMENT.md) and [invention-lab demo design](docs/INVENTION_LAB_DEMO.md)
- [Capability execution](docs/CAPABILITY_EXECUTION.md), [synthesis](docs/CAPABILITY_SYNTHESIS.md), [acquisition](docs/CAPABILITY_ACQUISITION.md), [server acquisition](docs/SERVER_ACQUISITION.md), [adopt-before-acquire](docs/CAPABILITY_GAP_ADOPTION.md)
- [Planner](docs/PLANNER.md), [provider contracts](docs/PROVIDER_CONTRACTS.md), [development setup](docs/DEV.md)
