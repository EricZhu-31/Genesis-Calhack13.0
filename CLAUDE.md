# Working on GENESIS

Read [README.md](README.md) first, then [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md)
for what is implemented versus proposed.

## Ground rules

- The Rust kernel (`genesis-core` + `genesis-server`) is the authority boundary.
  Organization changes go through the engine's validate/commit path; never
  write organization state around it.
- Keep claims honest: fixture/loopback tests, local processes and live provider
  runs are different kinds of evidence. Do not describe a fixture result as a
  live or measured result.
- Never replay a provider POST whose outcome is unknown; reconcile first.
- Keep credentials and raw provider logs out of Git (`.env` is ignored).
- `docs/invention-lab/evidence/**` is retained experiment data that tests read.
  Do not edit or regenerate it.

## Commands

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
(cd apps/web && npx tsc -b --noEmit && npx vitest run)
cargo run -p genesis-server --example rehearse_autonomy   # end-to-end A → B, fixtures
```

`[profile.dev]` uses line-table debug info only; full debug info pushes
`target/` to ~30 GB.

## History

The hackathon-era coordination log (PR queue, agent ownership, sponsor gates)
was removed from this file on the `slim/core` branch. It is in Git history.
