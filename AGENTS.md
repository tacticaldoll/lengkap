# AGENTS.md

Meta-guideline for AI coding agents and contributors working in this repository. Read this first,
then let `openspec/specs/` and active change specs be the source of durable architecture truth.

## Lengkap In One Sentence

Lengkap is a `no_std + alloc`, sans-I/O all-of evidence completion core.

The core owns completion mechanics and nothing else. Users own what a slot means, whether evidence
is true, how pending state is stored, when to poll, and what effect follows a decision.

## Architectural Axioms

Before proposing or writing code, protect these axioms:

1. **The contract owns completion mechanics**: `lengkap-contract` owns fixed slots, monotonic
   capture, deterministic completion, and structural input validation.
2. **Users own everything else**: users own evidence truth, slot meaning, persistence, clocks,
   async work, I/O, scheduling, and reactions.
3. **The facade holds no logic**: `lengkap` is a complete, logic-free
   `pub use lengkap_contract::*;` facade.
4. **The governor is independent**: `lengkap-governance` is an unpublished independent judge.
5. **No inward consumer vocabulary**: do not add Worklane types or job-queue vocabulary to the
   core. Worklane is an originating pressure and intended bridge consumer, not an inward
   dependency.
6. **No speculative mechanism**: do not add serialization, async, storage, clocks, callbacks,
   quorum, any-of, or dynamic slot growth without a concrete consumer and an OpenSpec change.

## Lineage

```text
   tianheng  +  〔sans-I/O · OpenSpec · vocabulary-as-governance · least-commitment〕
                    │  inherited discipline — provenance, not coupling
                    ▼
             ●  lengkap

   siblings: ▢ ▢ ▢   intentionally blank — this repo is sibling-blind. Which
                     products compose together is a consumer app's knowledge, never
                     a component's; naming a sibling here would leak that knowledge
                     and rot when the roster changes.
   note: skeleton from tacticaldoll/rust-family-template.
```

Lengkap shares a **discipline** with its lineage, not code: its own crates, specs, constitution, and
release cadence. It does not import, track, or depend on any sibling product, and its governed
prose (`PROJECT.md`, `AGENTS.md`, `BACKLOG.md`, specs, and code comments) names none.

## Document Authority

- `openspec/specs/` is shipped architecture truth.
- `openspec/changes/` contains active proposed truth until it is synced.
- `PROJECT.md` states product vision, positioning, and non-goals.
- `docs/domain-language.md` is the canonical vocabulary.
- `BACKLOG.md` records settled and deferred decisions, open design questions, and candidate
  patterns, not mandatory phases.
- `AGENTS.md` is operating protocol for agents and contributors.
- `AGENTS.lengkap-law.md` is the generated, freshness-gated projection of the accepted Rust
  constitution in `crates/lengkap-governance`. The constitution is authoritative; read the
  projection after this file, regenerate it with its documented command, and never edit it by
  hand.
- Other files under `docs/` elaborate one topic each and yield to the documents above.

Decision provenance lives in git — the commit body and pull request that made a change record its
rationale. Forward-looking or reversed decisions are noted in `BACKLOG.md`. There is no separate
architecture-decision-record file class; the living documents above are the single source of
truth for current state, and git is the source of truth for why it changed.

If these documents conflict, fix the conflict through an OpenSpec change before implementing
feature code.

## Adversarial Review Stance

Every change passes an adversarial review at BOTH the propose and apply phases before it is
committed. Actively challenge the design:

- **Propose phase**: Does the change add public surface without concrete product pressure
  documented through OpenSpec? Does it move a user obligation (evidence truth, slot meaning,
  persistence, clocks, async work, I/O, scheduling, reactions) into the core? Does it add Worklane
  types or job-queue vocabulary, or serialization, async, storage, clocks, callbacks, quorum,
  any-of, or dynamic slot growth without a concrete consumer?
- **Apply phase**: Does the implementation remove, rename, or semantically repurpose a public
  item? Does the facade remain a complete, logic-free re-export? Does the contract stay
  `no_std + alloc`, and does it still build on the Rust 1.85 floor? For a deliberate law change:
  are there focused violating and clean reaction proofs and a regenerated projection?

Reject or redesign changes that grow the core in anticipation rather than from concrete product
pressure.

## Governance and Conformance

Lengkap separates the *judgment* from the *check on its projection*.

- **Governance is judgment, and lives in prose** — `openspec/specs/`, this file, `PROJECT.md`,
  and `BACKLOG.md`. Intent and meaning are decided here and stay review-governed.
- **Code is the projection of a judgment onto the structural plane** — a `pub use` set, an absent
  `async fn`, a dependency edge, a missing trait bound.
- **Conformance verifies the projection still matches the judgment.** It is a family: Tianheng
  (structure, dependencies, source scans), `rustc` (type facts), and tests (behavior). They bite
  the projection, never the judgment itself.

Tianheng's accepted constitution projects into `AGENTS.lengkap-law.md`; a freshness test byte-checks
that generated context against the live declaration, so accepted law is visible without a second
hand-maintained authority. A green gate means "no visible violation", not proof: a judgment that
casts no structural shadow stays prose, and a source scan cannot see what a macro expands to.

Before turning a judgment into a Tianheng tooth, it must pass four gates — casting a shadow is
necessary, not sufficient:

1. **Shadow** — does the judgment project into a syntactically decidable structural fact? (No →
   it stays prose and review.)
2. **Faithful** — is that fact a faithful proxy, not a gameable one? (Lines of code are not
   thinness; a proxy invites Goodhart.)
3. **Stable** — is the judgment stable? A tooth on a moving projection is a recurring maintenance
   tax and a second copy of the truth; prefer a test.
4. **Sync** — is the extra `prose ⟷ tooth` coupling worth it? The tooth is itself a *second
   projection* of the judgment, and nothing mechanically checks it matches the prose — only
   review does. The regress terminates in a human.

Fail any gate and the honest home is prose, review, or a test — never a faked tooth. A tooth
complements review; it never replaces it. Where an accepted boundary does hold a claim, its
reason is the single statement of that rule, and prose that merely restated it may be retired.

Repair code toward a violated reason; never weaken a law, baseline new drift, or change severity
merely to make a check green. A deliberate law change requires explicit authority, focused
violating and clean reaction proofs, projection regeneration, and adversarial review.

## OpenSpec Workflow

`openspec/` is the version-controlled, agent-neutral source of truth: `openspec/specs/` is the
living specification of what the system is, and `openspec/changes/` holds active change proposals
as delta specs. Per-agent command files (`.claude/`, `.codex/`, editor shims) are generated per
clone and never committed; generate your own with `openspec init --tools <tool>`.

The lifecycle is:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: investigate and shape intent. Read the relevant `openspec/specs/` first. Do not
   write feature code outside a change.
2. **Propose**: `openspec new change "<change>"`, then write `proposal.md`, `design.md`,
   `tasks.md`, and delta specs with success, failure, and edge scenarios. Commit as
   `docs(<change>): propose <summary>`.
3. **Apply**: implement against the active delta specs, one task at a time, and check a task off
   only after the Definition of Done passes. Keep changes minimal and scoped; never bundle
   unrelated work. Commit coherent compiling milestones as `feat(...)` or `fix(...)`.
4. **Sync**: merge verified delta specs into `openspec/specs/` (agent-driven — the CLI has no sync
   command), then `git rm -r openspec/changes/<change>/`. There is no archive: the change's
   content now lives in `openspec/specs/` and git history. Never run `openspec archive`. Commit
   as `docs(specs): sync <change>`.

Requirement changes reach `openspec/specs/` through sync, never through silent code edits.
Without agent slash commands, use the CLI:

```bash
openspec list [--json] [--specs]
openspec new change "<change>"
openspec status --change "<change>" --json
openspec instructions <artifact> --change "<change>"
```

## Language

- Write OpenSpec artifacts, `BACKLOG.md` entries, code comments, and commit messages in English.
- Converse with users in the language they use.
- Wrap Markdown prose near 100 columns; tables and code blocks are exempt.

## Commit And Integration Governance

### Branch Commits

- Use Conventional Commits: `type(scope): summary`.
- Write the subject in English, lowercase imperative mood, at no more than 72 characters.
- Use the body to record motivation, important decisions, constraints, and verification when that
  context exists. Do not merely enumerate changed files.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull request is
  squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is needed, what changed,
  consequential decisions or tradeoffs, and verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title. Hosting tools append
  the pull request number by default; remove it.
- Give every squash commit a non-empty, self-describing body distilled from the approved pull
  request body: preserve durable rationale, decisions, constraints, and verification; omit
  transient checklists and generated commit lists.
- Do not append a pull request number, issue number, or URL to the squash subject or body.
- Every content-changing commit on `main`, including release preparation, must come from a
  squash-merged pull request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution in commits, pull
  requests, tags, changelogs, or release notes.
- Prohibited forms include AI `Co-authored-by` trailers, `generated by`, `written with`, model or
  agent names used as signatures, and tool signatures.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

### Changelog

- `CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
  [Semantic Versioning](https://semver.org/spec/v2.0.0.html), and is a strict release ledger: it
  has no `[Unreleased]` section. Unreleased work is recorded in OpenSpec changes, pull requests,
  and `BACKLOG.md`.
- Write each version's entry in its own release-preparation pull request, cross-checked against
  the commit history since the previous release.
- Every `## [X.Y.Z] - YYYY-MM-DD` heading has a matching `[X.Y.Z]: <url>` footer link to
  `.../releases/tag/vX.Y.Z`. `scripts/changelog-guard.sh` checks this and runs in the Definition
  of Done.

### Release Finalization

- Prepare release content in a pull request whose squash subject is exactly
  `chore(release): prepare X.Y.Z`.
- Sweep crate-level README files and other non-governed prose for stale version markers or
  disposition language that `BACKLOG.md` has since resolved, superseded, or placed downstream.
- Give the release preparation squash commit a non-empty body describing scope, compatibility,
  metadata changes, and verification.
- Run the complete Definition of Done after that commit reaches `main`.
- Publish crates in dependency order, waiting for each to appear in the crates.io index before
  publishing its dependents. If an upload's result is uncertain, query crates.io for the exact
  version before retrying — a published version cannot be overwritten.
- Finalize with annotated tag `vX.Y.Z` on that commit, with message exactly `release: X.Y.Z`.
- Push the tag without another commit. Release branches and empty release commits are not part
  of the flow.

## API And Release Discipline

The decision and structural-error enums are intentionally exhaustive because their variants
define the finite outcome space. Generic values and causes carry domain extensibility.

Removing, renaming, or semantically repurposing any public item is breaking. Add public surface
only for concrete product pressure documented through OpenSpec. A real consumer remains a
graduation test, not the sole source of product authority. Every pull request body states its
compatibility.

Do not run `cargo publish`, create a release tag, or create a GitHub release without a separately
authorized release change.

### Release Verification

These steps add to Release Finalization above; they do not replace it.

- The public release set is `lengkap-contract` followed by the dependent `lengkap` facade at the
  same version; `lengkap-governance` stays unpublished and is never included in a publish
  command.
- Before merging the release preparation pull request, inspect both product package archives and
  run a publication dry-run for the contract.
- After the squash commit reaches `main`, confirm the working tree is clean and exactly at that
  commit.
- Publish `lengkap-contract` first. Only after it is visible in the crates.io index, dry-run and
  then publish `lengkap`.
- Before tagging, verify a fresh external project with no path, patch, or source override,
  depending on the exact released `lengkap` version on the declared Rust 1.85 floor.
- Tag only after both crates and the external consumer verification succeed, then create the
  matching GitHub release from that tag.

## Definition Of Done

Run these from the workspace root before checking off implementation tasks or syncing specs. This
is the single source for the gate list — `README.md` and `docs/development-flow.md` point here
rather than restating it. If a command cannot run in the current environment, report that
explicitly.

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets
cargo +1.88 check --workspace --all-targets
cargo semver-checks --package lengkap-contract --baseline-version 0.1.0
cargo semver-checks --package lengkap --baseline-version 0.1.0
cargo deny check
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
./scripts/changelog-guard.sh
cargo +1.88 build --workspace
```

The Rust 1.85 gate is the public product contract. The Rust 1.88 full-workspace gates cover
repository-only governance tooling as a separate compatibility surface. The semver checks compare
only the two publishable product crates with their exact crates.io 0.1.0 baselines.
`lengkap-governance` observes normal dependencies, inline calls into selected standard-library
paths, public async functions, inline `std::time` `now` calls, and serde marker acquisition; it does
not prove every runtime effect or the semantic meaning of user evidence.

CI (`.github/workflows/ci.yml`) runs the same gates on push and pull request. Do not check a task
off, sync, or integrate while any required gate fails.
