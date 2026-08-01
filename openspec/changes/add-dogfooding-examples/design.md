## Context

`crates/lengkap-contract/examples/` currently holds two examples
(`worklane_fan_in`, `checkpoint_restore`), both referenced by name from
`README.md`. They establish the pattern this change follows: no
dev-dependencies, a domain modeled with locally-defined enums (never a real
domain crate), and assertions that double as the example's proof. Three
behavioral promises made in prose (`PROJECT.md`, `evidence-completion-core`
spec) have no example: recoverability after `Impossible`, domain-neutral
naming outside queue vocabulary, and structural-error recovery.

## Goals / Non-Goals

**Goals:**
- Add one example per undemonstrated behavior, each a self-contained `fn
  main()` with no new dependency.
- Keep every example's domain vocabulary local to that file, matching the
  existing two examples.
- Make each example's claim traceable to the spec/doc line it demonstrates.

**Non-Goals:**
- No change to `lengkap-contract`/`lengkap` public API, `Cargo.toml`
  dependencies, or `lengkap-governance` boundaries.
- No delta spec against `evidence-completion-core` or `curated-facade`: this
  change adds documentation artifacts, not new or modified contract
  behavior.
- Not a claim that any example constitutes graduation evidence or consumer
  pressure (`PROJECT.md`'s Tier 1 condition stays scoped to a real consumer).

## Decisions

- **One capability, `example-documentation`, instead of amending existing
  specs.** The three examples don't change what the core does, only what is
  demonstrated about it, so a new capability describing the
  examples-as-documentation contract is cleaner than a `MODIFIED` delta on
  `evidence-completion-core` that would otherwise carry no behavior change.
- **`readiness_gate.rs` models a fixed three-subsystem startup gate (config,
  database, cache), not a general "readiness" abstraction.** This keeps the
  example itself domain-neutral evidence without reading as a proposal to
  widen the core's vocabulary, and avoids brushing against the
  quorum/any-of/weighted non-goals in `BACKLOG.md` — it stays a strict
  fixed all-of case.
- **`impossible_then_recovered.rs` states the retry decision as caller
  policy in its doc comment**, mirroring `PROJECT.md`'s User Obligations
  wording ("whether contradictory evidence should be audited or rejected"),
  so the example cannot be misread as the core offering a retry feature.
- **Verification reuses the existing Definition of Done** rather than adding
  a new CI job: `cargo build --workspace` / `cargo clippy --workspace
  --all-targets -- -D warnings` / `cargo fmt --all --check` already cover
  `--all-targets`, which includes examples. `cargo run --example <name> -p
  lengkap-contract` is the manual spot-check during apply.
- **README additions are appended after the two existing example
  references**, same one-paragraph style, no new section heading.

## Risks / Trade-offs

- [Readiness-gate example reads as expanding the core's scope] → Mitigated
  by keeping the "config/database/cache" vocabulary entirely inside the
  example file; README wording stays limited to "demonstrates the naming
  outside queue vocabulary."
- [Impossible-recovery example reads as the core supporting retries] →
  Mitigated by an explicit doc comment attributing the decision to caller
  policy, not core behavior.
- [New capability spec has no code to enforce it] → Accepted: like the
  existing two examples, enforcement is `cargo run --example` succeeding and
  human review, not an automated contract check; this matches how
  documentation-level capabilities are already handled in this repo.
