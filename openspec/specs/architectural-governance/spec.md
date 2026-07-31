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

Continuous integration SHALL run build, test, clippy with warnings denied,
format checking, rustdoc with warnings denied, cargo-deny, and the Tianheng
governance check.

#### Scenario: A required gate fails

- **WHEN** any required repository gate returns a non-zero status
- **THEN** the CI workflow fails before the change is considered releasable

#### Scenario: All required gates pass

- **WHEN** the source, dependencies, documentation, and architecture satisfy all
  declared checks
- **THEN** CI reports every required gate as successful
