# Project Contract — Lengkap

## Status

**Tier 2 — release candidate, unpublished.** The contract is clean, stable
enough for a bridge trial, and protected by executable governance. It is not
Tier 1 until a real consumer adopts it and the contract and facade are actually
published through a separately authorized release.

An ungraduated Lengkap is a legitimate permanent result. More abstraction is
not a substitute for adoption evidence.

## Purpose

Lengkap is a pure all-of evidence completion mechanism. A caller declares a
fixed number of ordered required slots, supplies findings over time, and asks
the core to adjudicate:

- `Pending` while any slot is unresolved and none is known impossible;
- `Ready` with values in slot order when every slot is captured; or
- `Impossible` for the lowest unresolved impossible slot, with the accumulated
  assembly returned for recovery.

The core owns completion mechanics and nothing else. Users own what a slot
means, whether evidence is true, how pending state is stored, when to poll, and
what effect follows a decision.

## Core Contract

- **Fixed all-of set.** Slot cardinality and order are established at
  `Assembly` construction and never change during adjudication.
- **Monotonic first-value capture.** Once a slot contains a value, later
  findings cannot replace or revoke it.
- **Order-independent decision.** Ready output is in slot order and the lowest
  unresolved impossible slot wins, regardless of finding arrival order.
- **Atomic structural validation.** Out-of-range and same-call duplicate
  findings return the original assembly unchanged with the complete rejected
  finding batch.
- **Caller-owned checkpoint seam.** Ordered optional slots can leave and
  restore an assembly without prescribing serialization or persistence.
- **Inspectable progress.** Captured and remaining counts plus stable
  unresolved-slot iteration expose progress without consuming state.
- **Absence is pending.** A caller supplies no synthetic "still live" finding.
- **Empty all-of identity.** An assembly with zero slots is ready with no
  values; domains may reject empty groups at their own boundary.
- **Minimal type obligations.** User values and causes need not implement
  `Clone`, `Eq`, `Hash`, `Error`, or serialization traits.
- **Sans-I/O boundary.** The contract is unconditional `no_std + alloc`, has no
  dependencies, reads no ambient clock, exposes no public async API, performs
  no standard-library I/O, and acquires no serde marker.

## User Obligations

Lengkap deliberately cannot decide:

- which domain entities correspond to slots
- how many slots a domain operation requires
- whether an observation is authoritative or stale
- whether contradictory evidence should be audited or rejected
- how exported assembly slots are encoded and persisted between calls
- how evidence is fetched, normalized, scheduled, or retried
- what to do with `Pending`, `Ready`, `Impossible`, or a structural error

The caller must uphold those obligations. The core's purity makes the boundary
visible; it does not make domain truth automatic.

## Terminology

- **Slot**: a stable zero-based position in the required all-of set.
- **Assembly**: fixed ordered storage for values captured so far.
- **Finding**: caller-supplied evidence that a slot produced a value or became
  impossible.
- **Located finding**: one finding associated with one slot.
- **Decision**: `Pending`, `Ready`, or `Impossible`.
- **Structural error**: malformed adjudication input, distinct from a valid
  domain-level impossible result.

## Non-Goals

Lengkap is not:

- a queue, broker, workflow engine, scheduler, or callback system
- an evidence discovery or verification system
- a persistence, transport, wire-format, or serialization abstraction
- a quorum, any-of, weighted, or dynamically growing completion engine
- a conflict-resolution system for contradictory observations
- an exactly-once or distributed-consensus claim
- a Worklane-specific helper disguised with generic names

## Graduation

The first intended bridge consumer is Worklane. Its fan-in adapter can map:

- live work to no finding
- completed-or-unknown work with stored result bytes to `Produced`
- completed-or-unknown work without result bytes to
  `Impossible(MissingResult)`
- dead-lettered work to `Impossible(DeadLettered)`

That mapping is evidence, not automatic adoption. Graduation requires both:

1. Worklane or another real consumer adopts Lengkap without forcing domain
   vocabulary or I/O into the contract; and
2. `lengkap-contract` and `lengkap` are published through a separately
   reviewed release.

If the bridge reveals a structural mismatch, revise the contract from that
evidence before publication. Do not grow the core in anticipation.

## Architecture

The workspace contains one product mechanism and two support surfaces:

```text
lengkap ───────> lengkap-contract

lengkap-governance ───────> tianheng
```

`lengkap` is a complete glob re-export, not a second product. The governor is
independent from the graph it judges. The generated Tianheng projection is in
`docs/architecture/tianheng-law.md`.

## Change Prioritization

1. Completion correctness and preservation of user obligations.
2. Fit learned from a real bridge consumer.
3. Documentation and ergonomics around the existing contract.
4. Graduation and release, only after their explicit conditions hold.

Potential features without a current consumer belong in `BACKLOG.md`.
