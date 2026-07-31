# Release Distribution Specification

## Purpose

Define Lengkap's public product release set, ordered registry publication,
external verification, and tag finalization boundary.

## Requirements

### Requirement: The public release set contains only product crates

Each Lengkap release SHALL publish `lengkap-contract` and the `lengkap` facade
at the same version. `lengkap-governance` SHALL remain unpublished and SHALL
NOT be included in a Cargo publication command.

#### Scenario: The complete product release set is enumerated

- **WHEN** release preparation identifies workspace packages for publication
- **THEN** it includes `lengkap-contract` and `lengkap` at the release version
- **THEN** it excludes `lengkap-governance`

#### Scenario: Governor publication is attempted

- **WHEN** a release command targets `lengkap-governance`
- **THEN** its explicit unpublished manifest state prevents publication

#### Scenario: Product versions diverge

- **WHEN** the contract and facade do not resolve to the same release version
- **THEN** release finalization stops before a tag or GitHub release is created

### Requirement: Publication follows dependency order

The release transaction SHALL publish and verify `lengkap-contract` before it
verifies or publishes the dependent `lengkap` facade from crates.io.

#### Scenario: Contract publication succeeds

- **WHEN** `lengkap-contract` is packaged from the exact release commit and
  version 0.1.0 becomes visible in crates.io
- **THEN** facade publication verification may begin

#### Scenario: Contract publication fails

- **WHEN** the contract upload fails and version 0.1.0 is not present in
  crates.io
- **THEN** the facade is not published
- **THEN** no release tag or GitHub release is created

#### Scenario: Registry indexing is delayed

- **WHEN** the contract upload reports success but the registry dependency is
  not yet resolvable
- **THEN** release finalization waits and rechecks the exact published version
- **THEN** it does not publish the facade with a path or source override

#### Scenario: Publication status is ambiguous

- **WHEN** an upload times out or returns an uncertain result
- **THEN** the maintainer checks crates.io for the exact crate and version
  before considering any retry

### Requirement: Release artifacts are verified at their public boundary

Before publication, the complete workspace SHALL pass its required gates and
each product package SHALL contain its required metadata, source, README, and
dual-license files. After publication, a fresh external Rust 1.85 consumer
SHALL resolve `lengkap` from crates.io and use the facade API successfully.

#### Scenario: Workspace or package verification fails

- **WHEN** a required repository gate fails or a product package omits a
  required artifact
- **THEN** no crate is uploaded

#### Scenario: Both packages satisfy release verification

- **WHEN** the exact release commit passes every repository gate and package
  inspection
- **THEN** the ordered publication transaction may begin

#### Scenario: External consumer succeeds

- **WHEN** a new Rust 1.85 project depends on `lengkap = "0.1.0"` from
  crates.io and exercises adjudication through the facade
- **THEN** dependency resolution, compilation, and execution succeed without a
  path override

#### Scenario: External consumer fails

- **WHEN** the registry facade cannot be resolved or used at the declared Rust
  1.85 floor
- **THEN** the release remains untagged and has no GitHub release

### Requirement: Finalization names only a complete registry release

The exact release preparation commit on `main` SHALL receive annotated tag
`v0.1.0` with message `release: 0.1.0` only after both product crates and the
external consumer verification succeed. The matching GitHub release SHALL be
created from that tag without another content commit.

#### Scenario: Complete registry release is finalized

- **WHEN** both product crates are visible at version 0.1.0 and the external
  consumer passes
- **THEN** the release commit is tagged with annotated tag `v0.1.0`
- **THEN** the matching GitHub release is created from that tag

#### Scenario: Only one product crate is available

- **WHEN** either product crate is absent at version 0.1.0
- **THEN** no `v0.1.0` tag or GitHub release is created

#### Scenario: A finalization target differs from the release commit

- **WHEN** the proposed tag target is not the verified release preparation
  commit on `main`
- **THEN** release finalization stops
