# Example Documentation Specification

## Purpose

Define the requirements for the runnable, dependency-free examples under
`crates/lengkap-contract/examples/` that dogfood and document
`lengkap-contract`'s behavioral guarantees without adding coupling to the
core.

## Requirements

### Requirement: Examples stay dependency-free and locally scoped

Every example under `crates/lengkap-contract/examples/` SHALL compile without
adding a crate dependency and SHALL define any domain vocabulary it uses
locally within the example file rather than depending on a real domain
crate.

#### Scenario: An example adds no dependency

- **WHEN** a new example is added under `crates/lengkap-contract/examples/`
- **THEN** `Cargo.toml` gains no new `[dependencies]` or `[dev-dependencies]`
  entry to support it

#### Scenario: An example's domain vocabulary is local

- **WHEN** an example models a caller domain (for example, job observation or
  startup subsystems)
- **THEN** the types for that domain are defined inside the example file and
  are not imported from an external domain crate

### Requirement: Examples are referenced from README

`README.md` SHALL reference every example under
`crates/lengkap-contract/examples/` by name, in the same style as existing
references.

#### Scenario: A new example is discoverable from README

- **WHEN** an example is added under `crates/lengkap-contract/examples/`
- **THEN** `README.md` names that example and describes what it demonstrates

### Requirement: An example demonstrates recoverability after Impossible

An example SHALL demonstrate that a slot reported `Impossible` remains
unresolved in the returned assembly, and that a caller may resubmit a
`Produced` finding for that same slot in a later call and still reach
`Ready`.

#### Scenario: A caller retries after Impossible and reaches Ready

- **WHEN** an unresolved slot receives an `Impossible` finding and the
  caller subsequently submits a `Produced` finding for that same slot
- **THEN** the example's final decision is `Ready` with that slot's later
  value

### Requirement: An example demonstrates domain-neutral naming

An example SHALL model an all-of completion scenario outside queue or job
vocabulary, using `Assembly`, `Slot`, and `Finding` directly, to demonstrate
that this naming remains ergonomic outside the Worklane-shaped domain.

#### Scenario: A non-queue domain reaches Ready

- **WHEN** a startup-readiness scenario with fixed, named subsystems supplies
  a produced finding for every slot
- **THEN** the example's final decision is `Ready` with every subsystem's
  value in slot order

### Requirement: An example demonstrates structural-error recovery

An example SHALL demonstrate recovering from an `AdjudicationError` caused by
a caller-side mapping mistake against an assembly that already holds prior
captured progress, using `kind()` to identify the error and `into_parts()`
to recover both the unchanged assembly and the finding batch, then
resubmitting a corrected batch to reach a valid decision.

#### Scenario: A caller corrects and resubmits after a structural error

- **WHEN** a batch containing an out-of-range finding is adjudicated against
  an assembly with prior captured progress and returns a `SlotOutOfRange`
  `AdjudicationError`
- **THEN** the caller recovers the assembly and batch via `into_parts()`,
  removes the invalid finding, and a corrected resubmission returns
  `Pending` or `Ready` with the prior progress intact
