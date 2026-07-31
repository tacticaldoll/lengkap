# Architectural Governance Specification

## Purpose

Define the executable Tianheng boundaries, reaction evidence, and continuous
integration gates that keep Lengkap's contract, facade, and governor aligned
with their declared responsibilities.

## Requirements

### Requirement: The contract has zero dependencies

`lengkap-contract` SHALL declare no normal dependency, including dependencies on
another workspace crate.

#### Scenario: A dependency is introduced

- **WHEN** the contract manifest declares any normal dependency
- **THEN** the governance check fails with the boundary name and reason

#### Scenario: The contract remains dependency-free

- **WHEN** the contract manifest declares no normal dependency
- **THEN** the dependency boundary passes

### Requirement: The contract performs no I/O

`lengkap-contract` SHALL NOT use source paths under `std::io`, `std::fs`,
`std::net`, or `std::process`, and SHALL NOT introduce public async or direct
wall-clock observation.

#### Scenario: An I/O path is introduced

- **WHEN** contract source uses a forbidden standard-library I/O path
- **THEN** the governance check fails with the source location and reason

#### Scenario: Public async is introduced

- **WHEN** contract source exposes an async public function
- **THEN** the governance check fails with the source location and reason

#### Scenario: The pure contract is checked

- **WHEN** contract source contains only deterministic in-memory adjudication
- **THEN** every sans-I/O boundary passes

### Requirement: Serialization policy stays outside the contract

`lengkap-contract` SHALL NOT derive or implement `Serialize` or `Deserialize`.

#### Scenario: A serialization marker is introduced

- **WHEN** contract source derives or implements `Serialize` or `Deserialize`
- **THEN** the governance check fails with the source location and reason

#### Scenario: Domain values remain opaque

- **WHEN** the contract stores generic values without serialization markers
- **THEN** the serialization boundary passes

### Requirement: The facade remains isolated and logic-free

`lengkap` SHALL depend only on `lengkap-contract` among normal dependencies and
SHALL expose the contract through a complete public re-export without adding
domain logic.

#### Scenario: The facade acquires an unrelated dependency

- **WHEN** the facade manifest declares a normal dependency other than the
  contract
- **THEN** the governance check fails with the boundary name and reason

#### Scenario: The facade re-exports the contract

- **WHEN** a consumer imports a contract type through `lengkap`
- **THEN** it receives the same public type exported by `lengkap-contract`

### Requirement: The governor remains independent

`lengkap-governance` SHALL depend only on Tianheng and SHALL NOT depend on any
workspace crate it judges.

#### Scenario: The governor depends on a judged crate

- **WHEN** the governor manifest declares a dependency on the contract or facade
- **THEN** the governance check fails with the boundary name and reason

#### Scenario: The governor depends only on Tianheng

- **WHEN** the governor manifest has Tianheng as its sole normal dependency
- **THEN** the governor-independence boundary passes

### Requirement: The governor is repository-local

`lengkap-governance` SHALL explicitly declare itself unpublished and SHALL rely
on the repository-root license files rather than carry release-package license
copies. Publishable product crates SHALL retain the license files needed in
their independently distributed packages.

#### Scenario: Governor package metadata is inspected

- **WHEN** a maintainer inspects `lengkap-governance` package metadata
- **THEN** the manifest declares `publish = false` directly
- **THEN** the result does not depend on the workspace publication default

#### Scenario: Governor distribution is attempted

- **WHEN** release preparation enumerates publishable Lengkap packages
- **THEN** `lengkap-governance` is excluded from the release set
- **THEN** only repository-root license files govern its repository-local
  packaging convention

#### Scenario: Product crate packaging is inspected

- **WHEN** either publishable product crate is prepared as a standalone Cargo
  package
- **THEN** its package contains both Apache-2.0 and MIT license files
- **THEN** removing governor-local copies does not remove product license
  artifacts

### Requirement: Every law has reaction evidence

Each enforced boundary SHALL have a focused violating fixture and clean control
whose checks prove the intended exit class and diagnostic identity.

#### Scenario: A violating fixture reacts

- **WHEN** a focused fixture violates exactly one accepted boundary
- **THEN** its proof observes governance exit class 1 and the intended boundary
  diagnostic, not a scan or configuration failure

#### Scenario: A clean fixture remains clean

- **WHEN** the corresponding clean control is checked
- **THEN** governance returns exit class 0

### Requirement: CI enforces the repository gates

`lengkap-contract` and `lengkap` SHALL each declare Rust 1.85 as their minimum
supported Rust version. The unpublished `lengkap-governance` package SHALL
inherit the repository's Rust 1.88 tooling floor.

Continuous integration SHALL run build, test, clippy with warnings denied,
format checking, rustdoc with warnings denied, cargo-deny, and the Tianheng
governance check. It SHALL also run a product all-targets check on Rust 1.85 and
a full-workspace all-targets check on Rust 1.88 as separately identified
reactions.

#### Scenario: A required gate fails

- **WHEN** any required repository gate returns a non-zero status
- **THEN** the CI workflow fails before the change is considered releasable

#### Scenario: Product compatibility regresses

- **WHEN** either publishable product crate no longer checks successfully on
  Rust 1.85
- **THEN** the product MSRV CI job fails with its own reaction identity

#### Scenario: Repository tooling compatibility regresses

- **WHEN** any workspace target no longer checks successfully on Rust 1.88
- **THEN** the full-workspace MSRV CI job fails with its own reaction identity

#### Scenario: Governor is outside the product MSRV

- **WHEN** CI checks the Rust 1.85 product compatibility contract
- **THEN** it checks `lengkap-contract` and `lengkap` with all targets
- **THEN** it does not require the unpublished governor to compile on Rust 1.85

#### Scenario: All required gates pass

- **WHEN** the product crates support Rust 1.85 and the full repository supports
  Rust 1.88
- **AND** source, dependencies, documentation, and architecture satisfy every
  other required check
- **THEN** CI reports every required gate as successful
