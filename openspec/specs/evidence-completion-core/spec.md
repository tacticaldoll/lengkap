# Evidence Completion Core Specification

## Purpose

Define Lengkap's domain-neutral, `no_std + alloc` mechanism for accumulating
ordered all-of evidence and deciding pending, ready, impossible, or structurally
invalid outcomes without owning evidence truth or effects.

## Requirements

### Requirement: An assembly has fixed ordered slots

The core SHALL construct an `Assembly<Value>` with a fixed number of slots.
Each slot SHALL be addressable by its zero-based `Slot`, and the slot count
SHALL NOT change during adjudication.

#### Scenario: A new assembly starts unresolved

- **WHEN** a caller constructs an assembly with three slots
- **THEN** all three slots are unresolved in their stable zero-based order

#### Scenario: Slot count remains fixed

- **WHEN** findings are adjudicated against an assembly
- **THEN** the returned pending assembly has the same number and order of slots

### Requirement: Produced values are captured monotonically

A produced finding for an unresolved slot SHALL capture its value. Once a slot
has captured a value, later produced or impossible findings for that slot SHALL
NOT replace, revoke, or otherwise alter the captured value.

#### Scenario: A produced value fills an unresolved slot

- **WHEN** an unresolved slot receives a produced finding
- **THEN** the returned assembly contains that value at the same slot

#### Scenario: Later production cannot replace a value

- **WHEN** a slot captured a value in an earlier adjudication and later receives
  a different produced finding
- **THEN** the original value remains captured

#### Scenario: Later impossibility cannot revoke a value

- **WHEN** a slot captured a value in an earlier adjudication and later receives
  an impossible finding
- **THEN** the original value remains captured and does not make the decision
  impossible

### Requirement: Completion requires every slot

Adjudication SHALL return `Ready` if and only if every slot contains a captured
value. Ready values SHALL be returned in slot order, independently of finding
arrival order.

#### Scenario: Every slot is complete

- **WHEN** the last unresolved slots receive produced findings
- **THEN** adjudication returns `Ready` with every value in slot order

#### Scenario: Findings arrive out of order

- **WHEN** produced findings arrive in an order different from their slots
- **THEN** the ready values are still ordered by slot

#### Scenario: An empty assembly is complete

- **WHEN** an assembly has zero slots and receives no findings
- **THEN** adjudication returns `Ready` with an empty value vector

### Requirement: Missing evidence remains pending

Adjudication SHALL return `Pending` when at least one slot remains unresolved
and no unresolved slot has an impossible finding. Absence of a finding SHALL
mean only that the slot remains unresolved.

#### Scenario: No findings leave a non-empty assembly pending

- **WHEN** a non-empty assembly receives no findings
- **THEN** adjudication returns `Pending` with every prior captured value intact

#### Scenario: Partial production remains pending

- **WHEN** produced findings cover only some unresolved slots
- **THEN** adjudication returns `Pending` with those values captured and the
  remaining slots unresolved

### Requirement: Impossibility is deterministic

An impossible finding for an unresolved slot SHALL make the decision
`Impossible`. If multiple unresolved slots are impossible in one valid input,
the decision SHALL report the lowest slot and its cause, independently of
finding order.

#### Scenario: An unresolved slot becomes impossible

- **WHEN** an unresolved slot receives an impossible finding
- **THEN** adjudication returns `Impossible` naming that slot and cause

#### Scenario: Lowest impossible slot wins

- **WHEN** multiple unresolved slots receive impossible findings in any order
- **THEN** adjudication reports the lowest such slot and its associated cause

#### Scenario: Captured slots do not participate in impossibility

- **WHEN** an already captured slot and an unresolved slot both receive
  impossible findings
- **THEN** adjudication reports only the unresolved slot as impossible

### Requirement: Structural errors are atomic

Adjudication SHALL reject an out-of-range slot or duplicate findings for the
same slot in one call as a structured error. It SHALL validate all findings
before capture and SHALL return the original assembly unchanged on error.

#### Scenario: An out-of-range slot is rejected

- **WHEN** a finding addresses a slot outside the assembly
- **THEN** adjudication returns an out-of-range structural error with the
  original assembly

#### Scenario: Same-call duplicate findings are rejected

- **WHEN** one adjudication input contains two findings for the same slot
- **THEN** adjudication returns a duplicate-finding structural error with the
  original assembly

#### Scenario: An error after valid-looking input captures nothing

- **WHEN** an input contains a valid produced finding followed by any structural
  error
- **THEN** the returned original assembly has not captured the produced value

### Requirement: The contract is sans-I/O and domain-neutral

The core SHALL be an unconditional `no_std + alloc`, zero-dependency library.
It SHALL NOT produce evidence, perform I/O, own clocks or async work, serialize
values, persist assemblies, or prescribe reactions to a decision. Its public
model SHALL NOT depend on Worklane or expose job-queue vocabulary.

#### Scenario: A no-std consumer uses the core

- **WHEN** a consumer builds the contract without the Rust standard library
- **THEN** the evidence-completion API remains available using `alloc`

#### Scenario: A Worklane-shaped caller supplies domain meaning

- **WHEN** a caller maps live work to no finding, completed work with a result
  to production, and terminal work without a result to impossibility
- **THEN** the core adjudicates only the supplied slots and findings without
  importing Worklane types or performing broker access

#### Scenario: User values need no policy traits

- **WHEN** user value and cause types do not implement `Clone`, `Eq`, `Hash`, or
  serialization traits
- **THEN** they can still be adjudicated by ownership
