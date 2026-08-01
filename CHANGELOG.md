# Changelog

This file is a ledger of released versions, based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Normal development is recorded in OpenSpec changes, pull requests, and
`BACKLOG.md`, not in an `[Unreleased]` section.

A release preparation pull request adds a heading in this form:
`## [X.Y.Z] - YYYY-MM-DD`. Its footer links the version to the matching GitHub
tag:
`[X.Y.Z]: https://github.com/tacticaldoll/lengkap/releases/tag/vX.Y.Z`.

## [0.1.2] - 2026-08-01

### Added

- Three dogfooding examples: `impossible_then_recovered`, `readiness_gate`,
  and `resubmit_after_structural_error`, demonstrating that an `Impossible`
  slot remains recoverable, that the contract's naming stays ergonomic
  outside queue vocabulary, and that a caller can recover and resubmit
  after a structural error.

## [0.1.1] - 2026-07-31

### Added

- Semver compatibility reactions for both product crates against their exact
  crates.io 0.1.0 baselines.
- A domain-neutral checkpoint-and-restore example and focused complete, empty,
  and incomplete ready-extraction tests.

### Changed

- Tier 1 consumer-proven status after Worklane adopted the registry facade
  without moving broker access, persistence, polling, or reactions into
  Lengkap.
- Total ownership-based ready-value extraction with no internal production
  panic assertion.

## [0.1.0] - 2026-07-31

### Added

- A zero-dependency, unconditional `no_std + alloc` contract for fixed,
  ordered all-of evidence completion.
- Monotonic first-value capture with deterministic `Pending`, `Ready`, and
  recoverable `Impossible` decisions.
- Atomic structural validation that returns unchanged caller-owned inputs.
- Caller-owned checkpoint transfer and stable progress inspection.
- The logic-free `lengkap` facade as the recommended public entrypoint.
- Executable Tianheng architecture governance, split Rust 1.85 product and Rust
  1.88 repository gates, and bounded exhaustive contract tests.

[0.1.2]: https://github.com/tacticaldoll/lengkap/releases/tag/v0.1.2
[0.1.1]: https://github.com/tacticaldoll/lengkap/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/lengkap/releases/tag/v0.1.0
