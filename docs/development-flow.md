# Development Flow

This project uses OpenSpec for spec-driven development. `AGENTS.md` is the
authoritative contributor and agent guide; this file is a short checklist.

## One Change

1. Explore current specs and code before editing:
   - `openspec list --specs`
   - `openspec list`
   - read relevant files under `openspec/specs/`
2. Propose the change:
   - `openspec new change "<change-name>"`
   - write `proposal.md`, `design.md`, `tasks.md`, and delta specs
   - commit as `docs(<change-name>): propose <summary>`
3. Apply the change:
   - implement against `openspec/changes/<change-name>/specs/`
   - check off tasks only after code and tests pass
   - commit coherent compiling milestones as `feat(...)` or `fix(...)`
4. Sync verified semantics:
   - promote verified delta specs into `openspec/specs/`
   - commit as `docs(specs): sync <change-name>`
5. Archive the completed change, as a distinct gate from sync:
   - once verified, remove `openspec/changes/<change-name>/` directly
   - do not run `openspec archive` -- it recreates
     `openspec/changes/archive/`, which this project keeps empty except
     `.gitkeep`
   - commit as `chore(openspec): archive <change-name>`

## Commit Granularity

Apply commits should be larger than individual task checkboxes and smaller than
an entire risky feature. Prefer one commit per coherent milestone that builds,
tests, and preserves the spec contract.

Avoid:

- committing unrelated docs, refactors, and behavior together
- checking off `tasks.md` before the Definition of Done passes
- syncing `openspec/specs/` before implementation has been verified

## Definition Of Done

Run these from the workspace root:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets
cargo +1.88.0 check --workspace --all-targets
cargo semver-checks --package lengkap-contract --baseline-version 0.1.0
cargo semver-checks --package lengkap --baseline-version 0.1.0
cargo deny check
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

Rust 1.85 is the compatibility contract for the publishable product crates.
Rust 1.88 is the tooling floor for the complete repository, including the
unpublished governor. The semver reactions compare only those product crates
with their exact crates.io 0.1.0 baselines.

Release finalization is a separate change. Normal feature or repository-shape
changes do not publish crates, create tags, or create GitHub releases.
