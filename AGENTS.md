# AGENTS.md

Meta-guideline for any AI coding agent working in this repository. Read this
first.

## Source Of Truth And Workflow

This project uses OpenSpec. Current behavior lives in `openspec/specs/`; active
delta proposals live in `openspec/changes/`.

Follow:

```text
explore -> propose -> apply -> sync -> archive
```

- Do not write feature code without an active change containing tasks.
- Read relevant main specs and all active change artifacts before applying.
- Delta and main specs must contain comprehensive BDD-style success, failure,
  and edge scenarios before apply or archive.
- Sync merges verified delta specs into `openspec/specs/` and leaves the
  change directory active for verification. Archive is a distinct gate:
  once verified, remove the change directory directly instead of running
  `openspec archive`.
- Keep Markdown near 80 columns.
- Write OpenSpec artifacts, ADRs, code comments, and commits in English.
- Converse with users in the language they use.
- Every generated `tasks.md` ends with: "Update BACKLOG.md with the ✓ shipped
  status after archiving."

Use the OpenSpec CLI when no agent-specific command exists:

```bash
openspec list [--json] [--specs]
openspec new change "<name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>"
```

`openspec archive` is intentionally not part of this project's flow; it
recreates `openspec/changes/archive/`, which this repository keeps empty
except `.gitkeep`. Do not run it.

## Project Boundary

Lengkap is a `no_std + alloc`, sans-I/O all-of evidence completion core.
`PROJECT.md` is the orientation contract and OpenSpec is behavioral truth.

Protect these separations:

- `lengkap-contract` owns fixed slots, monotonic capture, deterministic
  completion, and structural input validation.
- Users own evidence truth, slot meaning, persistence, clocks, async work, I/O,
  scheduling, and reactions.
- `lengkap` is a complete, logic-free `pub use lengkap_contract::*;` facade.
- `lengkap-governance` is an unpublished independent judge.

Do not add Worklane types or job-queue vocabulary to the core. Worklane is an
originating pressure and intended bridge consumer, not an inward dependency.
Do not add serialization, async, storage, clocks, callbacks, quorum, any-of, or
dynamic slot growth without a concrete consumer and an OpenSpec change.

## Executable Governance

The canonical Tianheng constitution is
`crates/lengkap-governance/src/main.rs`. Its generated readable projection is
`docs/architecture/tianheng-law.md`; never edit the generated boundary body by
hand.

Run:

```bash
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

The gate observes direct dependencies, selected inline standard-library paths,
public async functions, selected ambient clock reads, and serde marker
acquisition. It does not prove every runtime effect or the semantic meaning of
user evidence. Repair code toward a violated reason; never weaken a law,
baseline new drift, or change severity merely to make a check green.

A deliberate law change requires explicit user authority, focused violating
and clean reaction proofs, projection regeneration, and adversarial review.

## API And Release Discipline

The decision and structural-error enums are intentionally exhaustive because
their variants define the finite outcome space. Generic values and causes carry
domain extensibility.

Removing, renaming, or semantically repurposing any public item is breaking.
Add public surface only for concrete product pressure documented through
OpenSpec. A real consumer remains a graduation test, not the sole source of
product authority.

Do not run `cargo publish`, create a release tag, or create a GitHub release
without a separately authorized release change. `CHANGELOG.md` is a strict
release ledger and therefore has no `[Unreleased]` section. Normal work is
recorded in OpenSpec, pull requests, and `BACKLOG.md`. A release preparation
pull request adds `## [X.Y.Z] - YYYY-MM-DD` and the footer link
`[X.Y.Z]: https://github.com/tacticaldoll/lengkap/releases/tag/vX.Y.Z`.

The public release set is `lengkap-contract` followed by the dependent
`lengkap` facade at the same version. Verify the complete workspace, publish
and verify the contract, then verify and publish the facade.
`lengkap-governance` remains unpublished. Tag only after both registry
artifacts and a fresh external consumer have been verified.

## Commits And Integration

- Branch from `main` and open every content change directly against `main`.
- Use Conventional Commits with an English lowercase imperative subject no
  longer than 72 characters.
- Give every pull request a non-empty rationale, decisions, compatibility, and
  verification body.
- Rebase on current `main`, verify, and squash-merge.
- The squash subject exactly matches the approved pull request title and its
  non-empty body is distilled from the pull request body.
- Do not append pull request numbers or URLs to squash subjects or bodies.
- Do not include AI, agent, model, tool, automation, or generation attribution.

## Definition Of Done

Run from the workspace root:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets
cargo +1.88.0 check --workspace --all-targets
cargo deny check
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

The Rust 1.85 gate is the public product contract. The Rust 1.88 full-workspace
gate covers repository-only governance tooling as a separate compatibility
surface.

Do not check a task off, sync, archive, or integrate while any required gate
fails.
