# Project Contract — Lengkap

## Status

**Tier 1 — consumer-proven.** Version 0.1.0 defines the initial public release.
Worklane consumes that registry artifact while keeping broker access,
checkpoint persistence, polling, and reactions in its own adapter. Adoption
required no Worklane vocabulary, I/O, or policy in Lengkap.

This status records completed evidence, not an invitation to widen the core.
More abstraction remains no substitute for concrete product pressure.

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
  finding batch; each located finding can be consumed back into its slot and
  owned finding.
- **Domain-neutral errors.** Structural failures provide useful formatting
  without inspecting or imposing formatting bounds on user values and causes.
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

## Graduation Evidence

The first bridge consumer is Worklane. Its fan-in adapter maps:

- live work to no finding
- completed-or-unknown work with stored result bytes to `Produced`
- completed-or-unknown work without result bytes to
  `Impossible(MissingResult)`
- dead-lettered work to `Impossible(DeadLettered)`

Tier 1 required both:

1. `lengkap-contract` and `lengkap` are available through a separately reviewed
   public release; and
2. a real consumer adopts that registry artifact without forcing domain
   vocabulary or I/O into the contract.

Both conditions are complete. Worklane declares `lengkap = "0.1.0"`, resolves
the facade and contract from crates.io with registry checksums, and owns the
mapping and checkpoint adapters around the pure decision. No core API or user
obligation changed during adoption.

Future consumer pressure may still reveal a structural mismatch. Revise the
contract only from that evidence in a compatible release; do not grow the core
in anticipation.

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
2. Independent product usability under those explicit obligations.
3. Fit learned from a real bridge consumer.
4. Documentation and ergonomics around the existing contract.
5. Graduation and release, only after their explicit conditions hold.

Potential features without a current consumer belong in `BACKLOG.md`.
