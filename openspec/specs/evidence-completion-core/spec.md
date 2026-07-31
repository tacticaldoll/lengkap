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

### Requirement: Assembly state crosses a caller-owned checkpoint boundary

The core SHALL let a caller consume an assembly into ordered optional slots and
restore an assembly from ordered optional slots. The round trip SHALL preserve
slot count, slot order, resolved positions, and owned values without requiring
serialization or cloning.

#### Scenario: A partial assembly round-trips

- **WHEN** a caller exports and restores an assembly with some captured values
- **THEN** every captured value and unresolved slot remains at the same position

#### Scenario: An empty assembly round-trips

- **WHEN** a caller exports and restores a zero-slot assembly
- **THEN** the restored assembly is complete and still has zero slots

#### Scenario: A complete assembly round-trips

- **WHEN** a caller exports and restores an assembly whose slots are all
  captured
- **THEN** the restored assembly remains complete with the same ordered values

#### Scenario: The caller owns persistence policy

- **WHEN** a caller moves exported slots through its own storage or encoding
  adapter
- **THEN** the core performs no serialization, I/O, versioning, or persistence

### Requirement: Assembly progress is inspectable

An assembly SHALL report captured and remaining counts and SHALL enumerate
unresolved slots in stable ascending order without consuming the assembly.
Captured plus remaining counts SHALL equal the fixed slot count.

#### Scenario: Partial progress is counted

- **WHEN** two of five slots have captured values
- **THEN** captured count is two and remaining count is three

#### Scenario: Unresolved slots retain stable order

- **WHEN** only slots one and three of a four-slot assembly are captured
- **THEN** unresolved-slot enumeration yields slots zero and two in that order

#### Scenario: Empty progress is complete

- **WHEN** progress is inspected for a zero-slot assembly
- **THEN** both counts are zero and unresolved-slot enumeration is empty

#### Scenario: Inspection does not consume state

- **WHEN** a caller inspects counts and unresolved slots before adjudication
- **THEN** the same assembly remains available with all captured values intact

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
`Impossible`. The decision SHALL return the accumulated assembly together with
the selected slot and cause. Produced findings in the same valid call SHALL be
captured in that returned assembly. If multiple unresolved slots are impossible
in one valid input, the decision SHALL report the lowest slot and its cause,
independently of finding order.

#### Scenario: An unresolved slot becomes impossible

- **WHEN** an unresolved slot receives an impossible finding
- **THEN** adjudication returns `Impossible` naming that slot and cause and
  returning the assembly

#### Scenario: Same-call progress remains recoverable

- **WHEN** one unresolved slot produces a value and another unresolved slot is
  impossible in the same valid call
- **THEN** the impossible decision's assembly contains the produced value

#### Scenario: Prior progress remains recoverable

- **WHEN** an assembly with earlier captured values receives an impossible
  finding for an unresolved slot
- **THEN** the impossible decision's assembly retains every earlier value

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
before capture and SHALL return both the original assembly unchanged and the
complete original finding batch on error. A caller SHALL be able to consume
each recovered located finding into its slot and owned finding without cloning.

#### Scenario: An out-of-range slot is rejected

- **WHEN** a finding addresses a slot outside the assembly
- **THEN** adjudication returns an out-of-range structural error with the
  original assembly and complete finding batch

#### Scenario: Same-call duplicate findings are rejected

- **WHEN** one adjudication input contains two findings for the same slot
- **THEN** adjudication returns a duplicate-finding structural error with the
  original assembly and both findings

#### Scenario: An error after valid-looking input captures nothing

- **WHEN** an input contains a valid produced finding followed by any structural
  error
- **THEN** the returned original assembly has not captured the produced value
  and the returned batch still owns every supplied finding

#### Scenario: Recovery requires no policy traits

- **WHEN** values and causes implement neither `Clone` nor serialization traits
- **THEN** a caller can still recover the unchanged assembly and complete batch

#### Scenario: Recovered findings return owned payloads

- **WHEN** a caller consumes a recovered located finding
- **THEN** it receives the stable slot and owned produced value or impossible
  cause

### Requirement: Structural errors have domain-neutral presentation

Structural errors SHALL provide human-readable formatting that identifies the
invalid structure without formatting caller values or causes. Structural
errors SHALL integrate with the core error trait, and adjudication errors SHALL
integrate when their generic payloads satisfy the trait's debug requirement.
Normal adjudication SHALL NOT require values or causes to implement formatting
or error traits.

#### Scenario: An out-of-range error is displayed

- **WHEN** a caller formats an out-of-range structural or adjudication error
- **THEN** the message identifies the addressed slot and fixed slot count
  without including caller payloads

#### Scenario: A duplicate error is displayed

- **WHEN** a caller formats a duplicate-finding structural or adjudication error
- **THEN** the message identifies the duplicated slot without including caller
  payloads

#### Scenario: Error integration is optional for payloads

- **WHEN** values and causes satisfy the debug requirement
- **THEN** the adjudication error can be used through the core error trait

#### Scenario: Ordinary use retains minimal bounds

- **WHEN** values and causes implement no formatting or error traits
- **THEN** callers can still construct, adjudicate, and recover them by
  ownership

### Requirement: Adjudication entrypoints are equivalent

The core SHALL provide free-function, assembly method, and single-finding
method entrypoints. Equivalent input SHALL produce the same decision or
structural error, and all entrypoints SHALL retain ownership-first semantics.

#### Scenario: Batch method matches the free function

- **WHEN** identical assemblies and batches are passed to the free function and
  batch method
- **THEN** both entrypoints return equivalent outcomes

#### Scenario: Single-finding method captures progress

- **WHEN** an unresolved slot receives one produced finding through the
  single-finding method
- **THEN** its value is captured under the same rules as batch adjudication

#### Scenario: Single-finding method reports invalid input

- **WHEN** one out-of-range finding is passed to the single-finding method
- **THEN** it returns the unchanged assembly and that complete one-item batch

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
