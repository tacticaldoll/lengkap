# Curated Facade Specification

## Purpose

Define `lengkap` as the complete logic-free public re-export of
`lengkap-contract` and keep its registry distribution state explicit.

## Requirements

### Requirement: The facade re-exports the complete contract API

The `lengkap` crate SHALL publicly re-export every public item from
`lengkap-contract`, including recovery, checkpoint, progress, and method-style
adjudication surfaces, and SHALL NOT define a second copy or wrapper of any
contract type.

#### Scenario: A consumer uses the facade only

- **WHEN** a consumer depends on `lengkap` and imports the evidence-completion
  API
- **THEN** all contract types and functions needed for adjudication, recovery,
  checkpoint transfer, and progress inspection are available from the facade
  root

#### Scenario: Facade and contract types are identical

- **WHEN** a value constructed through a facade import is passed where the
  contract's type is expected
- **THEN** it type-checks without conversion or adaptation

#### Scenario: Method entrypoints pass through the facade

- **WHEN** a caller constructs an assembly through the facade
- **THEN** batch and single-finding adjudication methods are available without a
  facade wrapper

### Requirement: The facade contains no product logic

The `lengkap` crate SHALL contain only crate documentation and the complete
public re-export of `lengkap-contract`. All evidence-completion behavior SHALL
remain in the contract crate.

#### Scenario: The facade is inspected

- **WHEN** the facade source is reviewed or tested
- **THEN** it contains no independent adjudication, state, I/O, or domain policy

### Requirement: Release state is explicit

The contract and facade manifests SHALL contain complete package metadata.
Repository and package documentation SHALL identify version 0.1.0 as the
initial public release. Project maturity SHALL be Tier 1 only after a real
consumer adopts the registry facade without moving domain vocabulary,
evidence truth, persistence, scheduling, I/O, or reactions into Lengkap.

#### Scenario: Package metadata is validated before release

- **WHEN** the workspace is packaged in verification mode
- **THEN** each publishable crate has the metadata and included files required
  for crates.io publication

#### Scenario: Facade resolves the published contract

- **WHEN** a registry consumer depends on `lengkap` version 0.1.0
- **THEN** Cargo resolves `lengkap-contract` version 0.1.0 from the registry
- **THEN** the complete contract API remains available through the facade root

#### Scenario: Registry adoption graduates project maturity

- **WHEN** Worklane or another real consumer resolves the published facade from
  crates.io without a path or source override
- **AND** that consumer keeps domain vocabulary, evidence truth, persistence,
  scheduling, I/O, and reactions outside Lengkap
- **THEN** project and package documentation identify Lengkap as Tier 1

#### Scenario: No real registry consumer has adopted

- **WHEN** no real consumer resolves and uses a published registry artifact
- **THEN** documentation does not claim Tier 1 maturity

#### Scenario: Adoption leaks consumer policy into the contract

- **WHEN** a consumer can adopt only by adding its domain vocabulary or I/O
  responsibilities to Lengkap
- **THEN** that adoption does not satisfy the Tier 1 graduation condition

#### Scenario: Authorized release finalization completes

- **WHEN** both product crates are published and the release is tagged
- **THEN** the changelog and package documentation identify the released
  version
- **THEN** project maturity reflects the strongest completed consumer evidence
