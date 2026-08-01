# Curated Facade Specification

## Purpose

Define `lengkap` as the complete logic-free public re-export of
`lengkap-contract`.

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
