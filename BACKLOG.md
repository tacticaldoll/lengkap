# Backlog And Deferred Decisions

## Origin

Lengkap was identified while examining Worklane's fan-in watcher. The queue
adapter already needed a pure rule: fixed required positions accumulate stored
results monotonically, become ready only when none are missing, and fail when
an unresolved job reaches a terminal state without a usable result.

Broker reads, async polling, result persistence, and callback behavior belong
to Worklane. The all-of decision does not. This repository tests whether that
mechanism can stand as an independent, user-obligation-driven component.

## Settled Decisions

- **One product core, not two.** `Assembly` state and adjudication are one small
  mechanism. Splitting them would create a seam without an independent user.
- **A facade exists from birth.** `lengkap` is only a complete glob re-export,
  giving consumers one curated dependency without creating a second product.
- **Fixed numeric slots.** Position supplies stable output order without
  domain identifiers, map policy, or `Eq + Hash` bounds.
- **Ownership over cloning.** Adjudication consumes its input and returns
  ownership, keeping user values and causes free of policy trait requirements.
- **First capture wins across calls.** Later findings cannot replace or revoke
  a captured value. Contradictory-evidence policy belongs to the caller.
- **Duplicates fail within one call.** Arrival order never decides between two
  simultaneous claims about the same slot.
- **Lowest impossible slot wins.** Terminal selection is deterministic and
  independent of finding order.
- **Empty assembly is ready.** The generic all-of identity remains intact;
  Worklane may reject empty fan-in groups at its adapter boundary.
- **Outcome enums are exhaustive.** Their variants are the finite contract;
  extensibility belongs in generic values and causes.
- **Unconditional `no_std + alloc`.** The mechanism has no need for standard
  library or external dependencies.
- **Governance states only observable facts.** Tianheng checks direct
  dependencies, selected source paths, async exposure, clock reads, and serde
  markers. A separate test checks the `#![no_std]` declaration.
- **Sync and archive are two gates; no archive folder is kept.** Sync
  merges verified delta specs into `openspec/specs/` and leaves the change
  directory active for verification. Archive is a distinct, later gate
  that removes the change directory once verified; its deliberation lives
  in git history and the merged pull request. `openspec/changes/archive/`
  stays empty except `.gitkeep`; `openspec archive` (which recreates that
  folder) is not used.
- **Independent product authority.** Concrete Lengkap pressure can justify core
  evolution without Worklane adoption. A real consumer remains a separate
  graduation test, not a prerequisite for product discovery.
- **Recoverable ownership boundaries.** Impossible decisions return accumulated
  state, while structural errors return both unchanged inputs.
- **Caller-owned slot transfer.** Ordered optional slots are the in-memory
  checkpoint seam; encoding, versioning, and persistence remain outside.
- **Bounded exhaustive evidence.** The small finite contract is checked across
  every state mask, unique-slot finding assignment, and arrival permutation
  through four slots without a test dependency.
- **Reactive MSRV.** Rust 1.88 is a dedicated workspace all-targets CI
  reaction, not manifest metadata alone.
- **Release-ledger changelog.** Pending work stays in OpenSpec, pull requests,
  and this backlog; `CHANGELOG.md` gains entries only during tagged release
  preparation.

## Deferred Work

- **Worklane bridge adoption.** Reassess fit in Worklane's own OpenSpec process;
  do not assume origin implies adoption.
- **Tier 1 graduation and first release.** Requires a real bridge consumer and
  a separately authorized crates.io and GitHub release.
- **Semver compatibility gate.** Consider `cargo-semver-checks` only once a
  published baseline exists.
- **Alternative completion modes.** Quorum, any-of, weighted evidence, dynamic
  slots, and contradiction policy remain out of scope without concrete demand.

## Shipped

- ✓ **Initial project shape (pre-release; archived 2026-07-31).** Contract,
  facade, governance, CI, documentation, and OpenSpec truth are complete. No
  crate, tag, or GitHub release was created.
- ✓ **Recoverable completion API (pre-release; archived 2026-07-31).**
  Impossible decisions preserve accumulated state, structural errors return
  both inputs, ordered slot transfer supports caller-owned checkpoints,
  progress is inspectable, and method-style adjudication remains equivalent to
  the free function.
- ✓ **Pre-release contract hardening (archived 2026-07-31).** Recovered
  findings return owned payloads, errors are domain-neutral, bounded
  state/permutation and invalid-input tests defend the contract, Rust 1.88 is
  enforced in CI, and release/license documentation is consistent.
- ✓ **Repository-local governor packaging (archived 2026-07-31).** The
  governance package is explicitly unpublished and relies on repository-root
  licenses, while publishable product crates retain self-contained license
  artifacts.
